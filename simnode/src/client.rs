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

//! Utilities for creating the neccessary client subsystems.

use crate::{
	ChainInfo, ParachainSproofInherentProvider, SignatureVerificationOverride, SimnodeApiServer,
	SimnodeRpcHandler,
};
use futures::{channel::mpsc, future::Either, lock::Mutex, StreamExt};
use jsonrpsee::RpcModule;
use manual_seal::{
	consensus::timestamp::SlotTimestampProvider,
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, EngineCommand, ManualSealParams,
};
use num_traits::AsPrimitive;
use sc_client_api::backend::BlockImportOperation as IBlockImportOperation;
use sc_client_db::BlockImportOperation;
use sc_consensus::{BlockImport, ImportQueue};
use sc_executor::WasmExecutor;
use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_service::{
	build_network, spawn_tasks, BuildNetworkParams, Configuration, PartialComponents,
	SpawnTasksParams, TFullBackend, TFullClient, TaskManager,
};
use sc_telemetry::Telemetry;
use sc_transaction_pool::FullPool;
use sc_transaction_pool_api::TransactionPool;
use simnode_runtime_api::CreateTransactionApi;
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
use sp_wasm_interface::ExtendedHostFunctions;
use std::sync::Arc;

/// Shared instance of [`ParachainSproofInherentProvider`]
pub type SharedParachainInherentProvider<T> = Arc<Mutex<ParachainSproofInherentProvider<T>>>;

/// The simnode executor type, we use the wasm executor to force the runtime use host functions
/// instead of native code for signature verification, this in turn uses our signature verification
/// overrides.
pub type Executor = WasmExecutor<
	ExtendedHostFunctions<sp_io::SubstrateHostFunctions, SignatureVerificationOverride>,
>;

/// Type alias for [`sc_service::TFullClient`]
pub type FullClientFor<C> =
	TFullClient<<C as ChainInfo>::Block, <C as ChainInfo>::RuntimeApi, Executor>;

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

/// Arguments to pass to the `create_rpc_io_handler`
pub struct RpcHandlerArgs<C: ChainInfo>
where
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block> + TaggedTransactionQueue<C::Block>,
{
	/// Client
	pub client: Arc<FullClientFor<C>>,
	/// Client Backend
	pub backend: Arc<FullBackendFor<C>>,
	/// Transaction pool
	pub pool: Arc<FullPool<C::Block, FullClientFor<C>>>,
	/// Select chain implementation
	/// Signifies whether a potentially unsafe RPC should be denied.
	pub deny_unsafe: sc_rpc::DenyUnsafe,
	/// Subscription task executor
	pub subscription_executor: sc_rpc::SubscriptionTaskExecutor,
}

/// Params needed to initialize simnode's subsystems.
pub struct SimnodeParams<Client, Backend, SelectChain, Pool, ImportQueue, BlockImport, U> {
	/// The necessary subsystems to run simnode.
	pub components: PartialComponents<
		Client,
		Backend,
		SelectChain,
		ImportQueue,
		Pool,
		(BlockImport, Option<Telemetry>, U),
	>,
	/// Config needed for simnode's own subsystems.
	pub config: Configuration,
	/// Use instant sealing for block production? if not uses manual seal.
	pub instant: bool,
	/// rpc builder.
	pub rpc_builder: Box<
		dyn Fn(DenyUnsafe, SubscriptionTaskExecutor) -> Result<RpcModule<()>, sc_service::Error>,
	>,
}

/// Set up and run simnode for a parachain runtime.
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
	BI: BlockImport<
			B,
			Error = sp_consensus::Error,
			Transaction = PrefixedMemoryDB<<B::Header as Header>::Hashing>,
		> + Send
		+ Sync
		+ 'static,
	S: SelectChain<B> + 'static,
	<C::RuntimeApi as ConstructRuntimeApi<B, FullClientFor<C>>>::RuntimeApi:
		Core<B>
			+ TaggedTransactionQueue<B>
			+ sp_offchain::OffchainWorkerApi<B>
			+ sp_api::Metadata<B>
			+ sp_session::SessionKeys<B>
			+ ApiExt<B, StateBackend = <BlockImportOperation<B> as IBlockImportOperation<B>>::State>
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
		Arc::new(Mutex::new(ParachainSproofInherentProvider::new(client.clone())));

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
		let pool = pool.clone();
		let params = SpawnTasksParams {
			config,
			client: client.clone(),
			backend: backend.clone(),
			task_manager: &mut task_manager,
			keystore: keystore_container.sync_keystore(),
			transaction_pool: pool.clone(),
			rpc_builder: Box::new(move |deny_unsafe, subscription_executor| {
				let mut io = rpc_builder(deny_unsafe, subscription_executor)?;

				io.merge(
					SimnodeRpcHandler::<C>::new(
						client.clone(),
						backend.clone(),
						(parachain_inherent_provider_clone.clone(), rpc_sink.clone()),
					)
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
