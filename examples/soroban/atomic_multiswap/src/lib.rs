//! This contract performs a batch of atomic token swaps between multiple
//! parties and does a simple price matching.
//! Parties don't need to know each other and also don't need to know their
//! signatures are used in this contract; they sign the `AtomicSwap` contract
//! invocation that guarantees that their token will be swapped with someone
//! while following the price limit.
//! This example demonstrates how authorized calls can be batched together.
#![no_std]
use loam_sdk::{import_contract, soroban_sdk::{Address, Vec}};
use loam_subcontract_core::{admin::Admin, Core};

pub mod subcontract;

import_contract!(example_atomic_swap);

use subcontract::{AtomicMultiSwap, AtomicMultiSwapContract, SwapSpec};

#[loam_sdk::derive_contract(Core(Admin), AtomicMultiSwap(AtomicMultiSwapContract))]
pub struct Contract;

mod test;
