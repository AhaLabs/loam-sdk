use loam_sdk::{
    loamstorage,
    soroban_sdk::{self, env, Address, LoamKey, PersistentMap, String},
};

//

use crate::{error::Error, version::Version};

use super::Wasm;
/// Contains
#[loamstorage]
pub struct PublishedWasm {
    pub versions: PersistentMap<(String, Version), Wasm>,
    pub author: PersistentMap<String, Address>,
    pub most_recent_version: PersistentMap<String, Version>,
}

impl PublishedWasm {
    pub fn new(name: &String, author: Address) -> Self {
        env().deployer().with_stellar_asset(serialized_asset)
        let mut s = Self::default();
        s.author.set(name.clone(), &author);
        s
    }
}

impl PublishedWasm {
    pub fn most_recent_version(&self, name: &String) -> Result<Version, Error> {
        self.most_recent_version
            .get(name.clone())
            .ok_or(Error::NoSuchVersion)
    }

    pub fn set_most_recent_version(&mut self, name: &String, version: Version) {
        self.most_recent_version.set(name.clone(), &version);
    }

    pub fn get(&self, name: &String, version: Option<Version>) -> Result<Wasm, Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version(name)?
        };
        self.versions
            .get((name.clone(), version))
            .ok_or(Error::NoSuchVersion)
    }

    pub fn set(
        &mut self,
        name: String,
        version: Option<Version>,
        binary: Wasm,
    ) -> Result<(), Error> {
        let version = if let Some(version) = version {
            version
        } else {
            self.most_recent_version(&name)?
        };
        self.versions.set((name, version), &binary);
        Ok(())
    }

    pub fn author(&self, name: &String) -> Option<Address> {
        self.author.get(name.clone())
    }
}
