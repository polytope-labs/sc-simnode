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

#[cfg(feature = "aura")]
pub mod aura;

#[cfg(feature = "parachain")]
pub mod parachain;

#[cfg(feature = "babe")]
pub mod babe;
pub mod timestamp;

use crate::{ChainInfo, SignatureVerificationOverride};
use jsonrpsee::RpcModule;
use sc_executor::{HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY};
use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient};
use sc_telemetry::Telemetry;
use sp_runtime::{generic::UncheckedExtrinsic, MultiAddress, MultiSignature};
use sp_wasm_interface::ExtendedHostFunctions;

/// The simnode executor type, we use the wasm executor to force the runtime use host functions
/// instead of native code for signature verification, this in turn uses our signature verification
/// overrides.
pub type Executor = WasmExecutor<
	ExtendedHostFunctions<sp_io::SubstrateHostFunctions, SignatureVerificationOverride>,
>;

/// Creates a [`WasmExecutor`] according to [`Configuration`].
pub fn new_wasm_executor(config: &Configuration) -> Executor {
	let strategy = config
		.default_heap_pages
		.map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |p| HeapAllocStrategy::Static { extra_pages: p as _ });
	WasmExecutor::builder()
		.with_execution_method(config.wasm_method)
		.with_onchain_heap_alloc_strategy(strategy)
		.with_offchain_heap_alloc_strategy(strategy)
		.with_max_runtime_instances(config.max_runtime_instances)
		.with_runtime_cache_size(config.runtime_cache_size)
		.build()
}

/// Type alias for [`sc_service::TFullClient`]
pub type FullClientFor<C> =
	TFullClient<<C as ChainInfo>::Block, <C as ChainInfo>::RuntimeApi, Executor>;

/// UncheckedExtrinsic type for Simnode
pub type UncheckedExtrinsicFor<T> = UncheckedExtrinsic<
	MultiAddress<
		<<T as ChainInfo>::Runtime as frame_system::Config>::AccountId,
		<<T as ChainInfo>::Runtime as frame_system::Config>::Nonce,
	>,
	<<T as ChainInfo>::Runtime as frame_system::Config>::RuntimeCall,
	MultiSignature,
	<T as ChainInfo>::SignedExtras,
>;

/// Type alias for [`sc_service::TFullBackend`]
pub type FullBackendFor<C> = TFullBackend<<C as ChainInfo>::Block>;

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
