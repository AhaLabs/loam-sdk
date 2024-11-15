// src/subcontract.rs
use loam_sdk::{
    soroban_sdk::{self, Lazy},
    subcontract, vec,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct AllocContract;

#[subcontract]
pub trait IsAlloc {
    /// Allocates a temporary vector holding values (0..count), then computes and returns their sum.
    #[allow(clippy::missing_errors_doc)]
    fn sum(&self, count: u32) -> Result<u32, Error>;
}

impl IsAlloc for AllocContract {
    fn sum(&self, count: u32) -> Result<u32, Error> {
        let mut v1 = vec![];
        (0..count).for_each(|i| v1.push_back(i));

        let mut sum: u32 = 0; 
        for i in v1 {
            sum = sum.checked_add(i).ok_or(Error::Overflow)?;
        }

        Ok(sum)
    }
}
