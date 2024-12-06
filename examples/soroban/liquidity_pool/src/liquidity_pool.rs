use crate::{error::Error, token::example_token as token};
use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, BytesN, IntoKey, IntoVal, Lazy},
    subcontract,
};
use num_integer::Roots;

#[contracttype]
#[derive(IntoKey)]
pub struct LiquidityPool {
    token_a: Address,
    token_b: Address,
    token_share: Address,
    total_shares: i128,
    reserve_a: i128,
    reserve_b: i128,
}

impl Default for LiquidityPool {
    fn default() -> Self {
        Self {
            token_a: env().current_contract_address(),
            token_b: env().current_contract_address(),
            token_share: env().current_contract_address(),
            total_shares: 0,
            reserve_a: 0,
            reserve_b: 0,
        }
    }
}

#[subcontract]
pub trait IsLiquidityPoolTrait {
    fn initialize(
        &mut self,
        token_wasm_hash: BytesN<32>,
        token_a: Address,
        token_b: Address,
    ) -> Result<(), Error>;
    fn share_id(&self) -> Address;
    fn deposit(
        &mut self,
        to: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) -> Result<(), Error>;
    fn swap(&mut self, to: Address, buy_a: bool, out: i128, in_max: i128) -> Result<(), Error>;
    fn withdraw(
        &mut self,
        to: Address,
        share_amount: i128,
        min_a: i128,
        min_b: i128,
    ) -> Result<(i128, i128), Error>;
    fn get_rsrvs(&self) -> (i128, i128);
}

impl IsLiquidityPoolTrait for LiquidityPool {
    fn initialize(
        &mut self,
        token_wasm_hash: BytesN<32>,
        token_a: Address,
        token_b: Address,
    ) -> Result<(), Error> {
        if token_a >= token_b {
            return Err(Error::InvalidTokenOrder);
        }

        let share_contract_id =
            crate::token::create_contract(env(), &token_wasm_hash, &token_a, &token_b);
        token::Client::new(env(), &share_contract_id).initialize(
            &env().current_contract_address(),
            &7u32,
            &"Pool Share Token".into_val(env()),
            &"POOL".into_val(env()),
        );
        self.token_a = token_a;
        self.token_b = token_b;
        self.token_share = share_contract_id;
        self.total_shares = 0;
        self.reserve_a = 0;
        self.reserve_b = 0;
        Ok(())
    }

    fn share_id(&self) -> Address {
        self.token_share.clone()
    }

    fn deposit(
        &mut self,
        to: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) -> Result<(), Error> {
        // Depositor needs to authorize the deposit
        to.require_auth();

        let (reserve_a, reserve_b) = (self.reserve_a, self.reserve_b);

        // Calculate deposit amounts
        let amounts =
            get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_a, reserve_b)?;

        if amounts.0 <= 0 || amounts.1 <= 0 {
            return Err(Error::DepositAmountsMustBePositive);
        }
        let token_a_client = token::Client::new(env(), &self.token_a);
        let token_b_client = token::Client::new(env(), &self.token_b);

        token_a_client.transfer(&to, &env().current_contract_address(), &amounts.0);
        token_b_client.transfer(&to, &env().current_contract_address(), &amounts.1);

        // Now calculate how many new pool shares to mint
        let (balance_a, balance_b) = (
            token_a_client.balance(&env().current_contract_address()),
            token_b_client.balance(&env().current_contract_address()),
        );

        let zero = 0;
        let new_total_shares = if reserve_a > zero && reserve_b > zero {
            let shares_a = (balance_a * self.total_shares) / reserve_a;
            let shares_b = (balance_b * self.total_shares) / reserve_b;
            shares_a.min(shares_b)
        } else {
            (balance_a * balance_b).sqrt()
        };

        let shares_to_mint = new_total_shares - self.total_shares;
        token::Client::new(env(), &self.token_share).mint(&to, &shares_to_mint);

