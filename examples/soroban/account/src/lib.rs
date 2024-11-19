#![no_std]
use loam_sdk::derive_contract;
use loam_sdk::soroban_sdk::{Address, Vec, BytesN, auth::Context};
use loam_subcontract_core::{admin::Admin,Core};

pub mod subcontract;
pub mod error;

use subcontract::{Account, AccountManager, Signature};
use error::AccError;

#[derive_contract(
    Core(Admin),
    Account(AccountManager),
)]
pub struct Contract;


mod test;