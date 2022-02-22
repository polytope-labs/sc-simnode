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
#![deny(unused_extern_crates, missing_docs)]

//! Basic example of end to end parachain runtime tests

use frame_benchmarking::frame_support::metadata::StorageEntryModifier::Default;
use parachain_inherent::ParachainInherentData;
use sc_cli::{CliConfiguration, Error};
use sc_consensus_manual_seal::consensus::timestamp::SlotTimestampProvider;
use sc_service::TFullBackend;
use sp_keyring::sr25519::Keyring::Alice;
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::sync::Arc;
use substrate_simnode::{
	ChainInfo, FullClientFor, RpcHandlerArgs, SignatureVerificationOverride, SimnodeCli,
};

/// A unit struct which implements `NativeExecutionDispatch` feeding in the
/// hard-coded runtime.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions = (
		frame_benchmarking::benchmarking::HostFunctions,
		// This allows [`Node::<T>::submit_extrinsic`] work by disabling
		// runtime signature verification.
		SignatureVerificationOverride,
	);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		parachain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		parachain_runtime::native_version()
	}
}

/// the substrate parachain template dosn't have a public cli
/// interface, so we use polkadot's but it won't work. Because the cli interface
/// loads the genesis wasm, which isn't  a parachain runtime but is instead the polkadot runtime.
///
/// This example is simply to have rust code that compiles and in the future we should probably
/// set up a parachain runtime just for the purposes of a run-able example.
struct PolkadotCli;

impl SimnodeCli for PolkadotCli {
	type CliConfig = sc_cli::RunCmd;
	type SubstrateCli = polkadot_cli::Cli;

	fn cli_config(cli: &Self::SubstrateCli) -> &Self::CliConfig {
		&cli.run.base
	}

	fn log_filters(cli_config: &Self::CliConfig) -> Result<String, Error> {
		cli_config.log_filters()
	}
}

/// ChainInfo implementation.
struct ParachainTemplateChainInfo;

impl ChainInfo for ParachainTemplateChainInfo {
	// Opaque block type!
	type Block = parachain_runtime::opaque::Block;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = parachain_runtime::Runtime;
	type RuntimeApi = parachain_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	// no consensus checks are performed on blocks that are to be imported.
	type BlockImport = Arc<FullClientFor<Self>>;
	// Inherent providers
	type InherentDataProviders = (
		//  Here we use [`SlotTimestampProvider`] to provide us with the next timestamp,
		// based on the runtime configured minimum duration between blocks and the current
		// slot number.
		SlotTimestampProvider,
		// Aura uses the timestamp from [`SlotTimestampProvider`] to calculate the current
		// slot number.
		sp_consensus_aura::inherents::InherentDataProvider,
		// This mocks the inherents expected by parachain_system, allowing us author blocks
		// valid for parachain runtimes
		ParachainInherentData,
	);
	// Pass your Cli impl here
	type Cli = PolkadotCli;

	fn create_rpc_io_handler<SC>(
		_deps: RpcHandlerArgs<Self, SC>,
	) -> jsonrpc_core::MetaIoHandler<sc_rpc::Metadata> {
		Default::default()
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	substrate_simnode::parachain_node::<ParachainTemplateChainInfo, _, _>(|node| async move {
		// submit extrinsics to the tx pool.
		let alice = MultiSigner::from(Alice.public()).into_account();
		let _hash = node
			.submit_extrinsic(
				frame_system::Call::remark_with_event { remark: (b"hello world").to_vec() },
				alice,
			)
			.await
			.unwrap();

		// seals a block, including any txs in the tx pool.
		node.seal_blocks(1).await;

		// look ma, I can read state.
		let _events =
			node.with_state(None, || frame_system::Pallet::<parachain_runtime::Runtime>::events());

		println!("{:#?}", _events);

		// get access to the underlying client.
		let _client = node.client();

		Ok(())
	})
}
