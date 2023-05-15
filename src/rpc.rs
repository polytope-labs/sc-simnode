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

//! Simnode RPC Subsystem

use crate::{ChainInfo, FullClientFor};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult as Result},
	proc_macros::rpc,
};
use simnode_runtime_api::CreateTransactionApi;
use sp_api::{ConstructRuntimeApi, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::{Bytes,crypto::{Ss58Codec, AccountId32}};
use std::sync::Arc;

/// Simnode RPC methods.
#[rpc(client, server)]
pub trait SimnodeApi {
	/// Constructs an extrinsic using simnode's runtime api.
	#[method(name = "simnode_authorExtrinsic")]
	fn author_extrinsic(&self, call: Bytes, account: String) -> Result<Bytes>;
}

/// Handler implementation for Simnode RPC API.
pub struct SimnodeRpcHandler<T: ChainInfo> {
	client: Arc<FullClientFor<T>>,
}

impl<T: ChainInfo> SimnodeRpcHandler<T> {
	/// Creates a new instance of simnode's RPC handler.
	pub fn new(client: Arc<FullClientFor<T>>) -> Self {
		Self { client }
	}
}

impl<T> SimnodeApiServer for SimnodeRpcHandler<T>
where
	T: ChainInfo + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as frame_system::Config>::RuntimeCall,
			<T::Runtime as frame_system::Config>::AccountId,
		>,
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
{
	fn author_extrinsic(&self, call: Bytes, account: String) -> Result<Bytes> {
		let at = self.client.info().best_hash;
		let call = codec::Decode::decode(&mut &call.0[..])
			.map_err(|e| RpcError::Custom(format!("failed to decode call: {e:?}")))?;
		let account = AccountId32::from_string(&account)
			.map_err(|e| RpcError::Custom(format!("failed to decode account: {e:?}")))?;
		let extrinsic =
			self.client.runtime_api().create_transaction(at, account.into(), call).map_err(|e| {
				RpcError::Custom(format!("CreateTransactionApi is unimplemented: {e:?}"))
			})?;

		Ok(extrinsic.into())
	}
}
