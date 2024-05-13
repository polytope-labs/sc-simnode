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

use crate::{
	client::{FullClientFor, UncheckedExtrinsicFor},
	with_state, ChainInfo,
};
use async_trait::async_trait;
use codec::Encode;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, types::ErrorObjectOwned};
use sc_client_api::Backend;
use sc_service::TFullBackend;
use simnode_runtime_api::CreateTransactionApi;
use sp_api::{ApiExt, ConstructRuntimeApi, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	Bytes,
};
use sp_runtime::{
	traits::{Block as BlockT, Header},
	MultiAddress, MultiSignature,
};

use std::sync::Arc;

/// Simnode RPC methods.
#[rpc(client, server)]
pub trait SimnodeApi {
	/// Constructs an extrinsic with an empty signature and the given AccountId as the Signer using
	/// simnode's runtime api.
	#[method(name = "simnode_authorExtrinsic")]
	fn author_extrinsic(&self, call: Bytes, account: String) -> RpcResult<Bytes>;

	/// reverts `n` number of blocks and their state from the chain.
	#[method(name = "simnode_revertBlocks")]
	fn revert_blocks(&self, n: u32) -> RpcResult<()>;

	/// Insert the [`UpgradeGoAhead`] command into the inherents as if it came from the relay chain.
	/// This greenlights/aborts a pending runtime upgrade.
	#[method(name = "simnode_upgradeSignal")]
	async fn upgrade_signal(&self, go_ahead: bool) -> RpcResult<()>;
}

/// Handler implementation for Simnode RPC API.
pub struct SimnodeRpcHandler<T: ChainInfo> {
	/// Client type
	client: Arc<FullClientFor<T>>,
	/// Backend type.
	backend: Arc<TFullBackend<T::Block>>,
}

impl<T> SimnodeRpcHandler<T>
where
	T: ChainInfo,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as frame_system::Config>::RuntimeCall,
			<T::Runtime as frame_system::Config>::AccountId,
		>,
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
{
	/// Creates a new instance of simnode's RPC handler.
	pub fn new(client: Arc<FullClientFor<T>>, backend: Arc<TFullBackend<T::Block>>) -> Self {
		Self { client, backend }
	}

	fn author_extrinsic(&self, call: Bytes, account: String) -> RpcResult<Vec<u8>> {
		let at = self.client.info().best_hash;

		let has_api = self
			.client
			.runtime_api()
			.has_api::<dyn CreateTransactionApi<
				T::Block,
				<T::Runtime as frame_system::Config>::RuntimeCall,
				<T::Runtime as frame_system::Config>::AccountId,
			>>(at)
			.map_err(|e| {
				ErrorObjectOwned::owned::<&str>(
					1000,
					format!("failed read runtime api: {e:?}"),
					None,
				)
			})?;

		let ext = if has_api {
			let call = codec::Decode::decode(&mut &call.0[..]).map_err(|e| {
				ErrorObjectOwned::owned::<&str>(1001, format!("failed to decode call: {e:?}"), None)
			})?;
			let account = AccountId32::from_string(&account).map_err(|e| {
				ErrorObjectOwned::owned::<&str>(
					1002,
					format!("failed to decode account: {e:?}"),
					None,
				)
			})?;
			self.client
				.runtime_api()
				.create_transaction(at, account.into(), call)
				.map_err(|e| {
					ErrorObjectOwned::owned::<&str>(
						1003,
						format!("CreateTransactionApi is unimplemented: {e:?}"),
						None,
					)
				})?
		} else {
			let call = codec::Decode::decode(&mut &call.0[..]).map_err(|e| {
				ErrorObjectOwned::owned::<&str>(1004, format!("failed to decode call: {e:?}"), None)
			})?;
			let account = AccountId32::from_string(&account).map_err(|e| {
				ErrorObjectOwned::owned::<&str>(
					1005,
					format!("failed to decode account: {e:?}"),
					None,
				)
			})?;
			let extra = self.with_state(None, || T::signed_extras(account.clone().into()));
			let ext = UncheckedExtrinsicFor::<T>::new_signed(
				call,
				MultiAddress::Id(account.into()),
				MultiSignature::Sr25519(sp_core::sr25519::Signature::from_raw([0u8; 64])),
				extra,
			);
			ext.encode()
		};

		Ok(ext)
	}

	/// Runs the given closure in an externalities provided environment, over the blockchain state
	pub fn with_state<R>(
		&self,
		id: Option<<T::Block as BlockT>::Hash>,
		closure: impl FnOnce() -> R,
	) -> R {
		with_state::<T, R>(self.client.clone(), id, closure)
	}

	fn revert_blocks(&self, n: u32) -> RpcResult<()> {
		self.backend.revert(n.into(), true).map_err(|e| {
			ErrorObjectOwned::owned::<&str>(2050, format!("failed to revert blocks: {e:?}"), None)
		})?;

		Ok(())
	}
}

#[async_trait]
impl<T> SimnodeApiServer for SimnodeRpcHandler<T>
where
	T: ChainInfo + Send + Sync + 'static,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as frame_system::Config>::RuntimeCall,
			<T::Runtime as frame_system::Config>::AccountId,
		>,
	<T::Runtime as frame_system::Config>::AccountId: From<AccountId32>,
	<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
{
	fn author_extrinsic(&self, call: Bytes, account: String) -> RpcResult<Bytes> {
		Ok(self.author_extrinsic(call, account)?.into())
	}

	fn revert_blocks(&self, n: u32) -> RpcResult<()> {
		self.revert_blocks(n)
	}

	async fn upgrade_signal(&self, _go_ahead: bool) -> RpcResult<()> {
		Err(ErrorObjectOwned::owned::<&str>(
			3050,
			format!("standalone runtimes don't need permission to upgrade their runtime"),
			None,
		))
	}
}
