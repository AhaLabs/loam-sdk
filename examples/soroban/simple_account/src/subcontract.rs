use loam_sdk::{
    loamstorage, soroban_sdk::{self, auth::Context, env, BytesN, InstanceItem, Lazy, Vec}, subcontract,
};

use crate::error::Error;

#[loamstorage]
pub struct SimpleAccountManager {
    owner: InstanceItem<BytesN<32>>,
}

#[subcontract]
pub trait IsSimpleAccount {
    fn init(&mut self, public_key: BytesN<32>) -> Result<(), Error>;
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        auth_context: Vec<Context>,
    ) -> Result<(), Error>;
}

impl IsSimpleAccount for SimpleAccountManager {
    fn init(&mut self, public_key: BytesN<32>) -> Result<(), Error> {
        if self.owner.get().is_some() {
            return Err(Error::OwnerAlreadySet);
        };
        self.owner.set(&public_key);
        Ok(())
    }

    #[allow(non_snake_case)]
    fn __check_auth(
        &self,
        signature_payload: BytesN<32>,
        signatures: Vec<BytesN<64>>,
        _auth_context: Vec<Context>,
    ) -> Result<(), Error> {
        if signatures.len() != 1 {
            return Err(Error::IncorrectSignatureCount);
        }

        env().crypto().ed25519_verify(
            &self.owner.get().unwrap(),
            &signature_payload.into(),
            &signatures.get(0).unwrap(),
        );

        Ok(())
    }
}
