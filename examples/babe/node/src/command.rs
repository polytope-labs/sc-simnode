// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

use polkadot_sdk::*;

use super::benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder};
use crate::{
	chain_spec, service,
	service::{new_partial, FullClient},
	Cli, Subcommand,
};
use babe_runtime::{opaque, Block, ExistentialDeposit};
use frame_benchmarking_cli::*;
use sc_cli::{Result, SubstrateCli};
use sc_service::PartialComponents;
use sp_keyring::Sr25519Keyring;

use sp_runtime::generic::Era;
use std::sync::Arc;

#[cfg(feature = "try-runtime")]
use babe_runtime::constants::time::SLOT_DURATION;

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
		"https://github.com/paritytech/substrate/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			"dev" => Box::new(chain_spec::development_config()),
			"local" => Box::new(chain_spec::local_testnet_config()),
			"staging" => Box::new(chain_spec::staging_testnet_config()),
			path =>
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};
		Ok(spec)
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config, cli).map_err(sc_cli::Error::Service)
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
						// ensure that we keep the task manager alive
						let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
							&config.executor,
						);
						let partial = new_partial(&config, executor)?;
						cmd.run(partial.client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						// ensure that we keep the task manager alive
						let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
							&config.executor,
						);
						let partial = new_partial(&config, executor)?;
						let db = partial.backend.expose_db();
						let storage = partial.backend.expose_storage();

						cmd.run(config, partial.client, db, storage)
					},
					BenchmarkCmd::Overhead(cmd) => {
						// ensure that we keep the task manager alive
						let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
							&config.executor,
						);
						let partial = new_partial(&config, executor)?;
						let ext_builder = RemarkBuilder::new(partial.client.clone());

						cmd.run(
							config.chain_spec.name().into(),
							partial.client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_builder,
							false,
						)
					},
					BenchmarkCmd::Extrinsic(cmd) => {
						// ensure that we keep the task manager alive
						let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
							&config.executor,
						);
						let partial = service::new_partial(&config, executor)?;
						// Register the *Remark* and *TKA* builders.
						let ext_factory = ExtrinsicFactory(vec![
							Box::new(RemarkBuilder::new(partial.client.clone())),
							Box::new(TransferKeepAliveBuilder::new(
								partial.client.clone(),
								Sr25519Keyring::Alice.to_account_id(),
								ExistentialDeposit::get(),
							)),
						]);

						cmd.run(
							partial.client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_factory,
						)
					},
					BenchmarkCmd::Machine(cmd) =>
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()),
				}
			})
		},
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
					&config.executor,
				);
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config, executor)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
					&config.executor,
				);
				let PartialComponents { client, task_manager, .. } =
					new_partial(&config, executor)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
					&config.executor,
				);
				let PartialComponents { client, task_manager, .. } =
					new_partial(&config, executor)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
					&config.executor,
				);
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config, executor)?;
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
				let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(
					&config.executor,
				);
				let PartialComponents { client, task_manager, backend, .. } =
					new_partial(&config, executor)?;
				let aux_revert = Box::new(|client: Arc<FullClient>, backend, blocks| {
					sc_consensus_babe::revert(client.clone(), backend, blocks)?;
					sc_consensus_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Simnode(cmd)) => {
			let runner = cli.create_runner(&cmd.run.normalize())?;
			let config = runner.config();
			let executor = sc_simnode::new_wasm_executor(&config);

			let PartialComponents {
				client,
				backend,
				task_manager,
				import_queue,
				keystore_container,
				select_chain,
				transaction_pool,
				other: (rpc_builder, (block_import, _, babe_link), _, telemetry),
			} = new_partial(&config, executor)?;

			let components = PartialComponents {
				client,
				backend,
				task_manager,
				import_queue,
				keystore_container,
				select_chain,
				transaction_pool,
				other: (block_import, telemetry, babe_link),
			};

			runner.run_node_until_exit(move |config| async move {
				let task_manager = sc_simnode::babe::start_simnode::<RuntimeInfo, _, _, _, _>(
					sc_simnode::SimnodeParams {
						components,
						config,
						instant: true,
						rpc_builder: Box::new(rpc_builder),
					},
				)
				.await?;
				Ok(task_manager)
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
	}
}

/// Runtime description for simnode
pub struct RuntimeInfo;

impl sc_simnode::ChainInfo for RuntimeInfo {
	// make sure you pass the opaque::Block here
	type Block = opaque::Block;
	// the runtime type
	type Runtime = babe_runtime::Runtime;
	// the runtime api
	type RuntimeApi = babe_runtime::RuntimeApi;
	// [`SignedExtra`] for your runtime
	type SignedExtras = babe_runtime::SignedExtra;

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
