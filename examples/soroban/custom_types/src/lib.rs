#![no_std]
use loam_subcontract_core::{Admin, Core};

pub mod subcontract;

use subcontract::State;

use subcontract::{Increment, IncrementContract};

#[loam_sdk::derive_contract(Core(Admin), Increment(IncrementContract))]
pub struct Contract;

mod test;