#![allow(deprecated)]
// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    log,
    soroban_sdk::{self, contracttype, env, to_string, Address, IntoKey, Lazy, Map, String},
    subcontract,
};

#[contracttype]
#[derive(IntoKey)]
pub struct StatusMessage(Map<Address, String>);

impl Default for StatusMessage {
    fn default() -> Self {
        Self(Map::new(env()))
    }
}

#[subcontract]
pub trait IsPostable {
    /// Documentation ends up in the contract's metadata and thus the CLI, etc
    fn messages_get(&self, author: loam_sdk::soroban_sdk::Address)
        -> loam_sdk::soroban_sdk::String;

    /// Only the author can set the message
    fn messages_set(
        &mut self,
        author: loam_sdk::soroban_sdk::Address,
        text: loam_sdk::soroban_sdk::String,
    );
}

impl IsPostable for StatusMessage {
    fn messages_get(&self, author: Address) -> String {
        self.0
            .get(author)
            .unwrap_or_else(|| to_string("No message"))
    }

    fn messages_set(&mut self, author: Address, text: String) {
        author.require_auth();
        log!("Setting message {} for {}", text, author);
        self.0.set(author, text);
    }
}
