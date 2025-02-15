#![no_std]

use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

mod contract;
pub use contract::*;

#[derive_contract(Core(Admin), DeployerContract(DeployerContractTrait))]
pub struct Contract;