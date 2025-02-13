#![no_std]
use loam_subcontract_core::{Admin, Core};

pub mod subcontract;

use subcontract::State;

use subcontract::{Incrementable, Inc};

#[loam_sdk::derive_contract(Core(Admin), Incrementable(Inc))]
pub struct Contract;

mod test;
