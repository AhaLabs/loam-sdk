#![no_std]

use loam_sdk::{
    soroban_sdk::{self, contracttype, Bytes, BytesN, Env, RawVal, Symbol, Vec, Lazy},
    derive_contract, subcontract,
};

#[derive(Lazy, Default)]
pub struct Deployer;

#[subcontract]
pub trait IsDeployerTrait {
    fn deploy(
        &self,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<RawVal>,
    ) -> (BytesN<32>, RawVal);
}

impl IsDeployerTrait for Deployer {
    fn deploy(
        &self,
        salt: Bytes,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<RawVal>,
    ) -> (BytesN<32>, RawVal) {
        // Deploy the contract using the installed WASM code with given hash.
        let id = env()
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        
        // Invoke the init function with the given arguments.
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}