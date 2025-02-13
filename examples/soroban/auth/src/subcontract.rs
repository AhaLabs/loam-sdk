use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, env, Address, Lazy, PersistentMap},
    subcontract,
};

#[loamstorage]
pub struct IncrementContract {
    counters: PersistentMap<Address, u32>,
}

#[subcontract]
pub trait IsIncrementable {
    fn increment(&mut self, user: Address, value: u32) -> u32;
}

impl IsIncrementable for IncrementContract {
    fn increment(&mut self, user: Address, value: u32) -> u32 {
        user.require_auth();

        let count = self.counters.get(user.clone()).unwrap_or(0);
        let new_count = count + value;
        self.counters.set(user, &new_count);

        new_count
    }
}
