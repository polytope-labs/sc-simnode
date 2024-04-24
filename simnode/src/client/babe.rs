// Copyright (C) 2023 Polytope Labs (Caymans) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Simnode for Standalone runtimes with Babe Consensus

use futures::{channel::mpsc, future::Either, FutureExt, StreamExt};
use manual_seal::{
	consensus::babe::BabeConsensusDataProvider,
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, EngineCommand, ManualSealParams,
};
use num_traits::AsPrimitive;
use sc_client_api::Backend;
use sc_consensus::{BlockImport, ImportQueue};
use sc_consensus_babe::BabeLink;
use sc_service::{
	build_network, spawn_tasks, BuildNetworkParams, PartialComponents, SpawnTasksParams,
	TFullBackend, TFullClient, TaskManager,
};
use sc_transaction_pool::FullPool;
use sc_transaction_pool_api::{OffchainTransactionPoolFactory, TransactionPool};
use sp_api::{ApiExt, ConstructRuntimeApi, Core};
use sp_block_builder::BlockBuilder;
use sp_consensus::SelectChain;
use sp_consensus_babe::AuthorityId;
use sp_core::crypto::AccountId32;
use sp_keyring::Sr25519Keyring::Alice;
use sp_runtime::traits::{Block as BlockT, Header};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;

use simnode_runtime_api::CreateTransactionApi;

use crate::{timestamp::SlotTimestampProvider, ChainInfo, SimnodeApiServer, SimnodeRpcHandler};

use super::*;

/// Set up and run simnode
pub async fn start_simnode<C, B, S, I, BI>(
	params: SimnodeParams<
		TFullClient<C::Block, C::RuntimeApi, Executor>,
		TFullBackend<B>,
		S,
		FullPool<B, FullClientFor<C>>,
		I,
		BI,
		BabeLink<B>,
	>,
) -> Result<TaskManager, sc_service::Error>
where
	B: BlockT,
	C: ChainInfo<Block = B> + 'static + Send + Sync,
	I: ImportQueue<B> + 'static,
	BI: BlockImport<B, Error = sp_consensus::Error> + Send + Sync + 'static,
	S: SelectChain<B> + 'static,
	<C::RuntimeApi as ConstructRuntimeApi<B, FullClientFor<C>>>::RuntimeApi:
		Core<B>
			+ TaggedTransactionQueue<B>
			+ sp_offchain::OffchainWorkerApi<B>
			+ sp_api::Metadata<B>
			+ sp_session::SessionKeys<B>
			+ ApiExt<B>
			+ BlockBuilder<B>
			+ sp_consensus_babe::BabeApi<B>
			+ CreateTransactionApi<
				C::Block,
				<C::Runtime as frame_system::Config>::RuntimeCall,
				<C::Runtime as frame_system::Config>::AccountId,
			>,
	<<B as BlockT>::Header as Header>::Number: AsPrimitive<u32>,
	<B as BlockT>::Hash: Unpin,
	<B as BlockT>::Header: Unpin,
	<C::Runtime as frame_system::Config>::RuntimeCall: Send + Sync,
	<C::Runtime as frame_system::Config>::AccountId: Send + Sync + From<AccountId32>,
{
	let SimnodeParams { components, config, instant, rpc_builder } = params;
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool: pool,
		other: (block_import, mut telemetry, babe_link),
	} = components;

	let net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);
	let (network, system_rpc_tx, tx_handler_controller, _network_starter, sync_service) = {
		let params = BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
			block_relay: None,
		};
		build_network(params)?
	};

	// offchain workers
	if config.offchain_worker.enabled {
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-worker",
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				is_validator: config.role.is_authority(),
				keystore: Some(keystore_container.keystore()),
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(OffchainTransactionPoolFactory::new(pool.clone())),
				network_provider: network.clone(),
				enable_http_requests: true,
				custom_extensions: |_| vec![],
			})
			.run(client.clone(), task_manager.spawn_handle())
			.boxed(),
		);
	}

	// Proposer object for block authorship.
	let env = sc_basic_authorship::ProposerFactory::new(
		task_manager.spawn_handle(),
		client.clone(),
		pool.clone(),
		config.prometheus_registry(),
		None,
	);

	// Channel for the rpc handler to communicate with the authorship task.
	let (command_sink, commands_stream) = mpsc::channel(10);
	let rpc_sink = command_sink.clone();

	let rpc_handlers = {
		let client = client.clone();
		let backend = backend.clone();
		let pool = pool.clone();
		let params = SpawnTasksParams {
			config,
			client: client.clone(),
			backend: backend.clone(),
			task_manager: &mut task_manager,
			keystore: keystore_container.keystore(),
			transaction_pool: pool.clone(),
			rpc_builder: Box::new(move |deny_unsafe, subscription_executor| {
				let mut io = rpc_builder(deny_unsafe, subscription_executor)?;

				io.merge(SimnodeRpcHandler::<C>::new(client.clone(), backend.clone()).into_rpc())
					.map_err(|_| {
						sc_service::Error::Other("Unable to merge simnode rpc api".to_string())
					})?;
				io.merge(ManualSeal::new(rpc_sink.clone()).into_rpc()).map_err(|_| {
					sc_service::Error::Other("Unable to merge manual seal rpc api".to_string())
				})?;
				Ok(io)
			}),
			network,
			system_rpc_tx,
			tx_handler_controller,
			sync_service,
			telemetry: telemetry.as_mut(),
		};
		spawn_tasks(params)?
	};

	_network_starter.start_network();
	let _rpc_handler = rpc_handlers.handle();

	let babe_consensus = BabeConsensusDataProvider::new(
		client.clone(),
		keystore_container.keystore(),
		babe_link.epoch_changes().clone(),
		vec![(AuthorityId::from(Alice.public()), 1000)],
	)
	.map_err(|e| sc_service::Error::Application(Box::new(e)))?;

	run_manual_seal(ManualSealParams {
		block_import,
		env,
		client: client.clone(),
		pool: pool.clone(),
		commands_stream: if instant {
			let tx_notifications =
				pool.import_notification_stream().map(move |_| EngineCommand::SealNewBlock {
					create_empty: true,
					// parachains need their blocks finalized instantly to be part of the main
					// chain.
					finalize: true,
					parent_hash: None,
					sender: None,
				});

			Either::Left(futures::stream::select(tx_notifications, commands_stream))
		} else {
			Either::Right(commands_stream)
		},
		select_chain,
		consensus_data_provider: Some(Box::new(babe_consensus)),
		create_inherent_data_providers: {
			let client = client.clone();
			move |parent, _| {
				let client = client.clone();
				async move {
					let client = client.clone();

					let timestamp = SlotTimestampProvider::new_babe(client.clone(), parent)
						.map_err(|err| format!("{:?}", err))?;

					let babe = sp_consensus_babe::inherents::InherentDataProvider::new(
						timestamp.slot().into(),
					);

					Ok((timestamp, babe))
				}
			}
		},
	})
	.await;

	Ok(task_manager)
}
