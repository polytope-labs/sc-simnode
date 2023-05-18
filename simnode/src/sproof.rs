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

//! Parachain inherent data provider, useful for signalling relay chain authorizations to
//! parachain simnodes.

use crate::{client::FullClientFor, ChainInfo};
use codec::Encode;
use num_traits::AsPrimitive;
use parachain_inherent::ParachainInherentData;
use polkadot_primitives::v2::PersistedValidationData;
use sp_api::BlockT;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Header;
use sproof_builder::RelayStateSproofBuilder;
use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

/// Provides the inherent for parachain runtimes. Can also be manipulated to send relay chain
/// signals to simulated node runtime.
pub struct ParachainSproofInherentProvider<T: ChainInfo> {
	// client type
	client: Arc<FullClientFor<T>>,
	// sproof builder
	sproof_builder: Option<RelayStateSproofBuilder>,
	// phantom type
	_phantom: PhantomData<T>,
}

/// A thread safe parachain sproof inherent provider
pub type SharedParachainSproofInherentProvider<T> = Arc<Mutex<ParachainSproofInherentProvider<T>>>;

impl<T> ParachainSproofInherentProvider<T>
where
	T: ChainInfo,
	<<T::Block as BlockT>::Header as Header>::Number: AsPrimitive<u32>,
{
	/// Construct a new sproof-er
	pub fn new(client: Arc<FullClientFor<T>>) -> Self {
		ParachainSproofInherentProvider { client, sproof_builder: None, _phantom: PhantomData }
	}

	/// updates the sproof to a new state
	pub fn update_sproof_builder(&mut self, sproof: RelayStateSproofBuilder) {
		self.sproof_builder = Some(sproof);
	}

	/// Given the current slot, create the inherent.
	pub fn create_inherent(&mut self, slot: u64) -> ParachainInherentData {
		let mut sproof = self.sproof_builder.take().unwrap_or_default();
		sproof.current_slot = slot.into();
		sproof.host_config.validation_upgrade_delay = 2;
		sproof.host_config.max_code_size = 15 * 1024 * 1024;
		// this makes every block random, so that you can still author blocks after reverting.
		// instead of getting the AlreadyInChain error.
		sproof.randomness = rand::random();

		let info = self.client.info();
		let header = self
			.client
			.header(info.best_hash)
			.expect("Failed to create inherent; panic!")
			.expect("Failed to create inherent; panic!")
			.encode();

		let (state_root, proof) = sproof.into_state_root_and_proof();

		ParachainInherentData {
			validation_data: PersistedValidationData {
				parent_head: header.into(),
				relay_parent_number: info.best_number.as_() + 100,
				relay_parent_storage_root: state_root,
				max_pov_size: 15 * 1024 * 1024,
			},
			relay_chain_state: proof,
			downward_messages: Default::default(),
			horizontal_messages: Default::default(),
		}
	}
}
