#![cfg(test)]

use super::*;
use loam_sdk::soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SorobanContract__, ());
    let client = SorobanContract__Client::new(&env, &contract_id);

    assert_eq!(client.increment(&1), 1);
    assert_eq!(client.increment(&10), 11);
    assert_eq!(
        client.get_state(),
        State {
            count: 11,
            last_incr: 10
        }
    );
}
