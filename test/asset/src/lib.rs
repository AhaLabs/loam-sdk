#![no_std]
use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, env, Address, InstanceStore, Lazy, LoamKey, PersistentMap, String},
    subcontract,
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

impl IsAToken for Token {
    fn init(&mut self, name: String, symbol: String, decimals: u32) {
        Self::init(name, symbol, decimals);
    }

    fn name(&self) -> Option<String> {
        self.name.get()
    }
}

#[subcontract]
pub trait IsAToken {
    fn init(&mut self, name: String, symbol: String, decimals: u32);

    fn name(&self) -> Option<String>;
}
