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

//! Host function overrides for signature verification.

use polkadot_sdk::*;

use sp_core::{ecdsa, ed25519, sr25519};
use sp_runtime_interface::{
	pass_by::{PassFatPointerAndRead, PassPointerAndRead},
	runtime_interface,
};

#[runtime_interface]
trait Crypto {
	fn ecdsa_verify(
		_sig: PassPointerAndRead<&ecdsa::Signature, 65>,
		_msg: PassFatPointerAndRead<&[u8]>,
		_pub_key: PassPointerAndRead<&ecdsa::Public, 33>,
	) -> bool {
		true
	}

	#[version(2)]
	fn ecdsa_verify(
		_sig: PassPointerAndRead<&ecdsa::Signature, 65>,
		_msg: PassFatPointerAndRead<&[u8]>,
		_pub_key: PassPointerAndRead<&ecdsa::Public, 33>,
	) -> bool {
		true
	}

	fn ed25519_verify(
		_sig: PassPointerAndRead<&ed25519::Signature, 64>,
		_msg: PassFatPointerAndRead<&[u8]>,
		_pub_key: PassPointerAndRead<&ed25519::Public, 32>,
	) -> bool {
		true
	}

	fn sr25519_verify(
		_sig: PassPointerAndRead<&sr25519::Signature, 64>,
		_msg: PassFatPointerAndRead<&[u8]>,
		_pub_key: PassPointerAndRead<&sr25519::Public, 32>,
	) -> bool {
		true
	}

	#[version(2)]
	fn sr25519_verify(
		_sig: PassPointerAndRead<&sr25519::Signature, 64>,
		_msg: PassFatPointerAndRead<&[u8]>,
		_pub_key: PassPointerAndRead<&sr25519::Public, 32>,
	) -> bool {
		true
	}
}

/// Provides host functions that overrides runtime signature verification
/// to always return true.
pub type SignatureVerificationOverride = crypto::HostFunctions;

// This is here to get rid of the warnings.
#[allow(unused_imports, dead_code)]
use self::crypto::{ecdsa_verify, ed25519_verify, sr25519_verify};
