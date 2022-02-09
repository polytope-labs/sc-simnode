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
use codec::Encode;
use sp_runtime::{traits::Extrinsic, MultiSignature};

/// Create transaction.
/// This trait is meant to be implemented by the runtime and is responsible for constructing
/// a transaction to be included in the block.
pub trait CreateTransaction<T: frame_system::Config> {
	type Extrinsic: Extrinsic + Encode;
	/// Attempt to create signed transaction
	/// Runtime implementation is free to construct the payload to sign
	/// in any way it wants.
	/// Returns `None` if signed transaction could not be created (
	/// for runtime-specific reason).
	/// If signer is None, an unsigned extrinsic should be created.
	fn create_transaction(
		call: <T as frame_system::Config>::Call,
		account: Option<<T as frame_system::Config>::AccountId>,
		signature: MultiSignature,
	) -> Option<Self::Extrinsic>;
}
