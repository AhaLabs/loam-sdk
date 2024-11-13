use loam_sdk::{
    soroban_sdk::{self, contracttype, IntoKey, Lazy},
    subcontract,
};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Counter(u32);

#[subcontract]
pub trait IsIncrementable {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32;
}

impl IsIncrementable for Counter {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}
