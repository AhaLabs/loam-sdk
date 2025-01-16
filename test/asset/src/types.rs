use loam_sdk::soroban_sdk::{self, contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct Txn(pub Address, pub Address);

#[contracttype]
#[derive(Clone)]
pub struct Allowance {
    pub amount: i128,
    pub live_until_ledger: u32,
}
