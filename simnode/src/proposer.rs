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

//! Fork-aware proposer factory for simnode.
//!
//! This module provides a custom proposer factory that properly handles building blocks
//! on non-best-chain parents (fork scenarios). The standard `sc_basic_authorship::ProposerFactory`
//! uses `pool.ready_at(parent)` which doesn't return transactions as "ready" for non-best parents
//! because the transaction pool validates against the best block.
//!
//! This factory detects when we're building on a fork and includes pending transactions directly.

use polkadot_sdk::*;

use codec::Encode;
use futures::future::{self, Future, FutureExt};
use log::{debug, info, trace, warn};
use sc_block_builder::{BlockBuilderApi, BlockBuilderBuilder};
use sc_transaction_pool_api::{InPoolTransaction, TransactionPool};
use sp_api::{ApiExt, CallApiAt, ProvideRuntimeApi};
use sp_blockchain::{ApplyExtrinsicFailed::Validity, Error::ApplyExtrinsicFailed, HeaderBackend};
use sp_consensus::{DisableProofRecording, Proposal};
use sp_core::traits::SpawnNamed;
use sp_inherents::InherentData;
use sp_runtime::{
	traits::{Block as BlockT, Header as HeaderT},
	Digest, ExtrinsicInclusionMode,
};
use std::{marker::PhantomData, pin::Pin, sync::Arc, time};

const LOG_TARGET: &str = "fork-aware-proposer";

/// Fork-aware proposer factory that properly handles building blocks on non-best-chain parents.
///
/// When building on the best chain, this delegates to the standard `sc_basic_authorship::ProposerFactory`.
/// When building on a fork (non-best parent), it creates a custom proposer that includes
/// pending transactions directly without relying on `pool.ready_at()`.
pub struct ForkAwareProposerFactory<A, C> {
	spawn_handle: Box<dyn SpawnNamed>,
	client: Arc<C>,
	transaction_pool: Arc<A>,
	default_block_size_limit: usize,
}

impl<A, C> Clone for ForkAwareProposerFactory<A, C> {
	fn clone(&self) -> Self {
		Self {
			spawn_handle: self.spawn_handle.clone(),
			client: self.client.clone(),
			transaction_pool: self.transaction_pool.clone(),
			default_block_size_limit: self.default_block_size_limit,
		}
	}
}

impl<A, C> ForkAwareProposerFactory<A, C> {
	/// Create a new fork-aware proposer factory.
	pub fn new(
		spawn_handle: impl SpawnNamed + 'static,
		client: Arc<C>,
		transaction_pool: Arc<A>,
	) -> Self {
		Self {
			spawn_handle: Box::new(spawn_handle),
			client,
			transaction_pool,
			default_block_size_limit: 4 * 1024 * 1024 + 512,
		}
	}
}

impl<Block, C, A> ForkAwareProposerFactory<A, C>
where
	A: TransactionPool<Block = Block> + 'static,
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + Send + Sync + 'static,
	C::Api: ApiExt<Block> + BlockBuilderApi<Block>,
{
	fn init_with_now(
		&mut self,
		parent_header: &<Block as BlockT>::Header,
		now: Box<dyn Fn() -> time::Instant + Send + Sync>,
	) -> ForkAwareProposer<Block, C, A> {
		let parent_hash = parent_header.hash();
		let best_hash = self.client.info().best_hash;
		let is_fork = parent_hash != best_hash;

		info!(
			target: LOG_TARGET,
			"🙌 Starting consensus session on top of parent {:?} (#{}), is_fork: {}",
			parent_hash,
			parent_header.number(),
			is_fork
		);

		ForkAwareProposer {
			spawn_handle: self.spawn_handle.clone(),
			client: self.client.clone(),
			parent_hash,
			parent_number: *parent_header.number(),
			transaction_pool: self.transaction_pool.clone(),
			now,
			default_block_size_limit: self.default_block_size_limit,
			is_fork,
			_phantom: PhantomData,
		}
	}
}

impl<A, Block, C> sp_consensus::Environment<Block> for ForkAwareProposerFactory<A, C>
where
	A: TransactionPool<Block = Block> + 'static,
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + CallApiAt<Block> + Send + Sync + 'static,
	C::Api: ApiExt<Block> + BlockBuilderApi<Block>,
{
	type CreateProposer = future::Ready<Result<Self::Proposer, Self::Error>>;
	type Proposer = ForkAwareProposer<Block, C, A>;
	type Error = sp_blockchain::Error;

	fn init(&mut self, parent_header: &<Block as BlockT>::Header) -> Self::CreateProposer {
		future::ready(Ok(self.init_with_now(parent_header, Box::new(time::Instant::now))))
	}
}

/// Fork-aware proposer that handles both best-chain and fork scenarios.
pub struct ForkAwareProposer<Block: BlockT, C, A: TransactionPool> {
	#[allow(dead_code)]
	spawn_handle: Box<dyn SpawnNamed>,
	client: Arc<C>,
	parent_hash: Block::Hash,
	parent_number: <<Block as BlockT>::Header as HeaderT>::Number,
	transaction_pool: Arc<A>,
	now: Box<dyn Fn() -> time::Instant + Send + Sync>,
	default_block_size_limit: usize,
	is_fork: bool,
	_phantom: PhantomData<DisableProofRecording>,
}

