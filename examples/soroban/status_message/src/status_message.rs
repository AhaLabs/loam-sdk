#![allow(deprecated)]
// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, env, Address, Lazy, PersistentMap, String},
    subcontract,
};

#[loamstorage]
pub struct StatusMessage {
    messages: PersistentMap<Address, String>,
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
    fn messages_get(&self, author: Address) -> Option<String> {
        self.messages.get(author)
    }

    fn messages_set(&mut self, author: Address, text: String) {
        author.require_auth();
        self.messages.set(author, &text);
    }
}
