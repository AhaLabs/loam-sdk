#![no_std]
use loam_sdk::soroban_sdk;
use loam_sdk_core_riff::{admin::Admin, Core};
use registry::{
    contract::ContractRegistry, wasm::WasmRegistry, Claimable, Deployable, DevDeployable,
    Publishable,
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
    Publishable(WasmRegistry),
    Deployable(ContractRegistry),
    Claimable(ContractRegistry),
    DevDeployable(ContractRegistry)
)]
pub struct Contract;

#[cfg(test)]
mod test;
