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

impl Default for SimpleAccountManager {
    fn default() -> Self {
        SimpleAccountManager {
            owner: BytesN::from_array(env(),&[0; 32]),
        }
    }
}
#[subcontract]
pub trait IsSimpleAccount {
    fn init(&mut self, public_key: BytesN<32>) -> Result<(), SimpleAccError>;
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        auth_context: Vec<Context>,
    ) -> Result<(), SimpleAccError>;
}

impl IsSimpleAccount for SimpleAccountManager {
    fn init(&mut self, public_key: BytesN<32>) -> Result<(), SimpleAccError> {
        if !(self.owner ==  BytesN::from_array(env(),&[0; 32])){
            return Err(SimpleAccError::OwnerAlreadySet);
        }
        self.owner = public_key;
        Ok(())
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
        
        env().crypto().ed25519_verify(
            &self.owner,
            &signature_payload.into(),
            &signatures.get(0).unwrap(),
        );
        
        Ok(())
    }
}
