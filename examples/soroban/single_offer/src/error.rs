use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SingleOfferError {
    OfferAlreadyCreated = 1,
    ZeroPriceNotAllowed = 2,
    PriceTooLow = 3,
}