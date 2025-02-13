use loam_sdk::{
    loamstorage, soroban_sdk::{self, contracttype, Lazy, PersistentItem, env}, subcontract, IntoKey
};

#[subcontract]
pub trait IsRiff {
    /// Increment increments an internal counter, and returns the value.
    fn increment(&mut self) -> Result<u32, crate::error::Error>;
}

const MAX: u32 = 5;

#[loamstorage]
pub struct Impl {
    num: PersistentItem<u32>
}

impl IsRiff for Impl {
    /// Increment increments an internal counter, and returns the value. Errors
    /// if the value is attempted to be incremented past 5.
    fn increment(&mut self) -> Result<u32, crate::error::Error> {
        let mut num = self.num.get().unwrap_or_default();
        num += 1;
        if num <= MAX {
            self.num.set(&num);
            // Return the count to the caller.
            Ok(num)
        } else {
            // Return an error if the max is exceeded.
            Err(crate::error::Error::LimitReached)
        }
    }
}
