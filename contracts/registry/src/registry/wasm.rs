use loam_sdk::soroban_sdk::{self, env, Address, String};

use crate::{
    error::Error,
    metadata::{Metadata, PublishedWasm, Wasm},
    version::{self, Version, INITAL_VERSION},
};

use super::IsPublishable;

impl IsPublishable for PublishedWasm {
    fn fetch(&self, contract_name: String, version: Option<Version>) -> Result<Wasm, Error> {
        self.get(&contract_name, version)
    }

    fn current_version(&self, contract_name: String) -> Result<Version, Error> {
        self.most_recent_version(&contract_name)
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
        if let Some(cunnet_author) = self.author(&wasm_name) {
            if author != cunnet_author {
                return Err(Error::AlreadyPublished);
            }
        }

        author.require_auth();

        let last_version: Version = self.most_recent_version(&wasm_name).unwrap_or_default();
        last_version.log();
        let new_version = last_version.clone().update(&kind.unwrap_or_default());
        new_version.log();

        let metadata = if let Some(repo) = repo {
            Metadata { repo }
        } else if new_version == INITAL_VERSION {
            Metadata::default()
        } else {
            self.get(&wasm_name, Some(last_version))?.metadata
        };
        let published_binary = Wasm {
            hash: wasm_hash,
            metadata,
        };
        self.set_most_recent_version(&wasm_name, new_version.clone());
        self.set(wasm_name, Some(new_version), published_binary)
    }
}