        self.total_shares = new_total_shares;
        self.reserve_a = balance_a;
        self.reserve_b = balance_b;
        Ok(())
    }

    fn swap(&mut self, to: Address, buy_a: bool, out: i128, in_max: i128) -> Result<(), Error> {
        to.require_auth();

        let (reserve_a, reserve_b) = (self.reserve_a, self.reserve_b);
        let (reserve_sell, reserve_buy) = if buy_a {
            (reserve_b, reserve_a)
        } else {
            (reserve_a, reserve_b)
        };

        // Calculate how much needs to be sold to buy amount out from the pool
        let n = reserve_sell * out * 1000;
        let d = (reserve_buy - out) * 997;
        let sell_amount = (n / d) + 1;
        if sell_amount > in_max {
            return Err(Error::ExceededMaxInput);
        }

        // Transfer the amount being sold to the contract
        let sell_token = if buy_a {
            self.token_b.clone()
        } else {
            self.token_a.clone()
        };
        let sell_token_client = token::Client::new(env(), &sell_token);
        sell_token_client.transfer(&to, &env().current_contract_address(), &sell_amount);

        let (balance_a, balance_b) = (
            token::Client::new(env(), &self.token_a).balance(&env().current_contract_address()),
            token::Client::new(env(), &self.token_b).balance(&env().current_contract_address()),
        );

        // residue_numerator and residue_denominator are the amount that the invariant considers after
        // deducting the fee, scaled up by 1000 to avoid fractions
        let residue_numerator = 997;
        let residue_denominator = 1000;
        let zero = 0;

        let new_invariant_factor = |balance: i128, reserve: i128, out: i128| {
            let delta = balance - reserve - out;
            let adj_delta = if delta > zero {
                residue_numerator * delta
            } else {
                residue_denominator * delta
            };
            residue_denominator * reserve + adj_delta
        };

        let (out_a, out_b) = if buy_a { (out, 0) } else { (0, out) };

        let new_inv_a = new_invariant_factor(balance_a, reserve_a, out_a);
        let new_inv_b = new_invariant_factor(balance_b, reserve_b, out_b);
        let old_inv_a = residue_denominator * reserve_a;
        let old_inv_b = residue_denominator * reserve_b;

        if new_inv_a * new_inv_b < old_inv_a * old_inv_b {
            return Err(Error::InvariantViolation);
        }

        if buy_a {
            token::Client::new(env(), &self.token_a).transfer(
                &env().current_contract_address(),
                &to,
                &out_a,
            );
        } else {
            token::Client::new(env(), &self.token_b).transfer(
                &env().current_contract_address(),
                &to,
                &out_b,
            );
        }

        let new_reserve_a = balance_a - out_a;
        let new_reserve_b = balance_b - out_b;

        if new_reserve_a <= 0 || new_reserve_b <= 0 {
            return Err(Error::NewReservesMustBePositive);
        }

        self.reserve_a = new_reserve_a;
        self.reserve_b = new_reserve_b;

        Ok(())
    }

    fn withdraw(
        &mut self,
        to: Address,
        share_amount: i128,
        min_a: i128,
        min_b: i128,
    ) -> Result<(i128, i128), Error> {
        to.require_auth();

        // First transfer the pool shares that need to be redeemed
        let share_token_client = token::Client::new(env(), &self.token_share);
        share_token_client.transfer(&to, &env().current_contract_address(), &share_amount);

        let (balance_a, balance_b) = (
            token::Client::new(env(), &self.token_a).balance(&env().current_contract_address()),
            token::Client::new(env(), &self.token_b).balance(&env().current_contract_address()),
        );
        let balance_shares = share_token_client.balance(&env().current_contract_address());

        // Now calculate the withdraw amounts
        let out_a = (balance_a * balance_shares) / self.total_shares;
        let out_b = (balance_b * balance_shares) / self.total_shares;

        if out_a < min_a || out_b < min_b {
            return Err(Error::MinimumNotSatisfied);
        }

        share_token_client.burn(&env().current_contract_address(), &balance_shares);
        self.total_shares -= balance_shares;

        token::Client::new(env(), &self.token_a).transfer(
            &env().current_contract_address(),
            &to,
            &out_a,
        );
        token::Client::new(env(), &self.token_b).transfer(
            &env().current_contract_address(),
            &to,
            &out_b,
        );

        self.reserve_a = balance_a - out_a;
        self.reserve_b = balance_b - out_b;

        Ok((out_a, out_b))
    }

    fn get_rsrvs(&self) -> (i128, i128) {
        (self.reserve_a, self.reserve_b)
    }
}

fn get_deposit_amounts(
    desired_a: i128,
    min_a: i128,
    desired_b: i128,
    min_b: i128,
    reserve_a: i128,
    reserve_b: i128,
) -> Result<(i128, i128), Error> {
    if reserve_a == 0 && reserve_b == 0 {
        return Ok((desired_a, desired_b));
    }

    let amount_b = desired_a * reserve_b / reserve_a;
    if amount_b <= desired_b {
        if amount_b < min_b {
            return Err(Error::MinimumNotSatisfied);
        }
        Ok((desired_a, amount_b))
    } else {
        let amount_a = desired_b * reserve_a / reserve_b;
        if amount_a > desired_a || desired_a < min_a {
            return Err(Error::InvalidAAmount);
        }
        Ok((amount_a, desired_b))
    }
}
