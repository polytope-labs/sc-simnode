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
	ChainInfo,
};
use async_trait::async_trait;
use codec::Encode;
use jsonrpsee::{
	core::{Error as RpcError, RpcResult as Result},
	proc_macros::rpc,
};
use sc_client_api::{Backend, ExecutorProvider};
use sc_service::TFullBackend;
use simnode_runtime_api::CreateTransactionApi;
use sp_api::{ApiExt, ConstructRuntimeApi, ProvideRuntimeApi, StorageTransactionCache};
use sp_blockchain::HeaderBackend;
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	Bytes, ExecutionContext,
};
use sp_runtime::{
	traits::{Block as BlockT, Header},
	MultiAddress, MultiSignature,
};
use sp_state_machine::{Ext, OverlayedChanges};
use std::sync::Arc;

use crate::sproof::SharedParachainSproofInherentProvider;
use futures::channel::mpsc;
use manual_seal::EngineCommand;
use polkadot_primitives::UpgradeGoAhead;
use sproof_builder::RelayStateSproofBuilder;

/// Simnode RPC methods.
#[rpc(client, server)]
pub trait SimnodeApi {
	/// Constructs an extrinsic with an empty signature and the given AccountId as the Signer using
	/// simnode's runtime api.
	#[method(name = "simnode_authorExtrinsic")]
	fn author_extrinsic(&self, call: Bytes, account: String) -> Result<Bytes>;

	/// reverts `n` number of blocks and their state from the chain.
	#[method(name = "simnode_revertBlocks")]
	fn revert_blocks(&self, n: u32) -> Result<()>;

	/// Insert the [`UpgradeGoAhead`] command into the inherents as if it came from the relay chain.
	/// This greenlights/aborts a pending runtime upgrade.
	#[method(name = "simnode_upgradeSignal")]
	async fn upgrade_signal(&self, go_ahead: bool) -> Result<()>;
}

/// Handler implementation for Simnode RPC API.
pub struct SimnodeRpcHandler<T: ChainInfo> {
	/// Client type
	client: Arc<FullClientFor<T>>,
	/// Backend type.
	backend: Arc<TFullBackend<T::Block>>,
	/// parachain inherent provider for sproofing the parachain inherent.
	/// Sink for sending commands to the manual seal authorship task.
	parachain: (
		SharedParachainSproofInherentProvider<T>,
		mpsc::Sender<EngineCommand<<T::Block as BlockT>::Hash>>,
	),
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
	pub fn new(
		client: Arc<FullClientFor<T>>,
		backend: Arc<TFullBackend<T::Block>>,
		parachain: (
			SharedParachainSproofInherentProvider<T>,
			mpsc::Sender<EngineCommand<<T::Block as BlockT>::Hash>>,
		),
	) -> Self {
		Self { client, backend, parachain }
	}

	fn author_extrinsic(&self, call: Bytes, account: String) -> Result<Vec<u8>> {
		let at = self.client.info().best_hash;

		let has_api = self
			.client
			.runtime_api()
			.has_api::<dyn CreateTransactionApi<
				T::Block,
				<T::Runtime as frame_system::Config>::RuntimeCall,
				<T::Runtime as frame_system::Config>::AccountId,
			>>(at)
			.map_err(|e| RpcError::Custom(format!("failed read runtime api: {e:?}")))?;

		let ext =
			if has_api {
				let call = codec::Decode::decode(&mut &call.0[..])
					.map_err(|e| RpcError::Custom(format!("failed to decode call: {e:?}")))?;
				let account = AccountId32::from_string(&account)
					.map_err(|e| RpcError::Custom(format!("failed to decode account: {e:?}")))?;
				self.client.runtime_api().create_transaction(at, account.into(), call).map_err(
					|e| RpcError::Custom(format!("CreateTransactionApi is unimplemented: {e:?}")),
				)?
			} else {
				let call = codec::Decode::decode(&mut &call.0[..])
					.map_err(|e| RpcError::Custom(format!("failed to decode call: {e:?}")))?;
				let account = AccountId32::from_string(&account)
					.map_err(|e| RpcError::Custom(format!("failed to decode account: {e:?}")))?;
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
		let mut overlay = OverlayedChanges::default();
		let mut cache = StorageTransactionCache::<
			T::Block,
			<TFullBackend<T::Block> as Backend<T::Block>>::State,
		>::default();
		let id = id.unwrap_or_else(|| self.client.info().best_hash);
		let block_number = self
			.client
			.number(id)
			.ok()
			.flatten()
			.unwrap_or_else(|| self.client.info().best_number);
		let mut extensions = self.client.execution_extensions().extensions(
			id,
			block_number,
			ExecutionContext::BlockConstruction,
		);
		let state_backend =
			self.client.state_at(id).expect(&format!("State at block {} not found", id));

		let mut ext = Ext::new(&mut overlay, &mut cache, &state_backend, Some(&mut extensions));
		sp_externalities::set_and_run_with_externalities(&mut ext, closure)
	}

	fn revert_blocks(&self, n: u32) -> Result<()> {
		self.backend
			.revert(n.into(), true)
			.map_err(|e| RpcError::Custom(format!("failed to revert blocks: {e:?}")))?;

		Ok(())
	}

	/// If this is a parachain node, it will allow you to signal runtime upgrades to your
	/// parachain runtime.
	pub async fn give_upgrade_signal(&self, signal: UpgradeGoAhead) -> Result<()>
	where
		<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
		T::Runtime: parachain_info::Config,
	{
		use futures::{channel::oneshot, SinkExt};

		let para_id =
			self.with_state(None, || parachain_info::Pallet::<T::Runtime>::parachain_id());
		let builder = RelayStateSproofBuilder {
			para_id,
			upgrade_go_ahead: Some(signal),
			..Default::default()
		};
		self.parachain.0.lock().await.update_sproof_builder(builder);

		let mut sink = self.parachain.1.clone();
		let (sender, receiver) = oneshot::channel();
		// NOTE: this sends a Result over the channel.
		let command = EngineCommand::SealNewBlock {
			create_empty: true,
			finalize: true,
			parent_hash: None,
			sender: Some(sender),
		};

		sink.send(command).await?;

		match receiver.await {
			Ok(Ok(_)) => Ok(()),
			Ok(Err(e)) => Err(e.into()),
			Err(e) => Err(RpcError::to_call_error(e)),
		}
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
	T::Runtime: parachain_info::Config,
{
	fn author_extrinsic(&self, call: Bytes, account: String) -> Result<Bytes> {
		Ok(self.author_extrinsic(call, account)?.into())
	}

	fn revert_blocks(&self, n: u32) -> Result<()> {
		self.revert_blocks(n)
	}

	async fn upgrade_signal(&self, go_ahead: bool) -> Result<()> {
		let signal = match go_ahead {
			true => UpgradeGoAhead::GoAhead,
			false => UpgradeGoAhead::Abort,
		};
		// insert the upgrade signal into the sproof provider, it'll be included in the next block.
		self.give_upgrade_signal(signal).await?;

		Ok(())
	}
}
