#![no_std]
#[allow(unexpected_cfgs)]
use loam_subcontract_core::{admin::Admin, Core};

#[loam_sdk::derive_contract(Core(Admin))]
pub struct Contract;
