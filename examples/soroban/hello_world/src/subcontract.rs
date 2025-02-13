use loam_sdk::{
    soroban_sdk::{self, Lazy, Symbol, Vec, symbol_short},
    subcontract, vec,
};

#[derive(Default, Lazy)]
pub struct Hello;

#[subcontract]
pub trait IsHelloWorld {
    fn hello(&self, to: Symbol) -> Vec<Symbol>;
}

impl IsHelloWorld for Hello {
    fn hello(&self, to: Symbol) -> Vec<Symbol> {
        vec![symbol_short!("Hello"), to]
    }
}
