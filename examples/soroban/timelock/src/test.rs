#![cfg(test)]
extern crate std;

use super::*;
use crate::{SorobanContract__, SorobanContract__Client};
use loam_sdk::soroban_sdk::testutils::{
    Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger,
};
use loam_sdk::soroban_sdk::{self, symbol_short, token, vec, Address, Env, IntoVal};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract(admin.clone());
    (
        token::Client::new(e, &sac),
        token::StellarAssetClient::new(e, &sac),
    )
}

fn create_claimable_balance_contract<'a>(e: &Env) -> SorobanContract__Client<'a> {
    SorobanContract__Client::new(e, &e.register_contract(None, SorobanContract__ {}))
}

struct ClaimableBalanceTest<'a> {
    env: Env,
    deposit_address: Address,
    claim_addresses: [Address; 3],
    token: TokenClient<'a>,
    contract: SorobanContract__Client<'a>,
}

impl ClaimableBalanceTest<'_> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let deposit_address = Address::generate(&env);

        let claim_addresses = [
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        let token_admin = Address::generate(&env);

        let (token, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&deposit_address, &1000);

        let contract = create_claimable_balance_contract(&env);
        ClaimableBalanceTest {
            env,
            deposit_address,
            claim_addresses,
            token,
            contract,
        }
    }
}

#[test]
fn test_deposit_and_claim() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &800,
        &vec![
            &test.env,
            test.claim_addresses[0].clone(),
            test.claim_addresses[1].clone(),
        ],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    assert_eq!(
        test.env.auths(),
        [(
            test.deposit_address.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("deposit"),
                    (
                        test.deposit_address.clone(),
                        test.token.address.clone(),
                        800_i128,
                        vec![
                            &test.env,
                            test.claim_addresses[0].clone(),
                            test.claim_addresses[1].clone()
                        ],
                        TimeBound {
                            kind: TimeBoundKind::Before,
                            timestamp: 12346,
                        },
                    )
                        .into_val(&test.env),
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        test.token.address.clone(),
                        symbol_short!("transfer"),
                        (
                            test.deposit_address.clone(),
                            &test.contract.address,
                            800_i128,
                        )
                            .into_val(&test.env),
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        ),]
    );

    assert_eq!(test.token.balance(&test.deposit_address), 200);
    assert_eq!(test.token.balance(&test.contract.address), 800);
    assert_eq!(test.token.balance(&test.claim_addresses[1]), 0);

    test.contract.claim(&test.claim_addresses[1]);
    assert_eq!(
        test.env.auths(),
        [(
            test.claim_addresses[1].clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("claim"),
                    (test.claim_addresses[1].clone(),).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        ),]
    );

    assert_eq!(test.token.balance(&test.deposit_address), 200);
    assert_eq!(test.token.balance(&test.contract.address), 0);
    assert_eq!(test.token.balance(&test.claim_addresses[1]), 800);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_double_deposit_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &1,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &1,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_unauthorized_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &800,
        &vec![
            &test.env,
            test.claim_addresses[0].clone(),
            test.claim_addresses[1].clone(),
        ],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_addresses[2]);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_out_of_time_bound_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &800,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_addresses[0]);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_double_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &800,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::Before,
            timestamp: 12346,
        },
    );

    test.contract.claim(&test.claim_addresses[0]);
    assert_eq!(test.token.balance(&test.claim_addresses[0]), 800);
    test.contract.claim(&test.claim_addresses[0]);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_deposit_after_claim_not_possible() {
    let test = ClaimableBalanceTest::setup();
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &800,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );

    test.contract.claim(&test.claim_addresses[0]);
    assert_eq!(test.token.balance(&test.claim_addresses[0]), 800);
    test.contract.deposit(
        &test.deposit_address,
        &test.token.address,
        &200,
        &vec![&test.env, test.claim_addresses[0].clone()],
        &TimeBound {
            kind: TimeBoundKind::After,
            timestamp: 12344,
        },
    );
}
