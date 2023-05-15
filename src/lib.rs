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
#![deny(missing_docs, unused_extern_crates)]

//! ### substrate-simnode

use sc_cli::{clap::Parser, CliConfiguration, SubstrateCli};
use sc_consensus::BlockImport;
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch};
use sc_service::{TFullBackend, TFullClient};
use sc_transaction_pool_api::TransactionPool;
use sp_api::{ConstructRuntimeApi, TransactionFor};
use sp_consensus::SelectChain;
use sp_inherents::InherentDataProvider;
use sp_runtime::{
	generic::UncheckedExtrinsic,
	traits::{Block as BlockT, SignedExtension},
	MultiAddress, MultiSignature,
};
use std::sync::Arc;

mod client;
mod host_functions;
mod node;
mod rpc;
mod sproof;

pub use client::*;
pub use host_functions::*;
pub use node::*;
pub use sproof::*;

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

/// Type alias for [`sc_transaction_pool_api::TransactionPool`]
type TransactionPoolFor<T> = Arc<
	dyn TransactionPool<
		Block = <T as ChainInfo>::Block,
		Hash = <<T as ChainInfo>::Block as BlockT>::Hash,
		Error = sc_transaction_pool::error::Error,
		InPoolTransaction = sc_transaction_pool::Transaction<
			<<T as ChainInfo>::Block as BlockT>::Hash,
			<<T as ChainInfo>::Block as BlockT>::Extrinsic,
		>,
	>,
>;

/// Wrapper trait for concrete type required by this testing framework.
pub trait ChainInfo: Sized {
	/// Opaque block type
	type Block: BlockT;

	/// ExecutorDispatch dispatch type
	type ExecutorDispatch: NativeExecutionDispatch + 'static;

	/// Runtime
	type Runtime: frame_system::Config;

	/// RuntimeApi
	type RuntimeApi: Send + Sync + 'static + ConstructRuntimeApi<Self::Block, FullClientFor<Self>>;

	/// select chain type.
	type SelectChain: SelectChain<Self::Block> + 'static;

	/// Block import type.
	type BlockImport: Send
		+ Sync
		+ Clone
		+ BlockImport<
			Self::Block,
			Error = sp_consensus::Error,
			Transaction = TransactionFor<FullClientFor<Self>, Self::Block>,
		> + 'static;

	/// The inherent data providers.
	type InherentDataProviders: InherentDataProvider + 'static;

	/// The signed extras required by the runtime
	type SignedExtras: SignedExtension;

	/// Cli utilities
	type Cli: SimnodeCli;

	/// Should return the json rpc Iohandler
	fn rpc_handler<SC>(deps: RpcHandlerArgs<Self, SC>) -> jsonrpsee::RpcModule<()>
	where
		<<Self as ChainInfo>::RuntimeApi as ConstructRuntimeApi<
			Self::Block,
			FullClientFor<Self>,
		>>::RuntimeApi: sp_api::Core<Self::Block>
			+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Self::Block>;

	/// This is for cases you don't yet have the simnode runtime api implemented.
	/// this function is caled in an externalities provided environment, so feel free to read state.
	fn signed_extras(
		from: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtras;
}

/// Cli Extension trait for simnode
pub trait SimnodeCli {
	/// type that implements [`CliConfiguration`]
	type CliConfig: CliConfiguration;

	/// type that implements [`SubstrateCli`]
	type SubstrateCli: Parser + SubstrateCli + Sized;

	/// get a reference to [`CliConfiguration`]
	fn cli_config(cli: &Self::SubstrateCli) -> &Self::CliConfig;

	/// get logging filters
	fn log_filters(cli_config: &Self::CliConfig) -> Result<String, sc_cli::Error>;
}
