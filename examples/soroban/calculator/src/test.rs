#![cfg(test)]
extern crate std;

use loam_sdk::soroban_sdk::{self, vec, Env};

use crate::{Error, SorobanContract__, SorobanContract__Client};

#[test]
fn test_calculator() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register(SorobanContract__, ());
    let calculator = SorobanContract__Client::new(&env, &contract_id);

    // Test the `add_u32` function
    let result = calculator.add_u32(&3, &4);
    // FIXME: should be returning a result
    assert_eq!(result, 7);

    // Test addition overflow
    // FIXME - error type not properly being returned
    let result = calculator.try_add_u32(&u32::MAX, &1);
    let Err(Ok(err)) = result else {
        panic!("Expected an error, got {:?}", result);
    };
    assert_eq!(err, Error::Overflow);

    // Test the `two_array` function
    let expected_array: soroban_sdk::Vec<u32> = vec![&env, 3, 4];
    let result = calculator.two_array(&3, &4);
    assert_eq!(result, expected_array);
}
