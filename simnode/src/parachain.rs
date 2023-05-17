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

//! Parachain [`SelectChain`] implementation

use async_trait::async_trait;
use sp_blockchain::HeaderBackend;
use sp_consensus::{Error, SelectChain};
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

/// Since parachains don't have a select chain implementation, use this one instead.
pub struct ParachainSelectChain<Client> {
	client: Arc<Client>,
}

impl<C> Clone for ParachainSelectChain<C> {
	fn clone(&self) -> Self {
		Self { client: self.client.clone() }
	}
}

impl<C> ParachainSelectChain<C> {
	/// Initialize the parachain [`SelectChain`]
	pub fn new(client: Arc<C>) -> Self {
		Self { client }
	}
}

#[async_trait]
impl<B, C> SelectChain<B> for ParachainSelectChain<C>
where
	B: BlockT,
	C: HeaderBackend<B>,
{
	async fn leaves(&self) -> Result<Vec<B::Hash>, Error> {
		Ok(vec![])
	}

	async fn best_chain(&self) -> Result<B::Header, Error> {
		let header = self
			.client
			.header(self.client.info().best_hash)
			.map_err(|e| Error::Other(Box::new(e)))?
			.ok_or_else(|| Error::StateUnavailable(format!("Header not found!")))?;
		Ok(header)
	}
}
