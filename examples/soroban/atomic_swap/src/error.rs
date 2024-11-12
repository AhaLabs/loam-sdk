// error.rs

use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Not enough token B for token A
    NotEnoughTokenB = 1,
    /// Not enough token A for token B
    NotEnoughTokenA = 2,
}
