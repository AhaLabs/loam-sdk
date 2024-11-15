#![no_std]

use loam_sdk::{soroban_sdk::{Address, Bytes}, derive_contract};
use loam_subcontract_core::{admin::Admin, Core};

mod token;
pub use token::*;

#[derive_contract(Core(Admin), TokenTrait(Token))]
pub struct Contract;

mod test;