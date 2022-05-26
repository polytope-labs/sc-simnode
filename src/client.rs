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
	ChainInfo, FullBackendFor, FullClientFor, Node, ParachainInherentSproofProvider,
	SharedParachainInherentProvider, SimnodeCli,
};
use futures::channel::mpsc;
use manual_seal::{
	consensus::{aura::AuraConsensusDataProvider, timestamp::SlotTimestampProvider},
	import_queue,
	rpc::{ManualSeal, ManualSealApiServer},
	run_manual_seal, ConsensusDataProvider, ManualSealParams,
};
use parachain_inherent::ParachainInherentData;
use sc_cli::{build_runtime, SubstrateCli};
use sc_client_api::backend::Backend;
use sc_executor::NativeElseWasmExecutor;
use sc_service::{
	build_network, new_full_parts, spawn_tasks, BuildNetworkParams, Configuration,
	KeystoreContainer, SpawnTasksParams, TFullBackend,
};
use sc_tracing::logging::LoggerBuilder;
use sc_transaction_pool::BasicPool;
use simnode_runtime_apis::CreateTransactionApi;
use sp_api::{ApiExt, ConstructRuntimeApi, Core, Metadata, TransactionFor};
use sp_block_builder::BlockBuilder;
use sp_blockchain::HeaderBackend;
use sp_consensus_aura::{sr25519::AuthorityId, AuraApi};
use sp_inherents::CreateInherentDataProviders;
use sp_offchain::OffchainWorkerApi;
use sp_runtime::traits::{Block as BlockT, Header};
use sp_session::SessionKeys;
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use std::{
	error::Error,
	future::Future,
	str::FromStr,
	sync::{Arc, Mutex},
};

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

/// Creates all the client parts you need for [`Node`](crate::node::Node)
pub fn build_node_subsystems<T, I>(
	config: Configuration,
	is_parachain: bool,
	block_import_provider: I,
) -> Result<Node<T>, sc_service::Error>
where
	T: ChainInfo + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		Core<T::Block>
			+ Metadata<T::Block>
			+ OffchainWorkerApi<T::Block>
			+ SessionKeys<T::Block>
			+ TaggedTransactionQueue<T::Block>
			+ BlockBuilder<T::Block>
			+ ApiExt<T::Block, StateBackend = <TFullBackend<T::Block> as Backend<T::Block>>::State>
			+ CreateTransactionApi<
				T::Block,
				<T::Runtime as frame_system::Config>::AccountId,
				<T::Runtime as frame_system::Config>::Call,
			>,
	<T::Runtime as frame_system::Config>::Call: From<frame_system::Call<T::Runtime>>,
	<<T as ChainInfo>::Block as BlockT>::Hash: FromStr + Unpin,
	<<T as ChainInfo>::Block as BlockT>::Header: Unpin,
	<<<T as ChainInfo>::Block as BlockT>::Header as Header>::Number:
		num_traits::cast::AsPrimitive<usize> + num_traits::cast::AsPrimitive<u32>,
	<<T as ChainInfo>::Runtime as frame_system::Config>::AccountId: codec::Codec,
	<<T as ChainInfo>::Runtime as frame_system::Config>::Call: codec::Codec,
	I: Fn(
		Arc<FullClientFor<T>>,
		sc_consensus::LongestChain<TFullBackend<T::Block>, T::Block>,
		&KeystoreContainer,
		Option<SharedParachainInherentProvider<T>>,
	) -> Result<
		(
			T::BlockImport,
			Option<
				Box<
					dyn ConsensusDataProvider<
						T::Block,
						Transaction = TransactionFor<FullClientFor<T>, T::Block>,
					>,
				>,
			>,
			Box<
				dyn CreateInherentDataProviders<
					T::Block,
					(),
					InherentDataProviders = T::InherentDataProviders,
				>,
			>,
		),
		sc_service::Error,
	>,
{
	let executor = NativeElseWasmExecutor::<T::ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore, mut task_manager) =
		new_full_parts::<T::Block, T::RuntimeApi, _>(&config, None, executor)?;
	let client = Arc::new(client);

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let parachain_inherent_provider = if is_parachain {
		Some(Arc::new(Mutex::new(ParachainInherentSproofProvider::new(client.clone()))))
	} else {
		None
	};
	let (block_import, consensus_data_provider, create_inherent_data_providers) =
		block_import_provider(
			client.clone(),
			select_chain.clone(),
			&keystore,
			parachain_inherent_provider.clone(),
		)?;
	let import_queue =
		import_queue(Box::new(block_import.clone()), &task_manager.spawn_essential_handle(), None);
	let pool = BasicPool::new_full(
		config.transaction_pool.clone(),
		true.into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (network, system_rpc_tx, _network_starter) = {
		let params = BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: None,
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
			keystore: keystore.sync_keystore(),
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
			telemetry: None,
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
		consensus_data_provider,
		create_inherent_data_providers,
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
		initial_block_number: client.info().best_number,
		client,
		manual_seal_command_sink: command_sink,
		parachain_inherent_provider,
	};

	Ok(node)
}

