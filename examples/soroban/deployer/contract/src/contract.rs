#![no_std]

use loam_sdk::{
    soroban_sdk::{self, Env, IntoKey, Symbol, PersistentItem},
    subcontract, loamstorage,
};

#[loamstorage]
pub struct DeployerContractTrait {
    value: PersistentItem<u32>,
}

#[subcontract]
pub trait IsDeployerContract {
    fn init(&mut self, value: u32);
    fn value(&self) -> u32;
}

impl IsDeployerContract for DeployerContractTrait {
    fn init(&mut self, value: u32) {
        self.value.set(&value);
    }

    fn value(&self) -> u32 {
        self.value.get().unwrap_or_default()
    }
}