impl<A, Block, C> sp_consensus::Proposer<Block> for ForkAwareProposer<Block, C, A>
where
	A: TransactionPool<Block = Block> + 'static,
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + CallApiAt<Block> + Send + Sync + 'static,
	C::Api: ApiExt<Block> + BlockBuilderApi<Block>,
{
	type Proposal = Pin<Box<dyn Future<Output = Result<Proposal<Block, ()>, Self::Error>> + Send>>;
	type Error = sp_blockchain::Error;
	type ProofRecording = DisableProofRecording;
	type Proof = ();

	fn propose(
		self,
		inherent_data: InherentData,
		inherent_digests: Digest,
		max_duration: time::Duration,
		block_size_limit: Option<usize>,
	) -> Self::Proposal {
		self.propose_with(inherent_data, inherent_digests, max_duration, block_size_limit)
			.boxed()
	}
}

impl<A, Block, C> ForkAwareProposer<Block, C, A>
where
	A: TransactionPool<Block = Block> + 'static,
	Block: BlockT,
	C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + CallApiAt<Block> + Send + Sync + 'static,
	C::Api: ApiExt<Block> + BlockBuilderApi<Block>,
{
	async fn propose_with(
		self,
		inherent_data: InherentData,
		inherent_digests: Digest,
		max_duration: time::Duration,
		block_size_limit: Option<usize>,
	) -> Result<Proposal<Block, ()>, sp_blockchain::Error> {
		let deadline = (self.now)() + max_duration - max_duration / 10;
		let block_timer = time::Instant::now();

		let mut block_builder = BlockBuilderBuilder::new(&*self.client)
			.on_parent_block(self.parent_hash)
			.with_parent_block_number(self.parent_number)
			.with_proof_recorder(None)
			.with_inherent_digests(inherent_digests)
			.build()?;

		// Apply inherents
		let inherents = block_builder.create_inherents(inherent_data)?;
		for inherent in inherents {
			match block_builder.push(inherent) {
				Err(ApplyExtrinsicFailed(Validity(e))) if e.exhausted_resources() => {
					warn!(
						target: LOG_TARGET,
						"⚠️  Dropping non-mandatory inherent from overweight block."
					)
				},
				Err(ApplyExtrinsicFailed(Validity(e))) if e.was_mandatory() => {
					return Err(ApplyExtrinsicFailed(Validity(e)))
				},
				Err(e) => {
					warn!(
						target: LOG_TARGET,
						"❗️ Inherent extrinsic returned unexpected error: {}. Dropping.", e
					);
				},
				Ok(_) => {},
			}
		}

		let mode = block_builder.extrinsic_inclusion_mode();
		if mode == ExtrinsicInclusionMode::AllExtrinsics {
			self.apply_extrinsics(&mut block_builder, deadline, block_size_limit).await?;
		}

		let (block, storage_changes, _proof) = block_builder.build()?.into_inner();

		info!(
			target: LOG_TARGET,
			"🎁 Prepared block for proposing at {} ({} ms) hash: {:?}; parent_hash: {}; extrinsics_count: {}; is_fork: {}",
			block.header().number(),
			block_timer.elapsed().as_millis(),
			<Block as BlockT>::Hash::from(block.header().hash()),
			block.header().parent_hash(),
			block.extrinsics().len(),
			self.is_fork
		);

		Ok(Proposal { block, proof: (), storage_changes })
	}

	async fn apply_extrinsics(
		&self,
		block_builder: &mut sc_block_builder::BlockBuilder<'_, Block, C>,
		deadline: time::Instant,
		block_size_limit: Option<usize>,
	) -> Result<(), sp_blockchain::Error> {
		let block_size_limit = block_size_limit.unwrap_or(self.default_block_size_limit);

		// Always use pool.ready() to get all pending transactions.
		// For fork scenarios (is_fork = true), the standard ready_at_with_timeout may not
		// return transactions properly because they were validated against a different parent.
		// For best-chain scenarios (is_fork = false), pool.ready() works the same.
		info!(
			target: LOG_TARGET,
			"📦 Applying extrinsics: is_fork={}, parent_hash={:?}",
			self.is_fork,
			self.parent_hash
		);

		// Get all ready transactions from the pool
		let pending: Vec<_> = self.transaction_pool.ready().collect();

		info!(
			target: LOG_TARGET,
			"📦 Found {} pending transactions to try",
			pending.len()
		);

		for pending_tx in pending {
			let now = (self.now)();
			if now > deadline {
				debug!(
					target: LOG_TARGET,
					"Deadline reached, stopping transaction inclusion"
				);
				break;
			}

			let pending_tx_data = (**pending_tx.data()).clone();
			let pending_tx_hash = pending_tx.hash().clone();

			let block_size = block_builder.estimate_block_size(false);
			if block_size + pending_tx_data.encoded_size() > block_size_limit {
				debug!(
					target: LOG_TARGET,
					"[{:?}] Transaction would overflow block size limit, skipping",
					pending_tx_hash
				);
				continue;
			}

			trace!(
				target: LOG_TARGET,
				"[{:?}] Trying to push transaction to block",
				pending_tx_hash
			);

			match sc_block_builder::BlockBuilder::push(block_builder, pending_tx_data) {
				Ok(()) => {
					info!(
						target: LOG_TARGET,
						"✅ [{:?}] Successfully pushed transaction to block",
						pending_tx_hash
					);
				},
				Err(ApplyExtrinsicFailed(Validity(e))) if e.exhausted_resources() => {
					info!(
						target: LOG_TARGET,
						"⚠️ [{:?}] Transaction exhausted resources, skipping",
						pending_tx_hash
					);
				},
				Err(e) => {
					info!(
						target: LOG_TARGET,
						"❌ [{:?}] Transaction invalid: {}",
						pending_tx_hash,
						e
					);
				},
			}
		}

		Ok(())
	}
}
