#![no_std]
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

pub use error::Error;
use subcontract::{Calc, Calculator};

#[loam_sdk::derive_contract(Core(Admin), Calc(Calculator))]
pub struct Contract;

mod test;