#![cfg(test)]

use crate::codegen::{parachain, parachain::api::balances::events::Transfer};
use anyhow::anyhow;
use sp_core::{crypto::Ss58Codec, Bytes};
use sp_keyring::sr25519::Keyring;
use subxt::{
	rpc_params, tx::SubmittableExtrinsic, utils::AccountId32, OnlineClient, SubstrateConfig,
};

#[tokio::test]
async fn simple_transfer() -> Result<(), anyhow::Error> {
	let client = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;

	let bob = AccountId32::from(Keyring::Bob.public());
	let alice_account = AccountId32::from(Keyring::Alice.public());

	let addr = parachain::api::storage().system().account(alice_account.clone());
	let old = client
		.storage()
		.at_latest()
		.await?
		.fetch(&addr)
		.await?
		.expect("Account should exist")
		.data
		.free;

	let call = client
		.tx()
		.call_data(&parachain::api::tx().balances().transfer(bob.clone().into(), old / 2))?;

	let extrinsic: Bytes = client
		.rpc()
		.request(
			"simnode_authorExtrinsic",
			// author an extrinsic from alice
			rpc_params![Bytes::from(call), Keyring::Alice.to_account_id().to_ss58check()],
		)
		.await?;

	let submittable = SubmittableExtrinsic::from_bytes(client.clone(), extrinsic.0);
	let events = submittable.submit_and_watch().await?.wait_for_finalized_success().await?;
	let transfer = events
		.find::<Transfer>()
		.collect::<Result<Vec<_>, subxt::Error>>()?
		.pop()
		.ok_or_else(|| anyhow!("transfer event not found!"))?;

	// assert that the event was emitted
	assert_eq!(
		transfer,
		Transfer { from: alice_account.clone(), to: bob.clone(), amount: old / 2 }
	);

	// confirm that state has changed
	let addr = parachain::api::storage().system().account(&alice_account);
	let new = client
		.storage()
		.at_latest()
		.await?
		.fetch(&addr)
		.await?
		.expect("Account should exist")
		.data
		.free;

	assert!(new <= old / 2);

	Ok(())
}

// todo: runtime_upgrades::{parachain, standalone}

// todo: standalone
