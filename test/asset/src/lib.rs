#![no_std]
use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, env, Address, InstanceStore, LoamKey, PersistentMap, String},
};

mod types;
use types::{Allowance, Txn};

#[loamstorage]
#[derive(Default)]
pub struct Token {
    /// Name of the token
    name: InstanceStore<String>,
    /// Mapping of account addresses to their token balances
    balances: PersistentMap<Address, i128>,
    /// Mapping of transactions to their associated allowances
    allowances: PersistentMap<Txn, Allowance>,
    /// Mapping of addresses to their authorization status
    authorized: PersistentMap<Address, bool>,
    /// Symbol of the token
    symbol: InstanceStore<String>,
    /// Number of decimal places for token amounts
    decimals: InstanceStore<u32>,
}

impl Token {
    pub fn init(name: String, symbol: String, decimals: u32) {
        let mut token = Token::default();
        token.name.set(&name);
        token.symbol.set(&symbol);
        token.decimals.set(&decimals);
    }
}
