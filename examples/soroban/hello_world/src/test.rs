#![cfg(test)]

use super::*;
use loam_sdk::soroban_sdk::{vec, Env, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanContract__);
    let client = SorobanContract__Client::new(&env, &contract_id);

    let words = client.hello(&Symbol::short("Dev"));
    assert_eq!(
        words,
        vec![&env, Symbol::short("Hello"), Symbol::short("Dev"),]
    );
}
