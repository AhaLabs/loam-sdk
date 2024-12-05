#![no_std]

use loam_sdk::derive_contract;
use loam_sdk::soroban_sdk::Address;
use loam_subcontract_core::{admin::Admin, Core};

mod single_offer;
pub use single_offer::*;

#[derive_contract(Core(Admin), SingleOfferTrait(SingleOffer))]
pub struct Contract;

mod test;