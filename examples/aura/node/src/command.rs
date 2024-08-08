use crate::{
	benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder},
	chain_spec,
	cli::{Cli, Subcommand},
	rpc, service,
};
use aura_runtime::{Block, EXISTENTIAL_DEPOSIT};
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::SubstrateCli;

use sc_service::PartialComponents;
use sp_keyring::Sr25519Keyring;
use sp_runtime::generic::Era;

#[cfg(feature = "try-runtime")]
use try_runtime_cli::block_building_info::timestamp_with_aura_info;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"" | "local" => Box::new(chain_spec::local_testnet_config()?),
			path =>
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		})
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor =
					sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config, executor)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor =
					sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
				let PartialComponents { client, task_manager, .. } =
					service::new_partial(&config, executor)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor =
					sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
				let PartialComponents { client, task_manager, .. } =
					service::new_partial(&config, executor)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor =
					sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config, executor)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor =
					sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
				let PartialComponents { client, task_manager, backend, .. } =
					service::new_partial(&config, executor)?;
				let aux_revert = Box::new(|client, _, blocks| {
					sc_consensus_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							);
						}

						cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, ()>(Some(
							config.chain_spec,
						))
					},
					BenchmarkCmd::Block(cmd) => {
						let executor =
							sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
						let PartialComponents { client, .. } =
							service::new_partial(&config, executor)?;
						cmd.run(client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						let executor =
							sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
						let PartialComponents { client, backend, .. } =
							service::new_partial(&config, executor)?;
						let db = backend.expose_db();
						let storage = backend.expose_storage();

						cmd.run(config, client, db, storage)
					},
					BenchmarkCmd::Overhead(cmd) => {
						let executor =
							sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
						let PartialComponents { client, .. } =
							service::new_partial(&config, executor)?;
						let ext_builder = RemarkBuilder::new(client.clone());

						cmd.run(
							config,
							client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_builder,
						)
					},
					BenchmarkCmd::Extrinsic(cmd) => {
						let executor =
							sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config);
						let PartialComponents { client, .. } =
							service::new_partial(&config, executor)?;
						// Register the *Remark* and *TKA* builders.
						let ext_factory = ExtrinsicFactory(vec![
							Box::new(RemarkBuilder::new(client.clone())),
							Box::new(TransferKeepAliveBuilder::new(
								client.clone(),
								Sr25519Keyring::Alice.to_account_id(),
								EXISTENTIAL_DEPOSIT,
							)),
						]);

						cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
					},
					BenchmarkCmd::Machine(cmd) =>
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()),
				}
			})
		},
		Some(Subcommand::Simnode(cmd)) => {
			let runner = cli.create_runner(&cmd.run.normalize())?;
			let config = runner.config();

			let executor = sc_simnode::new_wasm_executor(&config);
			let components = service::new_partial(config, executor)?;

			runner.run_node_until_exit(move |config| async move {
				let client = components.client.clone();
				let pool = components.transaction_pool.clone();
				let task_manager = sc_simnode::aura::start_simnode::<RuntimeInfo, _, _, _, _, _>(
					sc_simnode::SimnodeParams {
						components,
						config,
						instant: true,
						rpc_builder: Box::new(move |deny_unsafe, _| {
							let client = client.clone();
							let pool = pool.clone();
							let full_deps = rpc::FullDeps { client, pool, deny_unsafe };
							let io = rpc::create_full(full_deps).expect("Rpc to be initialized");

							Ok(io)
						}),
					},
				)
				.await?;
				Ok(task_manager)
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use crate::service::ExecutorDispatch;
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a, executor runtime, or
				// a task manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				let info_provider = timestamp_with_aura_info(6000);

				Ok((
					cmd.run::<Block, ExtendedHostFunctions<
						sp_io::SubstrateHostFunctions,
						<ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
					>, _>(Some(info_provider)),
					task_manager,
				))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
	}
}

pub struct RuntimeInfo;

impl sc_simnode::ChainInfo for RuntimeInfo {
	// make sure you pass the opaque::Block here
	type Block = aura_runtime::opaque::Block;
	// the runtime type
	type Runtime = aura_runtime::Runtime;
	// the runtime api
	type RuntimeApi = aura_runtime::RuntimeApi;
	// [`SignedExtra`] for your runtime
	type SignedExtras = aura_runtime::SignedExtra;

	// initialize the [`SignedExtra`] for your runtime, you'll notice I'm calling a pallet method in
	// order to read from storage. This is possible becase this method is called in an externalities
	// provided environment. So feel free to reasd your runtime storage.
	fn signed_extras(
		from: <Self::Runtime as frame_system::pallet::Config>::AccountId,
	) -> Self::SignedExtras {
		let nonce = frame_system::Pallet::<Self::Runtime>::account_nonce(from);
		(
			frame_system::CheckNonZeroSender::<Self::Runtime>::new(),
			frame_system::CheckSpecVersion::<Self::Runtime>::new(),
			frame_system::CheckTxVersion::<Self::Runtime>::new(),
			frame_system::CheckGenesis::<Self::Runtime>::new(),
			frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
			frame_system::CheckNonce::<Self::Runtime>::from(nonce),
			frame_system::CheckWeight::<Self::Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
		)
	}
}
