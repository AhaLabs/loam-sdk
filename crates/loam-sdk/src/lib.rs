#![no_std]
pub use loam_sdk_macro::*;

#[cfg(feature = "loam-soroban-sdk")]
pub mod soroban_sdk;

#[cfg(feature = "loam-soroban-sdk")]
#[macro_export]
macro_rules! vec {
    ($($tokens:tt)*) => {
        soroban_sdk::vec![soroban_sdk::env(), $($tokens)*]
    };
}

#[cfg(feature = "loam-soroban-sdk")]
#[macro_export]
macro_rules! map {
    ($($tokens:tt)*) => {
        soroban_sdk::map![soroban_sdk::env(), $($tokens)*]
    };
}

#[cfg(feature = "loam-soroban-sdk")]
#[macro_export]
macro_rules! log {
    ($($tokens:tt)*) => {
        {
        use soroban_sdk::Env;
        soroban_sdk::log![soroban_sdk::env(), $($tokens)*];
        }
    };
}
