//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Lazy, BytesN, Env, IntoKey, Vec},
    subcontract,
};

#[contracttype]
#[derive(Clone)]
pub enum TimeBoundKind {
    Before,
    After,
}

#[contracttype]
#[derive(Clone)]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct ClaimableBalance {
    pub token: BytesN<32>,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contracttype]
#[derive(IntoKey, Clone)]
pub struct Timelock {
    balance: ClaimableBalance,
}

impl Default for Timelock {
    fn default() -> Self {
        Self {
            balance: ClaimableBalance {
                token: BytesN::from_array(&env(), &[0; 32]),
                amount: 0,
                claimants: Vec::new(&env()),
                time_bound: TimeBound {
                    kind: TimeBoundKind::Before,
                    timestamp: 0,
                },
            },
        }
    }
}

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[subcontract]
pub trait IsTimelockTrait {
    fn deposit(
        &mut self,
        from: Address,
        token: BytesN<32>,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    );
    fn claim(&mut self, claimant: Address);
}

impl IsTimelockTrait for Timelock {
    fn deposit(
        &mut self,
        from: Address,
        token: BytesN<32>,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if self.balance.amount != 0 {
            panic!("contract has been already initialized");
        }
        from.require_auth();

        let token_client = soroban_sdk::token::Client::new(&env(), &Address::from_string_bytes(&token.clone().into()));
        token_client.transfer(&from, &env().current_contract_address(), &amount);

        self.balance = ClaimableBalance {
            token,
            amount,
            time_bound,
            claimants,
        };
    }

    fn claim(&mut self, claimant: Address) {
        claimant.require_auth();

        if !check_time_bound(&env(), &self.balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        if !self.balance.claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        let token_client = soroban_sdk::token::Client::new(&env(), &Address::from_string_bytes(&self.balance.token.clone().into()));
        token_client.transfer(
            &env().current_contract_address(),
            &claimant,
            &self.balance.amount,
        );

        self.balance.amount = 0;
    }
}
