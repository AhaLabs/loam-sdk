#![no_std]
use loam_sdk::derive_contract;
use loam_sdk::soroban_sdk::{auth::Context, Address, BytesN, Vec};
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

use error::AccError;
use subcontract::{Account, AccountManager, Signature};

#[derive_contract(Core(Admin), Account(AccountManager))]
pub struct Contract;

mod test;
