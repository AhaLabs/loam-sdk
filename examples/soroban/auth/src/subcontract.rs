use loam_sdk::{
    soroban_sdk::{self, contracttype, Address, IntoKey, Lazy, Map},
    subcontract,
};

#[contracttype]
#[derive(IntoKey)]
pub struct IncrementContract(Map<Address, u32>);

impl Default for IncrementContract {
    fn default() -> Self {
        Self(Map::new(loam_sdk::soroban_sdk::env()))
    }
}

#[subcontract]
pub trait IsIncrementable {
    fn increment(&mut self, user: Address, value: u32) -> u32;
}

impl IsIncrementable for IncrementContract {
    fn increment(&mut self, user: Address, value: u32) -> u32 {
        user.require_auth();

        let count = self.0.get(user.clone()).unwrap_or(0);
        let new_count = count + value;
        self.0.set(user, new_count);

        new_count
    }
}
