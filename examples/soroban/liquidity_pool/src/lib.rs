#![no_std]

use loam_sdk::derive_contract;
use loam_sdk::soroban_sdk::{Address, BytesN};
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
mod liquidity_pool;
pub mod token;
use crate::error::Error;
pub use liquidity_pool::*;

#[derive_contract(Core(Admin), LiquidityPoolTrait(Storage))]
pub struct Contract;

mod test;
