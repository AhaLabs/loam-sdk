#![cfg(test)]
extern crate std;

use loam_sdk::soroban_sdk::{
    self, testutils::Address as _, Address, Env, String
};

use crate::{SorobanContract__, SorobanContract__Client};

#[test]
fn test_status_message() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_id = env.register_contract(None, SorobanContract__);
    let status_message = SorobanContract__Client::new(&env, &contract_id);

    // Create a test address
    let test_address = Address::generate(&env);

    // Test setting a message
    let test_message = String::from_str(&env, "Hello, Soroban!");
    status_message.messages_set(&test_address, &test_message);

    // Test getting the message
    let retrieved_message = status_message.messages_get(&test_address);
    assert_eq!(retrieved_message, Some(test_message));

    // Test getting a non-existent message
    let non_existent_address = Address::generate(&env);
    let non_existent_message = status_message.messages_get(&non_existent_address);
    assert_eq!(non_existent_message, None);

    // Test updating an existing message
    let updated_message = String::from_str(&env, "Updated message");
    status_message.messages_set(&test_address, &updated_message);
    let retrieved_updated_message = status_message.messages_get(&test_address);
    assert_eq!(retrieved_updated_message, Some(updated_message));
}
