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

//! Basic example of end to end runtime tests with a standalone blockchain

use frame_benchmarking::frame_support::metadata::StorageEntryModifier::Default;
use grandpa::GrandpaBlockImport;
use sc_cli::CliConfiguration;
use sc_consensus_babe::BabeBlockImport;
use sc_consensus_manual_seal::consensus::{
	babe::BabeConsensusDataProvider, timestamp::SlotTimestampProvider,
};
use sc_service::TFullBackend;
use sp_consensus_babe::AuthorityId;
use sp_keyring::sr25519::Keyring::Alice;
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::sync::Arc;
use substrate_simnode::{
	ChainInfo, FullClientFor, RpcHandlerArgs, SignatureVerificationOverride, SimnodeCli,
};

/// Stand alone blockchains have to define their own [`BlockImport`] implementation, which may
/// include digests in the block header that are relevant to the runtime.
type BlockImport<B, BE, C, SC> = BabeBlockImport<B, C, GrandpaBlockImport<BE, B, C, SC>>;

/// A unit struct which implements [`NativeExecutionDispatch`] feeding in the
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
		node_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		node_runtime::native_version()
	}
}

/// the substrate standalone node template dosn't have a public cli
/// interface, so we use polkadot's but it won't work. Because the polkadot runtime loads
/// the genesis wasm, which isn't  a parachain runtime but is instead the polkadot runtime which
/// expects some inherents and i haven't looked into mocking them yet.
///
/// This example is simply to have rust code that compiles and in the future we should probably
/// set up a standalone runtime just for the purposes of a run-able example.
struct PolkadotCli;

impl SimnodeCli for PolkadotCli {
	type CliConfig = sc_cli::RunCmd;
	type SubstrateCli = polkadot_cli::Cli;

	fn cli_config(cli: &Self::SubstrateCli) -> &Self::CliConfig {
		&cli.run.base
	}

	fn log_filters(cli_config: &Self::CliConfig) -> Result<String, sc_cli::Error> {
		cli_config.log_filters()
	}
}

/// ChainInfo implementation.
struct NodeTemplateChainInfo;

impl ChainInfo for NodeTemplateChainInfo {
	// Opaque block type!
	type Block = node_primitives::Block;
	type ExecutorDispatch = ExecutorDispatch;
	type Runtime = node_runtime::Runtime;
	type RuntimeApi = node_runtime::RuntimeApi;
	type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
	type BlockImport =
		BlockImport<Self::Block, TFullBackend<Self::Block>, FullClientFor<Self>, Self::SelectChain>;
	// Inherent providers
	type InherentDataProviders = (
		//  Here we use [`SlotTimestampProvider`] to provide us with the next timestamp,
		// based on the runtime configured minimum duration between blocks and the current
		// slot number.
		SlotTimestampProvider,
		// Babe uses the timestamp from [`SlotTimestampProvider`] to calculate the current
		// slot number.
		sp_consensus_babe::inherents::InherentDataProvider,
	);
	// Pass your Cli impl here
	type Cli = PolkadotCli;
	fn create_rpc_io_handler<SC>(
		_deps: RpcHandlerArgs<Self, SC>,
	) -> jsonrpc_core::io::MetaIoHandler<sc_rpc_api::metadata::Metadata> {
		Default::default()
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	substrate_simnode::standalone_node::<NodeTemplateChainInfo, _, _, _>(
		|client, select_chain, keystore, _| {
			// set up the babe -> grandpa import pipeline
			let (grandpa_block_import, ..) = grandpa::block_import(
				client.clone(),
				&(client.clone() as Arc<_>),
				select_chain.clone(),
				None,
			)?;

			let slot_duration = sc_consensus_babe::Config::get(&*client)?;
			let (block_import, babe_link) = sc_consensus_babe::block_import(
				slot_duration.clone(),
				grandpa_block_import,
				client.clone(),
			)?;

			// set up our standalone runtime's consensus data provider, used by manual seal
			// to include any digests expected by the consensus pallets of your runtime
			// in order to author blocks that are valid for your runtime.
			let consensus_data_provider = BabeConsensusDataProvider::new(
				client.clone(),
				keystore.sync_keystore(),
				babe_link.epoch_changes().clone(),
				vec![(AuthorityId::from(Alice.public()), 1000)],
			)
			.expect("failed to create ConsensusDataProvider");

			let create_inherent_data_providers = {
				let cloned_client = client.clone();

				Box::new(move |_, _| {
					let client = cloned_client.clone();
					async move {
						// inherents that our runtime expects.
						let timestamp = SlotTimestampProvider::new_babe(client.clone())
							.map_err(|err| format!("{:?}", err))?;
						let babe = sp_consensus_babe::inherents::InherentDataProvider::new(
							timestamp.slot().into(),
						);
						Ok((timestamp, babe))
					}
				})
			};

			Ok((
				block_import,
				Some(Box::new(consensus_data_provider)),
				create_inherent_data_providers,
			))
		},
		// here we'll get the node
		|node| async move {
			// seals blocks
			node.seal_blocks(1).await;
			// submit extrinsics
			let alice = MultiSigner::from(Alice.public()).into_account();
			let _hash = node
				.submit_extrinsic(
					frame_system::Call::remark_with_event { remark: (b"hello world").to_vec() },
					alice,
				)
				.await
				.unwrap();

			// look ma, I can read state.
			let _events =
				node.with_state(None, || frame_system::Pallet::<node_runtime::Runtime>::events());

			println!("{:#?}", _events);
			// get access to the underlying client.
			let _client = node.client();

			Ok(())
		},
	)
}
