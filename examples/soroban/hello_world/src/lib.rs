// examples/soroban/hello_world/src/lib.rs
#![no_std]
use loam_subcontract_core::{admin::Admin, Core};

pub mod subcontract;

use subcontract::HelloWorld;

#[loam_sdk::derive_contract(Core(Admin), HelloWorld)]
pub struct Contract;

mod test;