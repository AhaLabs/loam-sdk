#![no_std]
/// This is a simple contract that just extends TTL for its keys.
/// It's main purpose is to demonstrate how TTL extension can be tested,
/// so the most interesting part here is `test.rs`.
use loam_subcontract_core::{admin::Admin, Core};

pub mod subcontract;

use subcontract::{Ttl, TtlContract};

#[loam_sdk::derive_contract(Core(Admin), Ttl(TtlContract))]
pub struct Contract;

mod test;