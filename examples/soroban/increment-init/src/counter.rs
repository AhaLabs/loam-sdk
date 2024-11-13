use loam_sdk::{
    soroban_sdk::{self, contracttype, IntoKey, Lazy},
    subcontract,
};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Counter(u32);

#[subcontract]
pub trait IsCountable {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32;

    fn init(&mut self, num: u32);
}

impl IsCountable for Counter {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }

    fn init(&mut self, num: u32) {
        self.0 = num;
    }
}
