#![allow(unused)]
use loam_sdk::soroban_sdk::{self, contractimport, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal};

contractimport!(
    file = "../../../target/wasm32-unknown-unknown/release/example_token.wasm"
);

pub fn create_contract(
    e: &Env,
    token_wasm_hash: &BytesN<32>,
    token_a: &Address,
    token_b: &Address,
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&token_a.to_xdr(e));
    salt.append(&token_b.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash.clone())
}
