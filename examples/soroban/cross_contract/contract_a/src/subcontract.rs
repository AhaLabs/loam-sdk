use loam_sdk::{
    soroban_sdk::{self, Lazy},
    subcontract,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct ContractA;

#[subcontract]
pub trait IsContractA {
    /// Add two 32 bit unsigned integers
    #[allow(clippy::missing_errors_doc)]
    fn add(&self, x: u32, y: u32) -> Result<u32, Error>;
}

impl IsContractA for ContractA {
    fn add(&self, x: u32, y: u32) -> Result<u32, Error> {
        x.checked_add(y).ok_or(Error::Overflow)
    }
}
