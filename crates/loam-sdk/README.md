# loam-sdk

Build **composable**, **upgradeable**, **secure** Smart Contracts.

- **Composable**: With Loam SDK, you compose your smart contract from many _sub_ contracts. Subcontracts are like lego blocks that you can either use off-the-shelf from the open source ecosystem or that you can build yourself. A single Loam _smart_ contract is composed of one or more subcontracts.
- **Upgradeable**: The one subcontract that all Loam smart contracts must include ([loam-subcontract-core](../loam-subcontract-core)) adds an important method to the smart contract: `redeploy`. You can call this method to switch the `wasm` hash—the behavior/brains of the contract—to a new one, while keeping the same contract ID. The storage accessed by each particular subcontract is loaded lazily, so upgrading one subcontract does not require migrating the data of another; each subcontract within your smart contract can be considered and upgraded independently.
- **Secure**: The [core subcontract](../loam-subcontract-core) also adds `admin_set` and `admin_get` to your contract, to make sure that only your trusted admin account can call `redeploy`. Our full loam architecture, beyond Loam SDK, also includes a universal factory contract, which makes it possible to deploy your contract and call `admin_set` in a single transaction, helping avoid front-running.

- [Subcontracts](#subcontracts)
    - [Creating Contract Subcontracts](#creating-contract-subcontracts)
    - [External API](#external-api)
- [Core](#core-subcontract)
-   [Using the Core Subcontract](#using-the-core-subcontract)


# Subcontracts

A subcontract is a type that implements the `Lazy` trait, which is used for lazily loading and storing the type. Use the `loamstorage` macro along with special `loam` storage types - `PersistentMap`, `InstanceMap`, `TemporaryMap` (for key-value pair storage); `PersistentItem`, `InstanceItem`, `TemporaryItem` (for singleton storage). These map to `soroban` `Persistent`, `Instance`, and `Temporary` storage types.

## Creating  Subcontracts

Here's an example of how to create a subcontract:

```rust
#[loamstorage]
#[derive(Default)]
pub struct Token { 
    balance: PersistentMap<Address, i128>,
    symbol: InstanceItem<String>,
};
```

This generates an implementation to access the `Persistent` and `Instance` storage 
mechanisms of `env.storage()`. These can be accessed via `balance.set`, `balance.get`, etc.
Time to live can be extended via `balance.extend_ttl`. 

## External API

You can also create and implement external APIs for contract subcontracts:

```rust
#[subcontract]
pub trait IsPostable {
    fn messages_get(&self, author: Address) -> Option<String>;
    fn messages_set(&mut self, author: Address, text: String);
}
```

# Core Subcontract

The `Core` trait provides the minimum logic needed for a contract to be redeployable. A contract should be able to be redeployed to another contract that can also be redeployed. Redeployment requires admin status, as it would be undesirable for an account to redeploy the contract without permission.

## Using  `Core`

To use the core subcontract, create a `Contract` struct and implement `Core` with the `Admin` implementation, which ensures the contract is redeployable and will continue to be redeployable if the new contract also implements `Core`. After `Core` other Subcontracts can be added as needed.

```rust
use loam_sdk::derive_contract;
use loam_subcontract_core::{Admin, Core};

#[derive_contract(Core(Admin))]
pub struct Contract;

```

This code generates the following implementation:

```rust
struct SorobanContract;

#[contractimpl]
impl SorobanContract {
     pub fn admin_set(env: Env, admin: Address) {
        set_env(env);
        Contract::owner_set(owner);
    }
    pub fn admin_get(env: Env) -> Option<Address> {
        set_env(env);
        Contract::admin_get()
    }
    pub fn redeploy(env: Env, wasm_hash: BytesN<32>) {
        set_env(env);
        Contract::redeploy(wasm_hash);
    }
    // Subcontract methods would be inserted here.
    // Contract must implement all Subcontracts and is the proxy for the contract calls.
    // This is because the Subcontracts have default implementations which call the associated type
}
```

By specifying the associated a concrete implementation for `Core`, `Admin`, you enable its methods to be used (`admin_set`, `admin_get`, `redeploy`). However, you can also provide a different implementation if needed by replacing `Admin` with a different struct/enum that also implements [IsCore](replace).

Notice that the generated code includes `Contract::redeploy` and other methods. This ensures that the `Contract` type is redeployable, while also allowing for extensions, as different concrete implementation can overwrite the default methods.
