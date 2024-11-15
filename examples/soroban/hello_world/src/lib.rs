#![no_std]
use loam_subcontract_core::{admin::Admin, Core};
use loam_sdk::soroban_sdk::{Symbol, Lazy, Vec};

pub mod subcontract;

use subcontract::{HelloWorld, Hello};

#[loam_sdk::derive_contract(Core(Admin), HelloWorld(Hello))]
pub struct Contract;

mod test;