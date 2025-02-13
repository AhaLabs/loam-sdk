#![cfg(test)]

use super::*;
use loam_sdk::soroban_sdk::{self, symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SorobanContract__, ());
    let client = SorobanContract__Client::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}
