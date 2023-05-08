// Copyright (C) 2021 Polytope Capital (Caymans) Ltd.
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

//! Utilities for creating the neccessary client subsystems.

use crate::{
	ChainInfo, FullBackendFor, FullClientFor, NativeElseWasmExecutor, Node,
	ParachainInherentSproofProvider,
};
use futures::channel::mpsc;
use manual_seal::consensus::aura::AuraConsensusDataProvider;
use manual_seal::consensus::timestamp::SlotTimestampProvider;
use manual_seal::{
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, ManualSealParams,
};
use num_traits::AsPrimitive;
use sc_client_api::{
	backend::BlockImportOperation as IBlockImportOperation, BlockBackend, BlockchainEvents,
	ProofProvider,
};
use sc_client_db::BlockImportOperation;
use sc_consensus::{BlockImport, ImportQueue};
use sc_service::TFullClient;
use sc_service::{
	build_network, spawn_tasks, BuildNetworkParams, Configuration, PartialComponents,
	SpawnTasksParams, TFullBackend,
};
use sc_telemetry::Telemetry;
use sc_transaction_pool::FullPool;
use sp_api::{ApiExt, ConstructRuntimeApi, Core, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{HeaderBackend, HeaderMetadata};
use sp_consensus::block_validation::Chain;
use sp_consensus::SelectChain;
use sp_runtime::traits::{Block as BlockT, BlockIdTo, Header};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use sp_trie::PrefixedMemoryDB;
use std::sync::{Arc, Mutex};

/// Arguments to pass to the `create_rpc_io_handler`
pub struct RpcHandlerArgs<C: ChainInfo, SC>
where
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block> + TaggedTransactionQueue<C::Block>,
{
	/// Client
	pub client: Arc<FullClientFor<C>>,
	/// Client Backend
	pub backend: Arc<FullBackendFor<C>>,
	/// Transaction pool
	pub pool: Arc<sc_transaction_pool::FullPool<C::Block, FullClientFor<C>>>,
	/// Select chain implementation
	pub select_chain: SC,
	/// Signifies whether a potentially unsafe RPC should be denied.
	pub deny_unsafe: sc_rpc::DenyUnsafe,
	/// Subscription task executor
	pub subscription_executor: sc_rpc::SubscriptionTaskExecutor,
}

/// Simnode run
pub fn simnode<T: ChainInfo, C, B, S, I, P, BI, U>(
	components: PartialComponents<
		TFullClient<T::Block, T::RuntimeApi, NativeElseWasmExecutor<T::ExecutorDispatch>>,
		TFullBackend<B>,
		S,
		I,
		FullPool<B, FullClientFor<T>>,
		(BI, Option<&mut Telemetry>, U),
	>,
	config: Configuration,
	is_parachain: bool,
) -> Result<Node<T>, sc_service::Error>
where
	B: BlockT,
	B: BlockT,
	C: ProvideRuntimeApi<B>
		+ HeaderMetadata<B, Error = sp_blockchain::Error>
		+ Chain<B>
		+ ChainInfo
		+ BlockBackend<B>
		+ BlockIdTo<B, Error = sp_blockchain::Error>
		+ ProofProvider<B>
		+ HeaderBackend<B>
		+ BlockchainEvents<B>
		+ 'static
		+ Send
		+ Sync,
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block> + TaggedTransactionQueue<C::Block>,
	<C as ProvideRuntimeApi<B>>::Api: sp_offchain::OffchainWorkerApi<B>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<B>,
	I: ImportQueue<B> + 'static + sc_service::ImportQueue<<T as ChainInfo>::Block>,
	BI: BlockImport<B>
		+ BlockImport<
			B,
			Error = sp_consensus::Error,
			Transaction = PrefixedMemoryDB<<<B as BlockT>::Header as Header>::Hashing>,
		> + Send
		+ Sync
		+ 'static,
	S: Clone + SelectChain<B> + 'static,
	T: ChainInfo<Block = B> + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<B, FullClientFor<T>>>::RuntimeApi:
		Core<B>
			+ TaggedTransactionQueue<B>
			+ sp_offchain::OffchainWorkerApi<B>
			+ sp_api::Metadata<B>
			+ sp_session::SessionKeys<B>
			+ ApiExt<B, StateBackend = <BlockImportOperation<B> as IBlockImportOperation<B>>::State>
			+ BlockBuilder<B>
			+ sp_consensus_aura::AuraApi<B, sp_consensus_aura::sr25519::AuthorityId>,
	<<B as BlockT>::Header as Header>::Number: AsPrimitive<u32>,
	<B as BlockT>::Hash: Unpin,
	<B as BlockT>::Header: Unpin,
{
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool: pool,
		other: (block_import, telemetry, _),
	} = components;
	let parachain_inherent_provider = if is_parachain {
		Some(Arc::new(Mutex::new(ParachainInherentSproofProvider::new(client.clone()))))
	} else {
		None
	};

	let (network, system_rpc_tx, tx_handler_controller, _network_starter, sync_service) = {
		let params = BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
		};
		build_network(params)?
	};

	// offchain workers
	sc_service::build_offchain_workers(
		&config,
		task_manager.spawn_handle(),
		client.clone(),
		network.clone(),
	);

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
		let select_chain = select_chain.clone();
		let pool = pool.clone();
		let params = SpawnTasksParams {
			config,
			client: client.clone(),
			backend: backend.clone(),
			task_manager: &mut task_manager,
			keystore: keystore_container.sync_keystore(),
			transaction_pool: pool.clone(),
			rpc_builder: Box::new(move |deny_unsafe, subscription_executor| {
				let mut io = <T as ChainInfo>::create_rpc_io_handler(RpcHandlerArgs {
					client: client.clone(),
					backend: backend.clone(),
					pool: pool.clone(),
					select_chain: select_chain.clone(),
					deny_unsafe,
					subscription_executor,
				});
				io.merge(ManualSeal::new(rpc_sink.clone()).into_rpc()).map_err(|_| {
					sc_service::Error::Other("Unable to merge manual seal rpc api".to_string())
				})?;
				Ok(io)
			}),
			network,
			system_rpc_tx,
			tx_handler_controller,
			sync_service,
			telemetry,
		};
		spawn_tasks(params)?
	};

	// Background authorship future.
	let authorship_future = run_manual_seal(ManualSealParams {
		block_import,
		env,
		client: client.clone(),
		pool: pool.clone(),
		commands_stream,
		select_chain,
		consensus_data_provider: Some(Box::new(AuraConsensusDataProvider::new(client.clone()))),
		create_inherent_data_providers: {
			let client = client.clone();
			let parachain_inherent_provider = parachain_inherent_provider.clone();
			move |_, _| {
				let client = client.clone();
				let parachain_sproof = parachain_inherent_provider.clone().unwrap();
				async move {
					let client = client.clone();
					let parachain_sproof = parachain_sproof.clone();

					let timestamp = SlotTimestampProvider::new_aura(client.clone())
						.map_err(|err| format!("{:?}", err))?;

					let _aura = sp_consensus_aura::inherents::InherentDataProvider::new(
						timestamp.slot().into(),
					);

					let parachain_system =
						parachain_sproof.lock().unwrap().create_inherent(timestamp.slot().into());
					Ok((timestamp, _aura, parachain_system))
				}
			}
		},
	});

	// spawn the authorship task as an essential task.
	task_manager
		.spawn_essential_handle()
		.spawn("manual-seal", None, authorship_future);

	_network_starter.start_network();
	let rpc_handler = rpc_handlers.handle();

	let node = Node::<T> {
		rpc_handler,
		task_manager: Some(task_manager),
		pool,
		backend,
		initial_block_number: client.clone().info().best_number,
		client: client.clone(),
		manual_seal_command_sink: command_sink,
		parachain_inherent_provider: parachain_inherent_provider.clone(),
	};

	Ok(node)
}
