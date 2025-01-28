use loam_sdk::{
    loamstorage, log,
    soroban_sdk::{self, env, Address, LoamKey, PersistentMap, String},
};

use crate::{
    error::Error,
    metadata::{ContractMetadata, PublishedContract, PublishedWasm},
    version::{self, Version, INITAL_VERSION},
};

use super::IsPublishable;

#[loamstorage]

pub struct Wasm {
    registry: PersistentMap<String, PublishedContract>,
}

impl Wasm {
    pub fn find_contract(&self, name: String) -> Result<PublishedContract, Error> {
        self.registry
            .get(name)
            .ok_or(Error::NoSuchContractPublished)
    }

    pub fn find_version(
        &self,
        name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error> {
        self.find_contract(name)?.get(version)
    }

    pub fn set_contract(&mut self, name: String, contract: &PublishedContract) {
        self.registry.set(name, contract);
    }
}

impl IsPublishable for Wasm {
    fn fetch(
        &self,
        contract_name: String,
        version: Option<Version>,
    ) -> Result<PublishedWasm, Error> {
        self.find_version(contract_name, version)
    }

    fn current_version(&self, contract_name: String) -> Result<Version, Error> {
        self.find_contract(contract_name)?.most_recent_version()
    }

    fn publish(
        &mut self,
        wasm_name: String,
        author: Address,
        wasm: soroban_sdk::Bytes,
        repo: Option<String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error> {
        let wasm_hash = env().deployer().upload_contract_wasm(wasm);
        self.publish_hash(wasm_name, author, wasm_hash, repo, kind)
    }

    fn publish_hash(
        &mut self,
        wasm_name: soroban_sdk::String,
        author: soroban_sdk::Address,
        wasm_hash: soroban_sdk::BytesN<32>,
        repo: Option<soroban_sdk::String>,
        kind: Option<version::Update>,
    ) -> Result<(), Error> {
        let mut contract = self
            .find_contract(wasm_name.clone())
            .unwrap_or_else(|_| PublishedContract::new(author.clone()));

        if author != contract.author {
            return Err(Error::AlreadyPublished);
        }
        contract.author.require_auth();

        let keys = contract.versions.keys();
        let last_version = keys.last().unwrap_or_default();
        last_version.log();
        let new_version = last_version.clone().update(&kind.unwrap_or_default());
        new_version.log();

        let metadata = if let Some(repo) = repo {
            ContractMetadata { repo }
        } else if new_version == INITAL_VERSION {
            ContractMetadata::default()
        } else {
            contract.get(Some(last_version))?.metadata
        };
        let published_binary = PublishedWasm { hash: wasm_hash, metadata };
        contract.versions.set(new_version, published_binary);
        self.set_contract(wasm_name, &contract);
        Ok(())
    }
}
