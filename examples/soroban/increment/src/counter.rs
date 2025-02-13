use loam_sdk::{
    loamstorage, soroban_sdk::{self, Lazy, PersistentItem}, subcontract
};

#[loamstorage]
pub struct Counter{
    count: PersistentItem<u32>
}

#[subcontract]
pub trait IsIncrementable {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32;
}

impl IsIncrementable for Counter {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32 {
        let mut count = self.count.get().unwrap_or_default();
        count += 1;
        self.count.set(&count);
        count
    }
}
