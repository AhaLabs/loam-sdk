use loam_sdk::{
    soroban_sdk::{self, Address, Env, Lazy},
    subcontract,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct ContractC;

#[subcontract]
pub trait IsContractC {
    fn add_with(&self, env: Env, contract_id: Address, x: u32, y: u32) -> Result<u32, Error>;
}

impl IsContractC for ContractC {
    fn add_with(&self, env: Env, contract_id: Address, x: u32, y: u32) -> Result<u32, Error> {
        let client = contract_a::Client::new(&env, &contract_id);
        let client_2 = contract_a::Client::new(&env, &contract_id);
        
        let result1 = client.add(&x, &y);
        let result2 = client_2.add(&x, &y);
        
        result1.checked_add(result2)
            .and_then(|sum| sum.checked_shr(2))
            .ok_or(Error::Overflow)
    }
}
