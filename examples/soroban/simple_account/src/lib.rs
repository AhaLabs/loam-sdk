//! This a minimal exapmle of an account contract.
//!
//! The account is owned by a single ed25519 public key that is also used for
//! authentication.
//!
//! For a more advanced example that demonstrates all the capabilities of the
// examples/soroban/simple_account/src/lib.rs
#![no_std]
use loam_sdk::derive_contract;
use loam_sdk::soroban_sdk::{auth::Context, BytesN, Vec};
use loam_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

use error::SimpleAccError;
use subcontract::{SimpleAccount, SimpleAccountManager};

#[derive_contract(Core(Admin), SimpleAccount(SimpleAccountManager))]
pub struct Contract;

mod test;
