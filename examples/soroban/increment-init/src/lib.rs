#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

mod counter;
pub use counter::*;

#[derive_contract(Core(Admin), Countable(Counter))]
pub struct Contract;

#[cfg(test)]
mod test;
