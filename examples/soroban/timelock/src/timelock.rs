//! This contract demonstrates 'timelock' concept and implements a
//! greatly simplified Claimable Balance (similar to
//! <https://developers.stellar.org/docs/glossary/claimable-balance>).
//! The contract allows to deposit some amount of token and allow another
//! account(s) claim it before or after provided time point.
//! For simplicity, the contract only supports invoker-based auth.
use loam_sdk::{
    loamstorage, soroban_sdk::{self, contracttype, env, Address, Env, Lazy, PersistentItem, Vec}, subcontract
};

use crate::error::Error;

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

#[loamstorage]
pub struct Timelock {
    balance: PersistentItem<ClaimableBalance>,
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
    ) -> Result<(), Error>;
    fn claim(&mut self, claimant: Address) -> Result<(), Error>;
}

impl IsTimelockTrait for Timelock {
    fn deposit(
        &mut self,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) -> Result<(), Error> {
        if claimants.len() > 10 {
            return Err(Error::TooManyClaimants);
        }
        if self.balance.has() {
            return Err(Error::AlreadyInitialized);
        }
        from.require_auth();

        let token_client = soroban_sdk::token::Client::new(env(), &token.clone());
        token_client.transfer(&from, &env().current_contract_address(), &amount);

        self.balance.set(&ClaimableBalance {
            token,
            amount,
            claimants,
            time_bound,
        });
        Ok(())
    }

    fn claim(&mut self, claimant: Address) -> Result<(), Error> {
        claimant.require_auth();
        let mut balance = self.balance.get().unwrap();

        if balance.amount == 0 {
            return Err(Error::BalanceAlreadyClaimed);
        }

        if !check_time_bound(env(), &balance.time_bound) {
            return Err(Error::TimePredicateNotFulfilled);
        }

        if !balance.claimants.contains(&claimant) {
            return Err(Error::ClaimantNotAllowed);
        }

        let token_client = soroban_sdk::token::Client::new(env(), &balance.token.clone());
        token_client.transfer(
            &env().current_contract_address(),
            &claimant,
            &balance.amount,
        );

        balance.amount = 0;
        self.balance.set(&balance);
        Ok(())
    }
}
