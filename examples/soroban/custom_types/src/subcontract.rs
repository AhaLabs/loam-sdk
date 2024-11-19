use loam_sdk::{
    soroban_sdk::{self, contracttype, Lazy, IntoKey},
    subcontract,
};

#[contracttype]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

#[contracttype]
#[derive(IntoKey, Default)]
pub struct IncrementContract(State);

#[subcontract]
pub trait IsIncrement {
    fn increment(&mut self, incr: u32) -> u32;
    fn get_state(&self) -> State;
}


impl IsIncrement for IncrementContract {
    fn increment(&mut self, incr: u32) -> u32 {
        self.0.count += incr;
        self.0.last_incr = incr;
        self.0.count
    }

    fn get_state(&self) -> State {
        self.0.clone()
    }
}
