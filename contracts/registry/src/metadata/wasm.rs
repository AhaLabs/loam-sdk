use loam_sdk::soroban_sdk::{self, contracttype, to_string, BytesN, String};



#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[contracttype]
pub struct Metadata {
    pub repo: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            repo: to_string(""),
        }
    }
}

/// Contains info about specific version of published binary
#[contracttype]
#[derive(Clone, Debug)]
pub struct Wasm {
    pub hash: BytesN<32>,
    pub metadata: Metadata,
}
