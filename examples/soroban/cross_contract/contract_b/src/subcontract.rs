use loam_sdk::{
    soroban_sdk::{self, Address, Env, Lazy},
    subcontract,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct ContractB;

#[subcontract]
pub trait IsContractB {
    fn add_with(&self, env: Env, contract_id: Address, x: u32, y: u32) -> Result<u32, Error>;
}

impl IsContractB for ContractB {
    fn add_with(&self, env: Env, contract_id: Address, x: u32, y: u32) -> Result<u32, Error> {
        let client = contract_a::Client::new(&env, &contract_id);
        Ok(client.add(&x, &y))
    }
}
