// src/subcontract.rs
use loam_sdk::{
    soroban_sdk::{self, Address, BytesN, Env, Lazy, Vec},
    subcontract,
};

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
        env: Env,
        swap_contract: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    );
}

mod atomic_swap {
    soroban_sdk::contractimport!(
        file = "../atomic_swap/target/wasm32-unknown-unknown/release/soroban_atomic_swap_contract.wasm"
    );
}

impl IsAtomicMultiSwap for AtomicMultiSwapContract {
    fn multi_swap(
        &self,
        env: Env,
        swap_contract: BytesN<32>,
        token_a: BytesN<32>,
        token_b: BytesN<32>,
        swaps_a: Vec<SwapSpec>,
        swaps_b: Vec<SwapSpec>,
    ) {
        let mut swaps_b = swaps_b;
        let swap_client = atomic_swap::Client::new(&env, &swap_contract);
        for acc_a in swaps_a.iter() {
            let acc_a = acc_a.unwrap();
            for i in 0..swaps_b.len() {
                let acc_b = swaps_b.get(i).unwrap().unwrap();

                if acc_a.amount >= acc_b.min_recv && acc_a.min_recv <= acc_b.amount {
                    if swap_client
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
}