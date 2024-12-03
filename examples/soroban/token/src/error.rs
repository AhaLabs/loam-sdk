use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The operation results in an integer overflow
    InsufficientBalance = 1,
    AlreadyInitialized = 2,
    InsufficientAllowance = 3,
    ExpirationInPast = 4,
    ExpirationOverflow = 5,
}

