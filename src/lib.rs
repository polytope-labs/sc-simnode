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

use std::sync::Arc;
use sc_consensus::BlockImport;
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch};
use sc_service::TFullClient;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{ConstructRuntimeApi, TransactionFor};
use sp_consensus::SelectChain;
use sp_inherents::InherentDataProvider;
use sp_runtime::traits::{Block as BlockT, SignedExtension};

mod client;
mod host_functions;
mod node;
mod sproof;
mod utils;

pub use client::*;
pub use host_functions::*;
pub use node::*;
pub use sproof::*;
pub use utils::*;

/// Type alias for [`sc_service::TFullClient`]
pub type FullClientFor<C> = TFullClient<
	<C as ChainInfo>::Block,
	<C as ChainInfo>::RuntimeApi,
	NativeElseWasmExecutor<<C as ChainInfo>::ExecutorDispatch>,
>;

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

	/// The signed extras required by the runtime
	type SignedExtras: SignedExtension;

	/// The inherent data providers.
	type InherentDataProviders: InherentDataProvider + 'static;

	/// Signed extras, this function is caled in an externalities provided environment.
	fn signed_extras(
		from: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtras;
}
