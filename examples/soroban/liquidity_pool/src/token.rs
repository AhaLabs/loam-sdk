#![allow(unused)]
use loam_sdk::soroban_sdk::{self, Address, Bytes, BytesN, Env, IntoVal, contractimport};

contractimport!(
    file = "../../../target/wasm32-unknown-unknown/release/example_token.wasm"
);

pub fn create_contract(
    e: &Env,
    token_wasm_hash: &BytesN<32>,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&token_a.clone().into());
    salt.append(&token_b.clone().into());
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash.clone())
}
