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

//! Simnode Cli Utilities

use sc_cli::{CliConfiguration, SharedParams};

/// The `simnode` sub-command.
///
/// See [`Command`] for more info.
#[derive(Debug, Clone, clap::Parser)]
pub struct SimnodeCli {
	/// Shared params need by substrate
	#[clap(flatten)]
	shared: SharedParams,
}

impl CliConfiguration for SimnodeCli {
	fn shared_params(&self) -> &SharedParams {
		&self.shared
	}
}
