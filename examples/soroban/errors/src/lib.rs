#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

pub mod counter;
pub mod error;

use counter::{Impl, Riff};

#[derive_contract(Core(Admin), Riff(Impl))]
pub struct Contract;

mod test;
