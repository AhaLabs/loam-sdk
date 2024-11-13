use loam_sdk::{
    soroban_sdk::{self, auth::Context, contracttype, env, BytesN, Lazy, Vec},
    subcontract, IntoKey
};

use crate::error::SimpleAccError;

#[contracttype]
#[derive(IntoKey)]
pub struct SimpleAccountManager {
    owner: BytesN<32>,
}

#[subcontract]
pub trait IsSimpleAccount {
    fn init(&self, public_key: BytesN<32>);
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        auth_context: Vec<Context>,
    ) -> Result<(), SimpleAccError>;
}

impl IsSimpleAccount for SimpleAccountManager {
    fn init(&self, public_key: BytesN<32>) {
        if self.owner.has() {
            return Err(SimpleAccError::OwnerAlreadySet);
        }
        self.owner.set(&public_key);
    }

    #[allow(non_snake_case)]
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        _auth_context: Vec<Context>,
    ) -> Result<(), SimpleAccError> {
        if signatures.len() != 1 {
            return Err(SimpleAccError::IncorrectSignatureCount);
        }
        
        let public_key = self.owner.get().unwrap();
        env().crypto().ed25519_verify(
            &public_key,
            &signature_payload.into(),
            &signatures.get(0).unwrap().unwrap(),
        );
        
        Ok(())
    }
}
