#![no_std]
use loam_sdk::soroban_sdk;
use loam_subcontract_core::{admin::Admin, Core};

use metadata::PublishedWasm;
use registry::{
    contract::Contract as Contract_, Claimable, Deployable, DevDeployable, Publishable
};

pub mod error;
pub mod metadata;
pub mod registry;
pub mod util;
pub mod version;

use error::Error;
use version::Version;

#[loam_sdk::derive_contract(
    Core(Admin),
    Publishable(PublishedWasm),
    Deployable(Contract_),
    Claimable(Contract_),
    DevDeployable(Contract_)
)]
pub struct Contract;

#[cfg(test)]
mod test;
