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

//! Simnode for Standalone runtimes with Parachain Consensus

use super::*;
use crate::{ChainInfo, ParachainSproofInherentProvider, SimnodeApiServer, SimnodeRpcHandler};
use async_trait::async_trait;
use futures::{channel::mpsc, future::Either, lock::Mutex, FutureExt, StreamExt};
use jsonrpsee::core::{Error as RpcError, RpcResult};
use manual_seal::{
	consensus::timestamp::SlotTimestampProvider,
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, EngineCommand, ManualSealParams,
};
use num_traits::AsPrimitive;
use sc_client_api::Backend;

use sc_consensus::{BlockImport, ImportQueue};
use sc_service::{
	build_network, spawn_tasks, BuildNetworkParams, PartialComponents, SpawnTasksParams,
	TFullBackend, TFullClient, TaskManager,
};
use sc_transaction_pool::FullPool;
use sc_transaction_pool_api::{OffchainTransactionPoolFactory, TransactionPool};
use simnode_runtime_api::CreateTransactionApi;
use sp_api::{ApiExt, ConstructRuntimeApi, Core};
use sp_block_builder::BlockBuilder;
use sp_blockchain::HeaderBackend;
use sp_consensus::SelectChain;
use sp_core::{crypto::AccountId32, Bytes};
use sp_runtime::traits::{Block as BlockT, Header};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;

use std::sync::Arc;

/// Parachain handler implementation for Simnode RPC API.
pub struct ParachainRPCHandler<T: ChainInfo> {
	/// Holds the inner rpc handler for delegating requests
	inner: SimnodeRpcHandler<T>,
	/// Sink for sending commands to the manual seal authorship task.
	sink: futures::channel::mpsc::Sender<manual_seal::EngineCommand<<T::Block as BlockT>::Hash>>,
	/// parachain inherent provider for sproofing the parachain inherent.
	parachain: crate::sproof::SharedParachainSproofInherentProvider<T>,
}

#[async_trait]
impl<T> SimnodeApiServer for ParachainRPCHandler<T>
where
	T: ChainInfo + Send + Sync + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as frame_system::Config>::RuntimeCall,
			<T::Runtime as frame_system::Config>::AccountId,
		>,
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
	<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
	T::Runtime: parachain_info::Config,
{
	fn author_extrinsic(&self, call: Bytes, account: String) -> RpcResult<Bytes> {
		Ok(self.inner.author_extrinsic(call, account)?.into())
	}

	fn revert_blocks(&self, n: u32) -> RpcResult<()> {
		self.inner.revert_blocks(n)
	}

	async fn upgrade_signal(&self, go_ahead: bool) -> RpcResult<()> {
		use futures::{channel::oneshot, SinkExt};

		let signal = match go_ahead {
			true => polkadot_primitives::UpgradeGoAhead::GoAhead,
			false => polkadot_primitives::UpgradeGoAhead::Abort,
		};

		// insert the upgrade signal into the sproof provider, it'll be included in the next block.
		let para_id = self
			.inner
			.with_state(None, || parachain_info::Pallet::<T::Runtime>::parachain_id());
		let builder = sproof_builder::RelayStateSproofBuilder {
			para_id,
			upgrade_go_ahead: Some(signal),
			..Default::default()
		};
		self.parachain.lock().await.update_sproof_builder(builder);

		let mut sink = self.sink.clone();
		let (sender, receiver) = oneshot::channel();
		// NOTE: this sends a Result over the channel.
		let command = manual_seal::EngineCommand::SealNewBlock {
			create_empty: true,
			finalize: true,
			parent_hash: None,
			sender: Some(sender),
		};

		sink.send(command).await?;

		match receiver.await {
			Ok(Ok(_)) => Ok(()),
			Ok(Err(e)) => Err(e.into()),
			Err(e) => Err(RpcError::to_call_error(e)),
		}
	}
}

/// Parachain [`SelectChain`] implementation
/// Since parachains don't have a select chain implementation, use this one instead.
pub struct ParachainSelectChain<Client> {
	client: Arc<Client>,
}

impl<C> Clone for ParachainSelectChain<C> {
	fn clone(&self) -> Self {
		Self { client: self.client.clone() }
	}
}

impl<C> ParachainSelectChain<C> {
	/// Initialize the parachain [`SelectChain`]
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

#[async_trait]
impl<B, C> SelectChain<B> for ParachainSelectChain<C>
where
	B: BlockT,
	C: HeaderBackend<B>,
{
	async fn leaves(&self) -> Result<Vec<B::Hash>, sp_consensus::Error> {
		Ok(vec![])
	}

	async fn best_chain(&self) -> Result<B::Header, sp_consensus::Error> {
		let header = self
			.client
			.header(self.client.info().best_hash)
			.map_err(|e| sp_consensus::Error::Other(Box::new(e)))?
			.ok_or_else(|| sp_consensus::Error::StateUnavailable(format!("Header not found!")))?;
		Ok(header)
	}
}

/// Set up and run simnode
pub async fn start_simnode<C, B, S, I, BI, U>(
	params: SimnodeParams<
		TFullClient<C::Block, C::RuntimeApi, Executor>,
		TFullBackend<B>,
		S,
		FullPool<B, FullClientFor<C>>,
		I,
		BI,
		U,
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
			+ sp_consensus_aura::AuraApi<B, sp_consensus_aura::sr25519::AuthorityId>
			+ CreateTransactionApi<
				C::Block,
				<C::Runtime as frame_system::Config>::RuntimeCall,
				<C::Runtime as frame_system::Config>::AccountId,
			>,
	<<B as BlockT>::Header as Header>::Number: AsPrimitive<u32>,
	<B as BlockT>::Hash: Unpin,
	<B as BlockT>::Header: Unpin,
	C::Runtime: parachain_info::Config,
	<C::Runtime as frame_system::Config>::RuntimeCall: Send + Sync,
	<C::Runtime as frame_system::Config>::AccountId: Send + Sync + From<AccountId32>,
{
	use manual_seal::consensus::aura::AuraConsensusDataProvider;

	let SimnodeParams { components, config, instant, rpc_builder } = params;
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool: pool,
		other: (block_import, mut telemetry, _),
	} = components;

	let parachain_inherent_provider =
		Arc::new(Mutex::new(ParachainSproofInherentProvider::<C>::new(client.clone())));

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

	let parachain_inherent_provider_clone = parachain_inherent_provider.clone();
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

				io.merge(
					ParachainRPCHandler {
						inner: SimnodeRpcHandler::new(client.clone(), backend.clone()),
						sink: rpc_sink.clone(),
						parachain: parachain_inherent_provider_clone.clone(),
					}
					.into_rpc(),
				)
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
		consensus_data_provider: Some(Box::new(AuraConsensusDataProvider::new(client.clone()))),
		create_inherent_data_providers: {
			let client = client.clone();
			let parachain_inherent_provider = parachain_inherent_provider.clone();
			move |_, _| {
				let client = client.clone();
				let parachain_sproof = parachain_inherent_provider.clone();
				async move {
					let client = client.clone();
					let parachain_sproof = parachain_sproof.clone();

					let timestamp = SlotTimestampProvider::new_aura(client.clone())
						.map_err(|err| format!("{:?}", err))?;

					let aura = sp_consensus_aura::inherents::InherentDataProvider::new(
						timestamp.slot().into(),
					);

					let parachain_system =
						parachain_sproof.lock().await.create_inherent(timestamp.slot().into())?;
					Ok((timestamp, aura, parachain_system))
				}
			}
		},
	})
	.await;

	Ok(task_manager)
}
