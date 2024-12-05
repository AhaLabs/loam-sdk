#![no_std]

use loam_sdk::{soroban_sdk::{Address, Vec}, derive_contract};
use loam_subcontract_core::{admin::Admin, Core};

mod error;
mod timelock;
pub use timelock::*;
use error::TimelockError;

#[derive_contract(Core(Admin), TimelockTrait(Timelock))]
pub struct Contract;

mod test;
