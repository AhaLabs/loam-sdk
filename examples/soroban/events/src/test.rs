#![cfg(test)]

use super::*;
use loam_sdk::soroban_sdk::{testutils::Events, vec, Env, IntoVal, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanContract__);
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
                (Symbol::short("COUNTER"), Symbol::short("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (Symbol::short("COUNTER"), Symbol::short("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (Symbol::short("COUNTER"), Symbol::short("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}
