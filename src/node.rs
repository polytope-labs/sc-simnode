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
use std::sync::{Arc, Mutex};

use crate::{
	sproof::ParachainInherentSproofProvider, ChainInfo, FullClientFor, TransactionPoolFor,
	UncheckedExtrinsicFor,
};
use futures::{
	channel::{mpsc, oneshot},
	FutureExt, SinkExt,
};
use jsonrpsee::RpcModule;
use manual_seal::EngineCommand;
use polkadot_primitives::v2::UpgradeGoAhead;
use sc_client_api::{backend::Backend, CallExecutor, ExecutorProvider};
use sc_executor::NativeElseWasmExecutor;
use sc_service::{TFullBackend, TFullCallExecutor, TFullClient, TaskManager};
use simnode_runtime_apis::CreateTransactionApi;
use sp_api::{ConstructRuntimeApi, OverlayedChanges, ProvideRuntimeApi, StorageTransactionCache};
use sp_blockchain::HeaderBackend;
use sp_core::ExecutionContext;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header, NumberFor},
	transaction_validity::TransactionSource,
	MultiSignature, OpaqueExtrinsic,
};
use sp_state_machine::Ext;
use sproof_builder::RelayStateSproofBuilder;

/// Shared instance of [`ParachainInherentSproofProvider`]
pub type SharedParachainInherentProvider<T> =
	Arc<Mutex<ParachainInherentSproofProvider<<T as ChainInfo>::Block, FullClientFor<T>>>>;

