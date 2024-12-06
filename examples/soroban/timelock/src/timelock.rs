//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! https://developers.stellar.org/docs/glossary/claimable-balance).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, BytesN, Env, IntoKey, Lazy, Vec},
    subcontract,
};

use crate::error::TimelockError;

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
    pub token: Address,
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
                token: env().current_contract_address(),
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
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) -> Result<(), TimelockError>;
    fn claim(&mut self, claimant: Address) -> Result<(), TimelockError>;
}

impl IsTimelockTrait for Timelock {
    fn deposit(
        &mut self,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) -> Result<(), TimelockError> {
        if claimants.len() > 10 {
            return Err(TimelockError::TooManyClaimants);
        }
        if self.balance.claimants.len() != 0 {
            return Err(TimelockError::AlreadyInitialized);
        }
        from.require_auth();

        let token_client = soroban_sdk::token::Client::new(&env(), &token.clone());
        token_client.transfer(&from, &env().current_contract_address(), &amount);

        self.balance = ClaimableBalance {
            token,
            amount,
            time_bound,
            claimants,
        };
        Ok(())
    }

    fn claim(&mut self, claimant: Address) -> Result<(), TimelockError> {
        claimant.require_auth();

        if self.balance.amount == 0 {
            return Err(TimelockError::BalanceAlreadyClaimed);
        }

        if !check_time_bound(&env(), &self.balance.time_bound) {
            return Err(TimelockError::TimePredicateNotFulfilled);
        }

        if !self.balance.claimants.contains(&claimant) {
            return Err(TimelockError::ClaimantNotAllowed);
        }

        let token_client = soroban_sdk::token::Client::new(&env(), &self.balance.token.clone());
        token_client.transfer(
            &env().current_contract_address(),
            &claimant,
            &self.balance.amount,
        );

        self.balance.amount = 0;
        Ok(())
    }
}
