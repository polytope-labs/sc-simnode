#![cfg(test)]

use crate::codegen::{
	api,
	api::{
		parachain_system::events::{ValidationFunctionApplied, ValidationFunctionStored},
		runtime_types::{
			frame_system::pallet::Call, parachain_template_runtime::RuntimeCall,
			sp_weights::weight_v2::Weight,
		},
	},
};
use anyhow::anyhow;
use manual_seal::CreatedBlock;
use sp_core::{crypto::Ss58Codec, Bytes, H256};
use sp_keyring::sr25519::Keyring;
use subxt::{
	rpc_params, tx::SubmittableExtrinsic, OnlineClient, SubstrateConfig,
};

#[tokio::test]
async fn test_all_features() -> Result<(), anyhow::Error> {
	runtime_upgrades().await?;
	Ok(())
}

async fn runtime_upgrades() -> Result<(), anyhow::Error> {
	let client = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

	let old_version = client.rpc().runtime_version(None).await?;
	assert_eq!(old_version.spec_version, 1);

	let code = include_bytes!("../../../assets/parachain-runtime-upgrade.wasm").to_vec();

	let call = client.tx().call_data(&api::tx().sudo().sudo_unchecked_weight(
		RuntimeCall::System(Call::set_code { code }),
		Weight { ref_time: 0, proof_size: 0 },
	))?;

	let extrinsic: Bytes = client
		.rpc()
		.request(
			"simnode_authorExtrinsic",
			// author an extrinsic from the sudo account.
			rpc_params![Bytes::from(call), Keyring::Alice.to_account_id().to_ss58check()],
		)
		.await?;
	let submittable = SubmittableExtrinsic::from_bytes(client.clone(), extrinsic.0);
	let events = submittable.submit_and_watch().await?.wait_for_finalized_success().await?;
	// assert that the event was stored
	events
		.find::<ValidationFunctionStored>()
		.collect::<Result<Vec<_>, subxt::Error>>()?
		.pop()
		.ok_or_else(|| anyhow!("transfer event not found!"))?;

	client.rpc().request::<()>("simnode_upgradeSignal", rpc_params![true]).await?;

	let events = client.events().at_latest().await?;

	// assert that the event was stored
	events
		.find::<ValidationFunctionApplied>()
		.collect::<Result<Vec<_>, subxt::Error>>()?
		.pop()
		.ok_or_else(|| anyhow!("transfer event not found!"))?;

	// assert the version
	let new_version = client.rpc().runtime_version(None).await?;
	assert_eq!(new_version.spec_version, 100);

	// try to create 10 blocks to assert that the runtime still works.
	for _ in 0..10 {
		client
			.rpc()
			.request::<CreatedBlock<H256>>("engine_createBlock", rpc_params![true, true])
			.await?;
	}

	Ok(())
}
