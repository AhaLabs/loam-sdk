#![no_std]
use loam_sdk::soroban_sdk::{Symbol, Vec};
use loam_subcontract_core::{admin::Admin, Core};

pub mod subcontract;

use subcontract::{Hello, HelloWorld};

#[loam_sdk::derive_contract(Core(Admin), HelloWorld(Hello))]
pub struct Contract;

mod test;
