use loam_sdk::soroban_sdk::{self, env, contracttype, Lazy};
use loam_sdk::subcontract;

#[contracttype]
pub enum DataKey {
    MyKey,
}

#[derive(Lazy, Default)]
pub struct TtlContract;

#[subcontract]
pub trait IsTtl {
    /// Creates a contract entry in every kind of storage.
    fn setup(&self);
    /// Extend the persistent entry TTL to 5000 ledgers, when its
    /// TTL is smaller than 1000 ledgers.
    fn extend_persistent(&self);
    /// Extend the instance entry TTL to become at least 10000 ledgers,
    /// when its TTL is smaller than 2000 ledgers.
    fn extend_instance(&self);
    /// Extend the temporary entry TTL to become at least 7000 ledgers,
    /// when its TTL is smaller than 3000 ledgers.
    fn extend_temporary(&self);
}

impl IsTtl for TtlContract {
    /// Creates a contract entry in every kind of storage.
    fn setup(&self) {
        env().storage().persistent().set(&DataKey::MyKey, &0);
        env().storage().instance().set(&DataKey::MyKey, &1);
        env().storage().temporary().set(&DataKey::MyKey, &2);
    }

    /// Extend the persistent entry TTL to 5000 ledgers, when its
    /// TTL is smaller than 1000 ledgers.
    fn extend_persistent(&self) {
        env().storage()
            .persistent()
            .extend_ttl(&DataKey::MyKey, 1000, 5000);
    }

    /// Extend the instance entry TTL to become at least 10000 ledgers,
    /// when its TTL is smaller than 2000 ledgers.
    fn extend_instance(&self) {
        env().storage().instance().extend_ttl(2000, 10000);
    }

    /// Extend the temporary entry TTL to become at least 7000 ledgers,
    /// when its TTL is smaller than 3000 ledgers.
    fn extend_temporary(&self) {
        env().storage()
            .temporary()
            .extend_ttl(&DataKey::MyKey, 3000, 7000);
    }
}