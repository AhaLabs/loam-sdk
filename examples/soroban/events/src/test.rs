#![cfg(test)]

use super::*;
use loam_sdk::soroban_sdk::{self, testutils::Events, vec, Env, IntoVal, symbol_short};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SorobanContract__, ());
    let client = SorobanContract__Client::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}
