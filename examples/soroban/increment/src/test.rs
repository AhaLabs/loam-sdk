#![cfg(test)]

use loam_sdk::soroban_sdk::{testutils::Logs, Env};

use crate::{SorobanContract__, SorobanContract__Client};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SorobanContract__, ());
    let client = SorobanContract__Client::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    std::println!("{}", env.logs().all().join("\n"));
}
