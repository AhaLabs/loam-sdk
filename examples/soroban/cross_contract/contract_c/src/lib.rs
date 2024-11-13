#![no_std]
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

pub use error::Error;
use subcontract::{ContractC, IsContractC};

loam_sdk::import_contract!(contract_a);

#[loam_sdk::derive_contract(Core(Admin), ContractC(ContractC))]
pub struct Contract;

mod test;
