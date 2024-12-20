//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

use polkadot_sdk::*;

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
