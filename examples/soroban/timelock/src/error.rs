use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    TooManyClaimants = 1,
    AlreadyInitialized = 2,
    TimePredicateNotFulfilled = 3,
    ClaimantNotAllowed = 4,
    BalanceAlreadyClaimed = 5,
}
