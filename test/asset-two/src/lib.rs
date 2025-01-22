#![no_std]
use loam_sdk::{
    derive_contract,
    soroban_sdk::{self, contracttype, env, Address, Lazy, Map, String},
    subcontract, IntoKey,
};

mod types;
use loam_subcontract_core::{Admin, Core};
use types::{Allowance, Txn};

#[derive_contract(Core(Admin), AToken(Token))]
pub struct Contract;

#[contracttype]
#[derive(IntoKey)]
pub struct Token {
    /// Name of the token
    name: String,
    /// Mapping of account addresses to their token balances
    balances: Map<Address, i128>,
    /// Mapping of transactions to their associated allowances
    allowances: Map<Txn, Allowance>,
    /// Mapping of addresses to their authorization status
    authorized: Map<Address, bool>,
    /// Symbol of the token
    symbol: String,
    /// Number of decimal places for token amounts
    decimals: u32,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            name: String::from_str(env(), ""),
            balances: Map::new(env()),
            allowances: Map::new(env()),
            authorized: Map::new(env()),
            symbol: String::from_str(env(), ""),
            decimals: 0,
        }
    }
}

impl Token {
    pub fn init(&mut self, name: String, symbol: String, decimals: u32) {
        *self = Token {
            name,
            balances: Map::new(env()),
            allowances: Map::new(env()),
            authorized: Map::new(env()),
            symbol,
            decimals,
        };
    }
}

impl IsAToken for Token {
    fn init(&mut self, name: String, symbol: String, decimals: u32) {
        self.init(name, symbol, decimals);
    }

    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn set_balance(&mut self, address: Address, amount: i128) {
        address.require_auth();
        self.balances.set(address, amount);
    }
}

#[subcontract]
pub trait IsAToken {
    fn init(&mut self, name: String, symbol: String, decimals: u32);

    fn name(&self) -> Option<String>;

    fn set_balance(&mut self, address: Address, amount: i128);
}