/// Set up and run simnode for a standalone runtime.
pub fn standalone_node<C, B, F, Fut>(build_subsystems: B, callback: F) -> Result<(), Box<dyn Error>>
where
	C: ChainInfo + 'static,
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block>
			+ Metadata<C::Block>
			+ OffchainWorkerApi<C::Block>
			+ SessionKeys<C::Block>
			+ TaggedTransactionQueue<C::Block>
			+ BlockBuilder<C::Block>
			+ ApiExt<C::Block, StateBackend = <TFullBackend<C::Block> as Backend<C::Block>>::State>
			+ CreateTransactionApi<
				C::Block,
				<C::Runtime as frame_system::Config>::AccountId,
				<C::Runtime as frame_system::Config>::Call,
			>,
	<<C as ChainInfo>::Runtime as frame_system::Config>::AccountId: codec::Codec,
	<<C as ChainInfo>::Runtime as frame_system::Config>::Call: codec::Codec,
	<C::Runtime as frame_system::Config>::Call: From<frame_system::Call<C::Runtime>>,
	<<C as ChainInfo>::Block as BlockT>::Hash: FromStr + Unpin,
	<<C as ChainInfo>::Block as BlockT>::Header: Unpin,
	<<<C as ChainInfo>::Block as BlockT>::Header as Header>::Number:
		num_traits::cast::AsPrimitive<usize> + num_traits::cast::AsPrimitive<u32>,
	F: FnOnce(Node<C>) -> Fut,
	Fut: Future<Output = Result<(), Box<dyn Error>>>,
	B: Fn(
		Arc<FullClientFor<C>>,
		sc_consensus::LongestChain<TFullBackend<C::Block>, C::Block>,
		&KeystoreContainer,
		Option<SharedParachainInherentProvider<C>>,
	) -> Result<
		(
			C::BlockImport,
			Option<
				Box<
					dyn ConsensusDataProvider<
						C::Block,
						Transaction = TransactionFor<FullClientFor<C>, C::Block>,
					>,
				>,
			>,
			Box<
				dyn CreateInherentDataProviders<
					C::Block,
					(),
					InherentDataProviders = C::InherentDataProviders,
				>,
			>,
		),
		sc_service::Error,
	>,
	B: Fn(
		Arc<FullClientFor<C>>,
		sc_consensus::LongestChain<TFullBackend<C::Block>, C::Block>,
		&KeystoreContainer,
		Option<SharedParachainInherentProvider<C>>,
	) -> Result<
		(
			C::BlockImport,
			Option<
				Box<
					dyn ConsensusDataProvider<
						C::Block,
						Transaction = TransactionFor<FullClientFor<C>, C::Block>,
					>,
				>,
			>,
			Box<
				dyn CreateInherentDataProviders<
					C::Block,
					(),
					InherentDataProviders = C::InherentDataProviders,
				>,
			>,
		),
		sc_service::Error,
	>,
{
	let tokio_runtime = build_runtime()?;
	// parse cli args
	let cli = <<<C as ChainInfo>::Cli as SimnodeCli>::SubstrateCli as SubstrateCli>::from_args();
	let cli_config = <C as ChainInfo>::Cli::cli_config(&cli);

	// set up logging
	LoggerBuilder::new(<C as ChainInfo>::Cli::log_filters(cli_config)?).init()?;

	// set up the test-runner
	let config = cli.create_configuration(cli_config, tokio_runtime.handle().clone())?;
	sc_cli::print_node_infos::<<<C as ChainInfo>::Cli as SimnodeCli>::SubstrateCli>(&config);

	let node = build_node_subsystems::<C, _>(config, false, build_subsystems)?;

	// hand off node.
	tokio_runtime.block_on(callback(node))?;

	Ok(())
}

