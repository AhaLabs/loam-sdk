#![no_std]
use loam_subcontract_core::Core;

pub mod subcontract;

use subcontract::{IsIncrement, Increment, IncrementContract};

#[loam_sdk::derive_contract(Core, Increment(IncrementContract))]
pub struct Contract;

mod test;