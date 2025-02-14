#![allow(unused)]
use loam_sdk::{
    import_contract,
    soroban_sdk::{self, contractimport, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal},
};

import_contract!(example_token);

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
        .deploy_v2(token_wasm_hash.clone(), ())
}
