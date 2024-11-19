#![cfg(test)]

use super::{SorobanContract__, SorobanContract__Client};
use loam_sdk::soroban_sdk::Env;

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanContract__);
    let client = SorobanContract__Client::new(&env, &contract_id);
    assert_eq!(client.sum(&1), 0);
    assert_eq!(client.sum(&2), 1);
    assert_eq!(client.sum(&5), 10);

}
