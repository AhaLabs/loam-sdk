#![no_std]

use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};
use loam_sdk::soroban_sdk::{Address, BytesN};

mod liquidity_pool;
pub mod token;
pub use liquidity_pool::*;

#[derive_contract(Core(Admin), LiquidityPoolTrait(LiquidityPool))]
pub struct Contract;
