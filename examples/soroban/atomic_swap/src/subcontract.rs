use loam_sdk::{
    soroban_sdk::{Address, env, IntoVal, Lazy, token},
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
        token_a: Address,
        token_b: Address,
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
        token_a: Address,
        token_b: Address,
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

        move_token(&token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&token_b, &b, &a, amount_b, min_b_for_a);

        Ok(())
    }
}


fn move_token(
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env(), token);
    let contract_address = env().current_contract_address();
    // This call needs to be authorized by `from` address. It transfers the
    // maximum spend amount to the swap contract's address in order to decouple
    // the signature from `to` address (so that parties don't need to know each
    // other).
    token.transfer(from, &contract_address, &max_spend_amount);
    // Transfer the necessary amount to `to`.
    token.transfer(&contract_address, to, &transfer_amount);
    // Refund the remaining balance to `from`.
    token.transfer(
        &contract_address,
        from,
        &(max_spend_amount - transfer_amount),
    );
}