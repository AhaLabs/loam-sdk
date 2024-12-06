use loam_sdk::{
    soroban_sdk::{env, log, Env, Lazy, Symbol},
    subcontract,
};

#[derive(Lazy, Default)]
pub struct Logger;

#[subcontract]
pub trait IsLog {
    /// Log a hello message with the given value
    #[allow(clippy::missing_errors_doc)]
    fn hello(&self, value: Symbol);
}

impl IsLog for Logger {
    fn hello(&self, value: Symbol) {
        log!(env(), "Hello {}", value);
    }
}
