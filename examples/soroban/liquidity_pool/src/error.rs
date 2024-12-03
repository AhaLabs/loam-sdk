use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidTokenOrder = 1,
    ExceededMaxInput = 2,
    InvariantViolation = 3,
    MinimumNotSatisfied = 4,
    InsufficientBAmount = 5,
    InvalidAAmount = 6,
}