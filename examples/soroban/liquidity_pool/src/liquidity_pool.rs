use loam_sdk::{
    soroban_sdk::{self, contracttype, env, token, xdr::ScAddress, Address, BytesN, IntoKey, Lazy},
    subcontract,
};
use crate::token::create_contract;

#[contracttype]
#[derive(IntoKey)]
pub struct LiquidityPool {
    token_a: BytesN<32>,
    token_b: BytesN<32>,
    token_share: BytesN<32>,
    total_shares: i128,
    reserve_a: i128,
    reserve_b: i128,
}

impl Default for LiquidityPool {
    fn default() -> Self {
        Self {
            token_a: BytesN::from_array(&env(), &[0; 32]),
            token_b: BytesN::from_array(&env(), &[0; 32]),
            token_share: BytesN::from_array(&env(), &[0; 32]),
            total_shares: 0,
            reserve_a: 0,
            reserve_b: 0,
        }
    }
}

#[subcontract]
pub trait IsLiquidityPoolTrait {
    fn initialize(&mut self, token_wasm_hash: BytesN<32>, token_a: BytesN<32>, token_b: BytesN<32>);
    fn share_id(&self) -> BytesN<32>;
    fn deposit(&mut self, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);
    fn swap(&mut self, to: Address, buy_a: bool, out: i128, in_max: i128);
    fn withdraw(&mut self, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);
    fn get_rsrvs(&self) -> (i128, i128);
}

impl IsLiquidityPoolTrait for LiquidityPool {
    fn initialize(&mut self, token_wasm_hash: BytesN<32>, token_a: BytesN<32>, token_b: BytesN<32>) {
        if token_a >= token_b {
            panic!("token_a must be less than token_b");
        }

        let share_contract_id = create_contract(&env(), &token_wasm_hash, &token_a, &token_b);
        self.token_a = token_a;
        self.token_b = token_b;
        let sc_address: ScAddress = share_contract_id.try_into().expect("Failed to convert Address to ScAddress");
        self.token_share = match sc_address {
            ScAddress::Contract(contract_id) => BytesN::from_array(env(), &contract_id.0),
            _ => panic!("Address is not a contract"),
        };
        self.total_shares = 0;
        self.reserve_a = 0;
        self.reserve_b = 0;
    }

    fn share_id(&self) -> BytesN<32> {
        self.token_share.clone()
    }

    fn deposit(&mut self, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128) {
        // Depositor needs to authorize the deposit
        to.require_auth();

        let (reserve_a, reserve_b) = (self.reserve_a, self.reserve_b);

        // Calculate deposit amounts
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_a, reserve_b);

        let token_a_client = token::Client::new(&env(), &Address::from_string_bytes(&self.token_a.into()));
        let token_b_client = token::Client::new(&env(), &Address::from_string_bytes(&self.token_b.into()));

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
            ((balance_a * balance_b) as f64).sqrt() as i128
        };

        let shares_to_mint = new_total_shares - self.total_shares;
        token::Client::new(&env(), &Address::from_string_bytes(&self.token_share.into())).mint(&to, &shares_to_mint);

        self.total_shares = new_total_shares;
        self.reserve_a = balance_a;
        self.reserve_b = balance_b;
    }

    fn swap(&mut self, to: Address, buy_a: bool, out: i128, in_max: i128) {
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
            panic!("in amount is over max")
        }

        // Transfer the amount being sold to the contract
        let sell_token = if buy_a { self.token_b } else { self.token_a };
        let sell_token_client = token::Client::new(&env(), &sell_token);
        sell_token_client.transfer(&to, &env().current_contract_address(), &sell_amount);

        let (balance_a, balance_b) = (
            token::Client::new(&env(), &self.token_a).balance(&env().current_contract_address()),
            token::Client::new(&env(), &self.token_b).balance(&env().current_contract_address()),
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
            panic!("constant product invariant does not hold");
        }

        if buy_a {
            token::Client::new(&env(), &self.token_a).transfer(&env().current_contract_address(), &to, &out_a);
        } else {
            token::Client::new(&env(), &self.token_b).transfer(&env().current_contract_address(), &to, &out_b);
        }

        self.reserve_a = balance_a - out_a;
        self.reserve_b = balance_b - out_b;
    }

    fn withdraw(&mut self, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128) {
        to.require_auth();

        // First transfer the pool shares that need to be redeemed
        let share_token_client = token::Client::new(&env(), &self.token_share);
        share_token_client.transfer(&to, &env().current_contract_address(), &share_amount);

        let (balance_a, balance_b) = (
            token::Client::new(&env(), &self.token_a).balance(&env().current_contract_address()),
            token::Client::new(&env(), &self.token_b).balance(&env().current_contract_address()),
        );
        let balance_shares = share_token_client.balance(&env().current_contract_address());

        // Now calculate the withdraw amounts
        let out_a = (balance_a * balance_shares) / self.total_shares;
        let out_b = (balance_b * balance_shares) / self.total_shares;

        if out_a < min_a || out_b < min_b {
            panic!("min not satisfied");
        }

        share_token_client.burn(&env().current_contract_address(), &balance_shares);
        self.total_shares -= balance_shares;

        token::Client::new(&env(), &self.token_a).transfer(&env().current_contract_address(), &to, &out_a);
        token::Client::new(&env(), &self.token_b).transfer(&env().current_contract_address(), &to, &out_b);

        self.reserve_a = balance_a - out_a;
        self.reserve_b = balance_b - out_b;

        (out_a, out_b)
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
) -> (i128, i128) {
    if reserve_a == 0 && reserve_b == 0 {
        return (desired_a, desired_b);
    }

    let amount_b = desired_a * reserve_b / reserve_a;
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        (desired_a, amount_b)
    } else {
        let amount_a = desired_b * reserve_a / reserve_b;
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}