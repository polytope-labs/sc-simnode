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
<<<<<<< Updated upstream
	ChainInfo, FullBackendFor, FullClientFor, NativeElseWasmExecutor, Node,
	ParachainInherentSproofProvider,
=======
	rpc::{SimnodeApiServer, SimnodeRpcHandler},
	sproof::ParachainSproofInherentProvider,
	ChainInfo, NativeElseWasmExecutor,
>>>>>>> Stashed changes
};
use futures::channel::mpsc;
use manual_seal::{
	consensus::{aura::AuraConsensusDataProvider, timestamp::SlotTimestampProvider},
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, ManualSealParams,
};
use num_traits::AsPrimitive;
use sc_client_api::backend::BlockImportOperation as IBlockImportOperation;
use sc_client_db::BlockImportOperation;
use sc_consensus::{BlockImport, ImportQueue};
use sc_service::{build_network, spawn_tasks, BuildNetworkParams, Configuration, PartialComponents, SpawnTasksParams, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::Telemetry;
use sc_transaction_pool::FullPool;
use sp_api::{ApiExt, ConstructRuntimeApi, Core};
use sp_block_builder::BlockBuilder;
use sp_consensus::SelectChain;
use sp_core::crypto::AccountId32;
use sp_runtime::{
	generic::UncheckedExtrinsic,
	traits::{Block as BlockT, Header},
	MultiAddress, MultiSignature,
};
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use sp_trie::PrefixedMemoryDB;
use std::sync::{Arc, Mutex};
<<<<<<< Updated upstream
=======

/// Shared instance of [`ParachainSproofInherentProvider`]
pub type SharedParachainInherentProvider<T> = Arc<Mutex<ParachainSproofInherentProvider<T>>>;

/// Type alias for [`sc_service::TFullClient`]
pub type FullClientFor<C> = TFullClient<
	<C as ChainInfo>::Block,
	<C as ChainInfo>::RuntimeApi,
	NativeElseWasmExecutor<<C as ChainInfo>::ExecutorDispatch>,
>;

/// UncheckedExtrinsic type for Simnode
pub type UncheckedExtrinsicFor<T> = UncheckedExtrinsic<
	MultiAddress<
		<<T as ChainInfo>::Runtime as frame_system::Config>::AccountId,
		<<T as ChainInfo>::Runtime as frame_system::Config>::Index,
	>,
	<<T as ChainInfo>::Runtime as frame_system::Config>::RuntimeCall,
	MultiSignature,
	<T as ChainInfo>::SignedExtras,
>;

/// Type alias for [`sc_service::TFullBackend`]
pub type FullBackendFor<C> = TFullBackend<<C as ChainInfo>::Block>;
>>>>>>> Stashed changes

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

/// Set up and run simnode for a standalone or parachain runtime.
<<<<<<< Updated upstream
pub fn start_simnode<T, C, B, S, I, BI, U>(
=======
pub async fn start_simnode<C, B, S, I, BI, U>(
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
) -> Result<Node<T>, sc_service::Error>
=======
) -> Result<TaskManager, sc_service::Error>
>>>>>>> Stashed changes
where
	B: BlockT,
	C: ChainInfo<Block = B> + 'static + Send + Sync,
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block> + TaggedTransactionQueue<C::Block>,
	I: ImportQueue<B> + 'static,
	BI: BlockImport<
			B,
			Error = sp_consensus::Error,
			Transaction = PrefixedMemoryDB<<B::Header as Header>::Hashing>,
		> + Send
		+ Sync
		+ 'static,
<<<<<<< Updated upstream
	S: Clone + SelectChain<B> + 'static,
	T: ChainInfo<Block = B> + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<B, FullClientFor<T>>>::RuntimeApi:
=======
	S: SelectChain<B> + 'static,
	<C::RuntimeApi as ConstructRuntimeApi<B, FullClientFor<C>>>::RuntimeApi:
>>>>>>> Stashed changes
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
		Some(Arc::new(Mutex::new(ParachainSproofInherentProvider::new(client.clone()))))
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

	let parachain_inherent_provider_clone = parachain_inherent_provider.clone();
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
<<<<<<< Updated upstream
=======
				io.merge(
					SimnodeRpcHandler::<C>::new(
						client.clone(),
						parachain_inherent_provider_clone.clone(),
					)
					.into_rpc(),
				)
				.map_err(|_| {
					sc_service::Error::Other("Unable to merge simnode rpc api".to_string())
				})?;
>>>>>>> Stashed changes
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

	_network_starter.start_network();
	let _rpc_handler = rpc_handlers.handle();

	run_manual_seal(ManualSealParams {
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
			// todo:
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
	})
	.await;

<<<<<<< Updated upstream
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
=======
	Ok(task_manager)
>>>>>>> Stashed changes
}
