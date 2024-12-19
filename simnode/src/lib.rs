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

use polkadot_sdk::*;

use sc_client_api::ExecutorProvider;
use sp_api::ConstructRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, SignedExtension};
use sp_state_machine::{Ext, OverlayedChanges};
use std::sync::Arc;

pub mod cli;
pub mod client;
pub mod overrides;
pub mod rpc;
pub mod sproof;

pub use cli::*;
pub use client::*;
pub use overrides::*;
pub use rpc::*;
pub use sproof::*;

/// Wrapper trait for concrete type required by this testing framework.
pub trait ChainInfo: Sized {
	/// Opaque block type
	type Block: BlockT;

	/// Runtime
	type Runtime: frame_system::Config;

	/// RuntimeApi
	type RuntimeApi: ConstructRuntimeApi<Self::Block, FullClientFor<Self>> + Send + Sync + 'static;

	/// The signed extras required by the runtime
	type SignedExtras: SignedExtension;

	/// This is for cases you don't yet have the simnode runtime api implemented.
	/// this function is caled in an externalities provided environment, so feel free to read state.
	fn signed_extras(
		from: <Self::Runtime as frame_system::Config>::AccountId,
	) -> Self::SignedExtras;
}

/// Runs the given closure in an externalities provided environment, over the blockchain state
pub fn with_state<T: ChainInfo, R>(
	client: Arc<FullClientFor<T>>,
	id: Option<<T::Block as BlockT>::Hash>,
	closure: impl FnOnce() -> R,
) -> R {
	let mut overlay = OverlayedChanges::default();
	let id = id.unwrap_or_else(|| client.info().best_hash);
	let block_number =
		client.number(id).ok().flatten().unwrap_or_else(|| client.info().best_number);
	let mut extensions = client.execution_extensions().extensions(id, block_number);
	let state_backend = client.state_at(id).expect(&format!("State at block {} not found", id));

	let mut ext = Ext::new(&mut overlay, &state_backend, Some(&mut extensions));
	sp_externalities::set_and_run_with_externalities(&mut ext, closure)
}
