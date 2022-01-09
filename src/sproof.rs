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

//! Parachain inherent data provider, useful for signalling relay chain authorizations to
//! parachain simnodes.

use codec::Encode;
use num_traits::AsPrimitive;
use parachain_inherent::ParachainInherentData;
use polkadot_primitives::v1::PersistedValidationData;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block, Header},
};
use sproof_builder::RelayStateSproofBuilder;
use std::{marker::PhantomData, sync::Arc};

/// Provides the inherent for parachain runtimes. Can also be manipulated to send relay chain
/// signals to simulated node runtime.
pub struct ParachainInherentSproofProvider<B, C> {
	// client type
	client: Arc<C>,
	// sproof builder
	sproof_builder: Option<RelayStateSproofBuilder>,
	_phantom: PhantomData<B>,
}

impl<B, C> ParachainInherentSproofProvider<B, C>
where
	B: Block,
	C: HeaderBackend<B>,
	<B::Header as Header>::Number: num_traits::cast::AsPrimitive<u32>,
{
	/// Construct a new sproof-er
	pub fn new(client: Arc<C>) -> Self {
		ParachainInherentSproofProvider { client, sproof_builder: None, _phantom: PhantomData }
	}

	/// updates the sproof to a new state
	pub fn update_sproof_builder(&mut self, sproof: RelayStateSproofBuilder) {
		self.sproof_builder = Some(sproof);
	}

	/// Given the current slot, create the inherent.
	pub fn create_inherent(&mut self, slot: u64) -> ParachainInherentData {
		let mut sproof = self.sproof_builder.take().unwrap_or_default();
		sproof.current_slot = slot.into();
		sproof.host_config.validation_upgrade_delay = 1;

		let info = self.client.info();
		let header = self
			.client
			.header(BlockId::Hash(info.best_hash))
			.expect("Failed to create inherent; panic!")
			.expect("Failed to create inherent; panic!")
			.encode();

		let (state_root, proof) = sproof.into_state_root_and_proof();

		ParachainInherentData {
			validation_data: PersistedValidationData {
				parent_head: header.into(),
				relay_parent_number: info.best_number.as_() * 100,
				relay_parent_storage_root: state_root,
				max_pov_size: 15 * 1024 * 1024,
			},
			relay_chain_state: proof,
			downward_messages: Default::default(),
			horizontal_messages: Default::default(),
		}
	}
}
