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
#![deny(missing_docs, unused_extern_crates)]

//! ### sc-simnode

use crate::client::RpcHandlerArgs;
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch};
use sp_api::ConstructRuntimeApi;
use sp_runtime::traits::{Block as BlockT, SignedExtension};

pub mod cli;
pub mod client;
pub mod host_functions;
pub mod parachain;
pub mod rpc;
pub mod sproof;

pub use cli::*;
pub use client::*;
pub use host_functions::*;
pub use parachain::*;
pub use rpc::*;
pub use sproof::*;

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

	/// The signed extras required by the runtime
	type SignedExtras: SignedExtension;

	/// Should return the json rpc Iohandler
	fn rpc_handler(deps: RpcHandlerArgs<Self>) -> jsonrpsee::RpcModule<()>
	where
		<Self::RuntimeApi as ConstructRuntimeApi<Self::Block, FullClientFor<Self>>>::RuntimeApi:
			sp_api::Core<Self::Block>
				+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Self::Block>;

	/// This is for cases you don't yet have the simnode runtime api implemented.
	/// this function is caled in an externalities provided environment, so feel free to read state.
	fn signed_extras(
		from: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtras;
}