/// Set up and run simnode for a parachain runtime.
pub fn parachain_node<C, F, Fut>(callback: F) -> Result<(), Box<dyn Error>>
where
	C: ChainInfo<
			BlockImport = Arc<FullClientFor<C>>,
			InherentDataProviders = (
				SlotTimestampProvider,
				sp_consensus_aura::inherents::InherentDataProvider,
				ParachainInherentData,
			),
		> + 'static,
	<C::RuntimeApi as ConstructRuntimeApi<C::Block, FullClientFor<C>>>::RuntimeApi:
		Core<C::Block>
			+ AuraApi<C::Block, AuthorityId>
			+ Metadata<C::Block>
			+ OffchainWorkerApi<C::Block>
			+ SessionKeys<C::Block>
			+ TaggedTransactionQueue<C::Block>
			+ BlockBuilder<C::Block>
			+ ApiExt<C::Block, StateBackend = <TFullBackend<C::Block> as Backend<C::Block>>::State>
			+ CreateTransactionApi<
				C::Block,
				<C::Runtime as frame_system::Config>::AccountId,
				<C::Runtime as frame_system::Config>::Call,
			>,
	<<C as ChainInfo>::Runtime as frame_system::Config>::AccountId: codec::Codec,
	<<C as ChainInfo>::Runtime as frame_system::Config>::Call: codec::Codec,
	<C::Runtime as frame_system::Config>::Call: From<frame_system::Call<C::Runtime>>,
	<<C as ChainInfo>::Block as BlockT>::Hash: FromStr + Unpin,
	<<C as ChainInfo>::Block as BlockT>::Header: Unpin,
	<<<C as ChainInfo>::Block as BlockT>::Header as Header>::Number:
		num_traits::cast::AsPrimitive<usize> + num_traits::cast::AsPrimitive<u32>,
	F: FnOnce(Node<C>) -> Fut,
	Fut: Future<Output = Result<(), Box<dyn Error>>>,
{
	let tokio_runtime = build_runtime()?;
	// parse cli args
	let cli = <<<C as ChainInfo>::Cli as SimnodeCli>::SubstrateCli as SubstrateCli>::from_args();
	let cli_config = <C as ChainInfo>::Cli::cli_config(&cli);

	// set up logging
	LoggerBuilder::new(<C as ChainInfo>::Cli::log_filters(cli_config)?).init()?;

	// set up the test-runner
	let config = cli.create_configuration(cli_config, tokio_runtime.handle().clone())?;
	sc_cli::print_node_infos::<<<C as ChainInfo>::Cli as SimnodeCli>::SubstrateCli>(&config);

	let node = build_node_subsystems::<C, _>(
		config,
		true,
		|client, _sc, _keystore, parachain_inherent| {
			let cloned_client = client.clone();
			let create_inherent_data_providers = Box::new(move |_, _| {
				let client = cloned_client.clone();
				let parachain_sproof = parachain_inherent.clone().unwrap();
				async move {
					let timestamp = SlotTimestampProvider::new_aura(client.clone())
						.map_err(|err| format!("{:?}", err))?;

					let _aura = sp_consensus_aura::inherents::InherentDataProvider::new(
						timestamp.slot().into(),
					);

					let parachain_system =
						parachain_sproof.lock().unwrap().create_inherent(timestamp.slot().into());
					Ok((timestamp, _aura, parachain_system))
				}
			});
			let aura_provider = AuraConsensusDataProvider::new(client.clone());
			Ok((client, Some(Box::new(aura_provider)), create_inherent_data_providers))
		},
	)?;

	// hand off node.
	tokio_runtime.block_on(callback(node))?;

	Ok(())
}
