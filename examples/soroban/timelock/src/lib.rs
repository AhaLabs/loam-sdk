#![no_std]

use loam_sdk::{
    derive_contract,
    soroban_sdk::{Address, Vec},
};
use loam_subcontract_core::{admin::Admin, Core};

mod error;
mod timelock;
use error::TimelockError;
pub use timelock::*;

#[derive_contract(Core(Admin), TimelockTrait(Timelock))]
pub struct Contract;

mod test;
