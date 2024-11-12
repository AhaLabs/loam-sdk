#![no_std]
use loam_sdk::derive_contract;

pub mod subcontract;

use subcontract::{Account, AccountManager};

#[derive_contract(
    Core(Admin),
    Account(AccountManager),
)]
pub struct Contract;


mod test;