/// Node Error type
#[derive(Debug)]
pub enum Error {
	/// Transaction pool errors
	TxPool(sc_transaction_pool::error::Error),
	/// Extrinsic related errors
	ExtrinsicError(String),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

impl std::error::Error for Error {}

/// This holds a reference to a running node on another thread,
/// the node process is dropped when this struct is dropped
/// also holds logs from the process.
pub struct Node<T: ChainInfo> {
	/// rpc handler for communicating with the node over rpc.
	pub(crate) rpc_handler: Arc<RpcModule<()>>,
	/// handle to the running node.
	pub(crate) task_manager: Option<TaskManager>,
	/// client instance
	pub(crate) client:
		Arc<TFullClient<T::Block, T::RuntimeApi, NativeElseWasmExecutor<T::ExecutorDispatch>>>,
	/// transaction pool
	pub(crate) pool: TransactionPoolFor<T>,
	/// channel to communicate with manual seal on.
	pub(crate) manual_seal_command_sink: mpsc::Sender<EngineCommand<<T::Block as BlockT>::Hash>>,
	/// backend type.
	pub(crate) backend: Arc<TFullBackend<T::Block>>,
	/// Block number at initialization of this Node.
	pub(crate) initial_block_number: NumberFor<T::Block>,
	/// a reference to the [`ParachainInherentSproofProvider`] for setting runtime upgrade signals
	pub(crate) parachain_inherent_provider: Option<SharedParachainInherentProvider<T>>,
}

type EventRecord<T> = frame_system::EventRecord<
	<T as frame_system::Config>::RuntimeEvent,
	<T as frame_system::Config>::Hash,
>;

impl<T> Node<T>
where
	T: ChainInfo,
	<T::RuntimeApi as ConstructRuntimeApi<T::Block, FullClientFor<T>>>::RuntimeApi:
		CreateTransactionApi<
			T::Block,
			<T::Runtime as frame_system::Config>::AccountId,
			<T::Runtime as frame_system::Config>::RuntimeCall,
		>,
	<<T as ChainInfo>::Runtime as frame_system::Config>::AccountId: codec::Codec,
	<<T as ChainInfo>::Runtime as frame_system::Config>::RuntimeCall: codec::Codec,
	<<T::Block as BlockT>::Header as Header>::Number: From<u32>,
	<<T as ChainInfo>::Block as BlockT>::Extrinsic: From<OpaqueExtrinsic>,
{
	/// Returns a reference to the rpc handlers, use this to send rpc requests.
	/// eg
	/// ```ignore
	/// 	let request = r#"{"jsonrpc":"2.0","method":"engine_createBlock","params": [true, true],"id":1}"#;
	/// 	let response = node.rpc_handler().raw_json_request(request).await;
	/// ```
	pub fn rpc_handler(&self) -> Arc<RpcModule<()>> {
		self.rpc_handler.clone()
	}

	/// Return a reference to the Client
	pub fn client(
		&self,
	) -> Arc<TFullClient<T::Block, T::RuntimeApi, NativeElseWasmExecutor<T::ExecutorDispatch>>> {
		self.client.clone()
	}

	/// Return a reference to the pool.
	pub fn pool(&self) -> TransactionPoolFor<T> {
		self.pool.clone()
	}

	/// Allows you read state at any given block, provided it hasn't been pruned.
    pub fn with_state<R>(&self, id: Option<<T::Block as BlockT>::Hash>, closure: impl FnOnce() -> R) -> R
        where
            <TFullCallExecutor<T::Block, NativeElseWasmExecutor<T::ExecutorDispatch>> as CallExecutor<T::Block>>::Error:
            std::fmt::Debug,
    {
		let mut overlay = OverlayedChanges::default();
		let mut cache = StorageTransactionCache::<
			T::Block,
			<TFullBackend<T::Block> as Backend<T::Block>>::State,
		>::default();
		let id = id.unwrap_or_else(|| self.client.info().best_hash);
		let mut extensions = self
			.client
			.execution_extensions()
			.extensions(&BlockId::Hash(id), ExecutionContext::BlockConstruction);
		let state_backend =
			self.backend.state_at(id).expect(&format!("State at block {} not found", id));

		let mut ext = Ext::new(&mut overlay, &mut cache, &state_backend, Some(&mut extensions));
		sp_externalities::set_and_run_with_externalities(&mut ext, closure)
	}

	/// submit some extrinsic to the node. if signer is None, will submit unsigned_extrinsic.
	pub async fn submit_extrinsic(
		&self,
		call: impl Into<<T::Runtime as frame_system::Config>::RuntimeCall> + Clone,
		signer: <T::Runtime as frame_system::Config>::AccountId,
	) -> Result<<T::Block as BlockT>::Hash, Error> {
		let at = self.client.info().best_hash;
		let id = BlockId::Hash(at);
		let extrinsic =
			self.client
				.runtime_api()
				.create_transaction(&id, call.clone().into(), signer.clone());
		let ext_bytes = if let Ok(raw_bytes) = extrinsic {
			raw_bytes
		} else {
			let extra = self.with_state(None, || T::signed_extras(signer.clone()));
			let ext = UncheckedExtrinsicFor::<T>::new_signed(
				call.into(),
				signer.into(),
				MultiSignature::Sr25519(sp_core::sr25519::Signature::from_raw([0u8; 64])),
				extra,
			);
			ext.encode()
		};
		let ext = OpaqueExtrinsic::from_bytes(&ext_bytes[..])
			.map_err(|_| Error::ExtrinsicError("Could not decode extrinsic".into()))?;

		self.pool
			.submit_one(&BlockId::Hash(at), TransactionSource::Local, ext.into())
			.await
			.map_err(|e| Error::TxPool(e))
	}

	/// Get the events at [`BlockId`]
	pub fn events(&self, id: Option<<T::Block as BlockT>::Hash>) -> Vec<EventRecord<T::Runtime>> {
		self.with_state(id, || frame_system::Pallet::<T::Runtime>::events())
	}

	/// If this is a parachain node, it will allow you to signal runtime upgrades to your
	/// parachain runtime.
	pub fn give_upgrade_signal(&self, signal: UpgradeGoAhead)
	where
		<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
		T::Runtime: parachain_info::Config,
	{
		if let Some(sproof_provider) = &self.parachain_inherent_provider {
			let para_id =
				self.with_state(None, || parachain_info::Pallet::<T::Runtime>::parachain_id());
			let builder = RelayStateSproofBuilder {
				para_id,
				upgrade_go_ahead: Some(signal),
				..Default::default()
			};
			sproof_provider.lock().unwrap().update_sproof_builder(builder)
		}
	}

	/// Instructs manual seal to seal new, possibly empty blocks.
	pub async fn seal_blocks(&self, num: usize)
	where
		<<T::Block as BlockT>::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
		T::Runtime: parachain_info::Config,
	{
		let mut sink = self.manual_seal_command_sink.clone();

		for count in 0..num {
			if let Some(sproof_provider) = &self.parachain_inherent_provider {
				let para_id =
					self.with_state(None, || parachain_info::Pallet::<T::Runtime>::parachain_id());
				let builder =
					if let Some(builder) = sproof_provider.lock().unwrap().sproof_builder.take() {
						RelayStateSproofBuilder { para_id, ..builder }
					} else {
						RelayStateSproofBuilder { para_id, ..Default::default() }
					};
				sproof_provider.lock().unwrap().update_sproof_builder(builder);
			}
			let (sender, future_block) = oneshot::channel();
			let future = sink.send(EngineCommand::SealNewBlock {
				create_empty: true,
				finalize: false,
				parent_hash: None,
				sender: Some(sender),
			});

			const ERROR: &'static str = "manual-seal authorship task is shutting down";
			future.await.expect(ERROR);

			match future_block.await.expect(ERROR) {
				Ok(block) => {
					log::info!("sealed {} (hash: {}) of {} blocks", count + 1, block.hash, num)
				},
				Err(err) => {
					log::error!("failed to seal block {} of {}, error: {:?}", count + 1, num, err)
				},
			}
		}
	}

	/// so you've decided to run the test runner as a binary, use this to shutdown gracefully.
	pub async fn until_shutdown(mut self) {
		let manager = self.task_manager.take();
		if let Some(mut task_manager) = manager {
			let task = task_manager.future().fuse();
			let signal = tokio::signal::ctrl_c();
			futures::pin_mut!(signal);
			futures::future::select(task, signal).await;
		}
	}
}

impl<T: ChainInfo> Node<T> {
	/// Revert count number of blocks from the chain.
	pub fn revert_blocks(&self, count: NumberFor<T::Block>) {
		self.backend.revert(count, true).expect("Failed to revert blocks: ");
	}
}

impl<T: ChainInfo> Drop for Node<T> {
	fn drop(&mut self) {
		// Revert all blocks added since creation of the node.
		let diff = self.client.info().best_number - self.initial_block_number;
		self.revert_blocks(diff);
	}
}
