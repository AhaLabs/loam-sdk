#![no_std]
use loam_sdk::soroban_sdk::Symbol;
use loam_subcontract_core::{admin::Admin, Core};

pub mod subcontract;

use subcontract::{Log, Logger};

#[loam_sdk::derive_contract(Core(Admin), Log(Logger))]
pub struct Contract;

mod test;
