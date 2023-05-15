use crate::{ChainInfo, FullClientFor};
use jsonrpsee::{
	core::{Error as RpcError, RpcResult as Result},
	proc_macros::rpc,
};
use simnode_runtime_api::CreateTransactionApi;
use sp_api::{ConstructRuntimeApi, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use std::sync::Arc;

/// Simnode RPC methods.
#[rpc(client, server)]
pub trait SimnodeApi {
	/// Constructs an extrinsic using simnode's runtime api.
	#[method(name = "simnode_authorExtrinsic")]
	fn author_extrinsic(&self, call: Bytes, account: Bytes) -> Result<Bytes>;
}

/// Handler implementation for Simnode RPC API.
pub struct SimnodeRpcHandler<T: ChainInfo> {
	client: Arc<FullClientFor<T>>,
}

impl<T: ChainInfo> SimnodeRpcHandler<T> {
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
{
	fn author_extrinsic(&self, call: Bytes, account: Bytes) -> Result<Bytes> {
		let at = self.client.info().best_hash;
		let call = codec::Decode::decode(&mut &call.0[..])
			.map_err(|e| RpcError::Custom(format!("failed to decode call: {e:?}")))?;
		let account = codec::Decode::decode(&mut &account.0[..])
			.map_err(|e| RpcError::Custom(format!("failed to decode account: {e:?}")))?;
		let extrinsic =
			self.client.runtime_api().create_transaction(at, account, call).map_err(|e| {
				RpcError::Custom(format!("CreateTransactionApi is unimplemented: {e:?}"))
			})?;

		Ok(extrinsic.into())
	}
}
