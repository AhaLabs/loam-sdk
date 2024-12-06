#![no_std]

use loam_sdk::derive_contract;
use loam_subcontract_core::{Admin, Core};

mod counter;
pub use counter::*;

#[derive_contract(Core(Admin), Incrementable(Counter))]
pub struct Contract;

mod test;
