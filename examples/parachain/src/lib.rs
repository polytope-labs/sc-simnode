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

#[cfg(test)]
mod tests {
	use super::*;
	use parachain_inherent::ParachainInherentData;
	use sc_cli::Error;
	use sc_consensus_manual_seal::consensus::{
		aura::AuraConsensusDataProvider, timestamp::SlotTimestampProvider,
	};
	use sc_service::TFullBackend;
	use sp_consensus_babe::AuthorityId;
	use sp_keyring::sr25519::Keyring::Alice;
	use sp_runtime::{generic::Era, traits::IdentifyAccount, MultiSigner};
	use std::sync::Arc;
	use substrate_simnode::{
		build_node_subsystems, build_runtime, ChainInfo, ConfigOrChainSpec, FullClientFor,
		SignatureVerificationOverride, SimnodeCli,
	};

	/// A unit struct which implements `NativeExecutionDispatch` feeding in the
	/// hard-coded runtime.
	pub struct ExecutorDispatch;

	impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
		type ExtendHostFunctions =
			(frame_benchmarking::benchmarking::HostFunctions, SignatureVerificationOverride);

		fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
			parachain_runtime::api::dispatch(method, data)
		}

		fn native_version() -> sc_executor::NativeVersion {
			parachain_runtime::native_version()
		}
	}

	/// ChainInfo implementation.
	struct ParachainTemplateChainInfo;

	impl ChainInfo for ParachainTemplateChainInfo {
		type Block = parachain_runtime::Block;
		type ExecutorDispatch = ExecutorDispatch;
		type Runtime = parachain_runtime::Runtime;
		type RuntimeApi = parachain_runtime::RuntimeApi;
		type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
		type BlockImport = Arc<FullClientFor<Self>>;
		type SignedExtras = parachain_runtime::SignedExtra;
		type InherentDataProviders = (
			SlotTimestampProvider,
			sp_consensus_babe::inherents::InherentDataProvider,
			ParachainInherentData,
		);
		type Cli = ();

		fn signed_extras(
			from: <Self::Runtime as frame_system::Config>::AccountId,
		) -> Self::SignedExtras {
			(
				frame_system::CheckSpecVersion::<Self::Runtime>::new(),
				frame_system::CheckTxVersion::<Self::Runtime>::new(),
				frame_system::CheckGenesis::<Self::Runtime>::new(),
				frame_system::CheckEra::<Self::Runtime>::from(Era::Immortal),
				frame_system::CheckNonce::<Self::Runtime>::from(frame_system::Pallet::<
					Self::Runtime,
				>::account_nonce(from)),
				frame_system::CheckWeight::<Self::Runtime>::new(),
				pallet_transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
			)
		}
	}

	#[test]
	fn substrate_simnode() {
		substrate_simnode::parachain_node::<ParachainTemplateChainInfo, _, _>(|node| async move {
			// seals blocks
			node.seal_blocks(1).await;
			// submit extrinsics
			let alice = MultiSigner::from(Alice.public()).into_account();
			let _hash = node
				.submit_extrinsic(
					frame_system::Call::remark { remark: (b"hello world").to_vec() },
					Some(alice),
				)
				.await
				.unwrap();

			// look ma, I can read state.
			let _events = node
				.with_state(None, || frame_system::Pallet::<parachain_runtime::Runtime>::events());
			// get access to the underlying client.
			let _client = node.client();
		});
	}
}
