#![no_std]

use loam_sdk::{
    derive_contract,
    soroban_sdk::{Address, Bytes},
};
use loam_subcontract_core::{admin::Admin, Core};

mod error;
mod token;
use crate::error::Error;
pub use token::*;

#[derive_contract(Core(Admin), TokenTrait(Token))]
pub struct Contract;

mod test;
