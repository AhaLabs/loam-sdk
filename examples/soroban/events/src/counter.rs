use loam_sdk::{
    loamstorage, soroban_sdk::{self,  env, symbol_short,  PersistentItem, Symbol, Lazy}, subcontract
};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[loamstorage]
pub struct Counter {
    count: PersistentItem<u32>,
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
        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env()
            .events()
            .publish((COUNTER, symbol_short!("increment")), count);

        // Return the count to the caller.
        count
    }
}
