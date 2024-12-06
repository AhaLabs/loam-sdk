use loam_sdk::{
    soroban_sdk::{self, contracttype, env, IntoKey, Lazy, Symbol},
    subcontract,
};

const COUNTER: Symbol = Symbol::short("COUNTER");

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

        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env()
            .events()
            .publish((COUNTER, Symbol::short("increment")), self.0);

        // Return the count to the caller.
        self.0
    }
}
