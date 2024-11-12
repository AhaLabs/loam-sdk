use loam_sdk::{
    soroban_sdk::{self, auth::Context, contracttype, env, BytesN, Env, Lazy, Map, Symbol, Vec},
    subcontract, IntoKey
};

use crate::error::AccError;

const TRANSFER_FN: Symbol = Symbol::short("transfer");

#[contracttype]
#[derive(Clone)]
pub struct Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(IntoKey)]
pub struct AccountManager {
    limits: Map<BytesN<32>, i128>,
    signers: Map<BytesN<32>, i128>, 
    count: i32,
}

#[subcontract]
pub trait IsAccount {
    fn init(&self, signers: Vec<BytesN<32>>);
    fn add_limit(&self, token: BytesN<32>, limit: i128);
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<Signature>,
        auth_context: Vec<Context>,
    ) -> Result<(), AccError>;
}

impl IsAccount for AccountManager {
    fn init(&self, signers: Vec<BytesN<32>>) {
        // In reality this would need some additional validation on signers
        // (deduplication etc.).
        for signer in signers.iter() {
            self.signers.set(signer, &());
        }
        self.count = &signers.len();
    }

    fn add_limit(&self, token: BytesN<32>, limit: i128) {
        // The current contract address is the account contract address and has
        // the same semantics for `require_auth` call as any other account
        // contract address.
        // Note, that if a contract *invokes* another contract, then it would
        // authorize the call on its own behalf and that wouldn't require any
        // user-side verification.
        &self.require_auth();
        &self.limits.set(token, &limit);
    }



    // This is the 'entry point' of the account contract and every account
    // contract has to implement it. `require_auth` calls for the Address of
    // this contract will result in calling this `__check_auth` function with
    // the appropriate arguments.
    //
    // This should return `()` if authentication and authorization checks have
    // been passed and return an error (or panic) otherwise.
    //
    // `__check_auth` takes the payload that needed to be signed, arbitrarily
    // typed signatures (`Signature` contract type here) and authorization
    // context that contains all the invocations that this call tries to verify.
    //
    // `__check_auth` has to authenticate the signatures. It also may use
    // `auth_context` to implement additional authorization policies (like token
    // spend limits here).
    //
    // Soroban host guarantees that `__check_auth` is only being called during
    // `require_auth` verification and hence this may mutate its own state
    // without the need for additional authorization (for example, this could
    // store per-time-period token spend limits instead of just enforcing the
    // limit per contract call).
    //
    // Note, that `__check_auth` function shouldn't call `require_auth` on the
    // contract's own address in order to avoid infinite recursion.
    #[allow(non_snake_case)]
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<Signature>,
        auth_context: Vec<Context>,
    ) -> Result<(), AccError> {
        // Perform authentication.
        authenticate(env(), &self.signers, &signature_payload, &signatures)?;

        let tot_signers: u32 = &self.count;
        let all_signed = tot_signers == signatures.len();

        let curr_contract_id = env().current_contract_id();

        // This is a map for tracking the token spend limits per token. This
        // makes sure that if e.g. multiple `transfer` calls are being authorized
        // for the same token we still respect the limit for the total
        // transferred amount (and not the 'per-call' limits).
        let mut spend_left_per_token = Map::<BytesN<32>, i128>::new(&env);
        // Verify the authorization policy.
        for context in auth_context.iter() {
            verify_authorization_policy(
                &env,
                &self.limits,
                &context.unwrap(),
                &curr_contract_id,
                all_signed,
                &mut spend_left_per_token,
            )?;
        }
        Ok(())
    }
}

fn authenticate(
    env: &Env,
    signers: Map<BytesN<32>, i128>, 
    signature_payload: &BytesN<32>,
    signatures: &Vec<Signature>,
) -> Result<(), AccError> {
    for i in 0..signatures.len() {
        let signature: Signature = signatures.get_unchecked(i).unwrap();
        if i > 0 {
            let prev_signature = signatures.get_unchecked(i - 1).unwrap();
            if prev_signature.public_key >= signature.public_key {
                return Err(AccError::BadSignatureOrder);
            }
        }
        if signers.has(signature.public_key.clone())
        {
            return Err(AccError::UnknownSigner);
        }
        env.crypto().ed25519_verify(
            &signature.public_key,
            &signature_payload.clone().into(),
            &signature.signature,
        );
    }
    Ok(())
}

fn verify_authorization_policy(
    env: &Env,
    limits: Map<BytesN<32>, i128>, 
    context: &Context,
    curr_contract_id: &BytesN<32>,
    all_signed: bool,
    spend_left_per_token: &mut Map<BytesN<32>, i128>,
) -> Result<(), AccError> {
    // For the account control every signer must sign the invocation.
    if &context.contract == curr_contract_id {
        if !all_signed {
            return Err(AccError::NotEnoughSigners);
        }
    }

    // Otherwise, we're only interested in functions that spend tokens.
    if context.fn_name != TRANSFER_FN && context.fn_name != Symbol::new(env, "increase_allowance") {
        return Ok(());
    }

    let spend_left: Option<i128> =
        if let Some(spend_left) = spend_left_per_token.get(context.contract.clone()) {
            Some(spend_left.unwrap())
        } else if let Some(limit_left) = limits
            .get(context.contract.clone())
        {
            Some(limit_left.unwrap())
        } else {
            None
        };

    // 'None' means that the contract is outside of the policy.
    if let Some(spend_left) = spend_left {
        // 'amount' is the third argument in both `approve` and `transfer`.
        // If the contract has a different signature, it's safer to panic
        // here, as it's expected to have the standard interface.
        let spent: i128 = context
            .args
            .get(2)
            .unwrap()
            .unwrap()
            .try_into_val(env)
            .unwrap();
        if spent < 0 {
            return Err(AccError::NegativeAmount);
        }
        if !all_signed && spent > spend_left {
            return Err(AccError::NotEnoughSigners);
        }
        spend_left_per_token.set(context.contract.clone(), spend_left - spent);
    }
    Ok(())
}
