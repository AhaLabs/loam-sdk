use loam_sdk::{soroban_sdk::{Symbol, Lazy}, subcontract, vec };

#[derive(Default, Lazy)]
pub struct Hello;

#[subcontract]
pub trait IsHelloWorld {
    fn hello(&self, to: Symbol) -> loam_sdk::soroban_sdk::Vec<Symbol>;
}

impl IsHelloWorld for Hello {
    fn hello(&self, to: Symbol) -> loam_sdk::soroban_sdk::Vec<Symbol> {
        vec![Symbol::short("Hello"), to]
    }
}
