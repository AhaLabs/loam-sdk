#![no_std]

use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

mod deployer;
pub use deployer::*;

#[derive_contract(IsDeployerTrait(Deployer))]
pub struct Contract;

#[cfg(test)]
mod test;
