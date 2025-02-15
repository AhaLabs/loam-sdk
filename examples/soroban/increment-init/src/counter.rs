use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, Lazy, PersistentItem},
    subcontract,
};

#[loamstorage]
pub struct Counter{
    count: PersistentItem<u32>
}

#[subcontract]
pub trait IsCountable {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32;

    fn init(&mut self, num: u32);
}

impl IsCountable for Counter {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32 {
        let mut count = self.count.get().unwrap();
        count += 1;
        self.count.set(&count);
        count
    }

    fn init(&mut self, num: u32) {
        self.count.set(&num);
    }
}
