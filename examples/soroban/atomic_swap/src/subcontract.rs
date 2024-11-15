use loam_sdk::{
    soroban_sdk::{self, Address, BytesN, env, IntoVal, Lazy, token},
    subcontract,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct AtomicSwapContract;

#[subcontract]
pub trait IsAtomicSwap {
    fn swap(
        &self,
        a: Address,
        b: Address,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) -> Result<(), Error>;
}

impl IsAtomicSwap for AtomicSwapContract {
    fn swap(
        &self,
        a: Address,
        b: Address,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) -> Result<(), Error> {
        if amount_b < min_b_for_a {
            return Err(Error::NotEnoughTokenB);
        }
        if amount_a < min_a_for_b {
            return Err(Error::NotEnoughTokenA);
        }

        a.require_auth_for_args(
            (token_a.clone(), token_b.clone(), amount_a, min_b_for_a).into_val(env()),
        );
        b.require_auth_for_args(
            (token_b.clone(), token_a.clone(), amount_b, min_a_for_b).into_val(env()),
        );

        move_token(token_a, &a, &b, amount_a, min_a_for_b);
        move_token(token_b, &b, &a, amount_b, min_b_for_a);

        Ok(())
    }
}

fn move_token(
    token: BytesN<32>,
    from: &Address,
    to: &Address,
    approve_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env(), &Address::from_string_bytes(&token.into()));
    let contract_address = env().current_contract_address();
    // ledger entry of expiration
    let ledger = env().ledger().sequence() + 1000;
    token.approve(from, &contract_address, &approve_amount, &ledger);
    token.transfer_from(&contract_address, &from, to, &transfer_amount);
}
