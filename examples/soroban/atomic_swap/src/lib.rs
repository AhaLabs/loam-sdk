//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]
use loam_sdk::soroban_sdk::Address;
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

pub use error::Error;
use subcontract::{AtomicSwap, AtomicSwapContract};

#[loam_sdk::derive_contract(Core(Admin), AtomicSwap(AtomicSwapContract))]
pub struct Contract;

mod test;
