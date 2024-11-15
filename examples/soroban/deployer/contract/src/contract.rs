#![no_std]

use loam_sdk::{
    soroban_sdk::{self, contracttype, Env, IntoKey, Symbol},
    subcontract,
};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct DeployerContractTrait {
    value: u32,
}

#[subcontract]
pub trait IsDeployerContract {
    fn init(&mut self, value: u32);
    fn value(&self) -> u32;
}

impl IsDeployerContract for DeployerContractTrait {
    fn init(&mut self, value: u32) {
        self.value = value;
    }

    fn value(&self) -> u32 {
        self.value
    }
}
