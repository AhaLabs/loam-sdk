use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, contracttype, env, Lazy, PersistentItem},
    subcontract,
};

#[contracttype]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

#[loamstorage]
pub struct Inc {
    s: PersistentItem<State>,
}

#[subcontract]
pub trait IsIncrementable {
    fn increment(&mut self, incr: u32) -> u32;
    fn get_state(&self) -> State;
}

impl IsIncrementable for Inc {
    fn increment(&mut self, incr: u32) -> u32 {
        let mut state = self.s.get().unwrap_or_default();
        state.count += incr;
        state.last_incr = incr;
        self.s.set(&state);
        state.count
    }

    fn get_state(&self) -> State {
        self.s.get().unwrap_or_default()
    }
}
