//! This a minimal exapmle of an account contract.
//!
//! The account is owned by a single ed25519 public key that is also used for
//! authentication.
//!
//! For a more advanced example that demonstrates all the capabilities of the
// examples/soroban/simple_account/src/lib.rs
#![no_std]
use loam_sdk::derive_contract;

pub mod subcontract;
pub mod error;

use subcontract::SimpleAccountManager;

#[derive_contract(
    Core(Admin),
    Account(SimpleAccountManager),
)]
pub struct Contract;

mod test;