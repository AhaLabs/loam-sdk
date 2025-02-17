#![no_std]

use soroban_sdk::unwrap::UnwrapOptimized;
pub use soroban_sdk::*;

pub mod into_key;
pub mod loam_storage;

pub use into_key::IntoKey;
pub use loam_storage::*;

/// Trait for loading and setting a singleton type
pub trait Lazy: Sized {
    fn get_lazy() -> Option<Self>;

    fn set_lazy(self);
}

static mut ENV: Option<Env> = None;

pub fn set_env(env: Env) {
    unsafe { ENV = Some(env) };
}

/// Utility to cast a `&str` to a `String`.
#[must_use]
pub fn to_string(s: &str) -> String {
    soroban_sdk::String::from_str(env(), s)
}

/// Returns a reference to the current environment.
///
/// # Panics
///
/// This function will panic if the environment has not been initialized.
/// It is expected that the environment is always initialized before this
/// function is called in normal operation.
#[must_use]
#[allow(static_mut_refs)]
#[inline]
pub fn env() -> &'static Env {
    unsafe { ENV.as_ref().unwrap_optimized() }
}

impl<T> Lazy for T
where
    T: IntoKey + TryFromVal<Env, Val> + IntoVal<Env, Val>,
{
    fn get_lazy() -> Option<Self> {
        env().storage().persistent().get(&Self::into_key())
    }

    fn set_lazy(self) {
        env().storage().persistent().set(&Self::into_key(), &self);
    }
}

pub use loam_sdk_macro::{IntoKey, Lazy};
