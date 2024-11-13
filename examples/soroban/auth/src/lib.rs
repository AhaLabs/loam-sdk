//! This contract demonstrates how to implement authorization using
//! Soroban-managed auth framework for a simple case (a single user that needs
//! to authorize a single contract invocation).
//!
//! See `timelock` and `single_offer` examples for demonstration of performing
//! authorized token operations on behalf of the user.
//!
//! See `atomic_swap` and `atomic_multiswap` examples for demonstration of
//! multi-party authorizaton.
//!
//! See `account` example for demonstration of an acount contract with
// lib.rs

#![no_std]
use loam_sdk::{derive_contract, soroban_sdk::Address};
use loam_subcontract_core::{admin::Admin,Core};

mod subcontract;
pub use subcontract::*;

#[derive_contract(Core(Admin), Incrementable(IncrementContract))]
pub struct Contract;