#![no_std]

use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};
use loam_sdk::soroban_sdk::{Address, BytesN};

mod liquidity_pool;
pub mod token;
pub mod error;
pub use liquidity_pool::*;
use crate::error::Error;

#[derive_contract(Core(Admin), LiquidityPoolTrait(LiquidityPool))]
pub struct Contract;

mod test;