// src/subcontract.rs
use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Lazy, Vec},
    subcontract,
};

use crate::example_atomic_swap as atomic_swap;

#[derive(Clone)]
#[contracttype]
pub struct SwapSpec {
    pub address: Address,
    pub amount: i128,
    pub min_recv: i128,
}

#[derive(Lazy, Default)]
pub struct AtomicMultiSwapContract;

#[subcontract]
pub trait IsAtomicMultiSwap {
    fn multi_swap(
        &self,
        swap_contract: Address,
        token_a: Address,
        token_b: Address,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    );
}

impl IsAtomicMultiSwap for AtomicMultiSwapContract {
    fn multi_swap(
        &self,
        swap_contract: Address,
        token_a: Address,
        token_b: Address,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    ) {
        let mut swaps_b = swaps_b;
        let swap_client = atomic_swap::Client::new(env(), &swap_contract);
        for acc_a in swaps_a.iter() {
            for i in 0..swaps_b.len() {
                let acc_b = swaps_b.get(i).unwrap();

                if acc_a.amount >= acc_b.min_recv
                    && acc_a.min_recv <= acc_b.amount
                    && swap_client
                        .try_swap(
                            &acc_a.address,
                            &acc_b.address,
                            &token_a,
                            &token_b,
                            &acc_a.amount,
                            &acc_a.min_recv,
                            &acc_b.amount,
                            &acc_b.min_recv,
                        )
                        .is_ok()
                {
                    swaps_b.remove(i);
                    break;
                }
            }
        }
    }
}
