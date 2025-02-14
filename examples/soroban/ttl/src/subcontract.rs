use loam_sdk::soroban_sdk::{self, env, InstanceItem, Lazy, PersistentItem, TemporaryItem};
use loam_sdk::{loamstorage, subcontract};

#[loamstorage]
pub struct TtlContract {
    p: PersistentItem<u32>,
    i: InstanceItem<u32>,
    t: TemporaryItem<u32>,
}

#[subcontract]
pub trait IsTtl {
    /// Creates a contract entry in every kind of storage.
    fn setup(&mut self);
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
    fn setup(&mut self) {
        self.p.set(&0);
        self.i.set(&1);
        self.t.set(&2);
    }

    /// Extend the persistent entry TTL to 5000 ledgers, when its
    /// TTL is smaller than 1000 ledgers.
    fn extend_persistent(&self) {
        self.p.extend_ttl(1000, 5000);
    }

    /// Extend the instance entry TTL to become at least 10000 ledgers,
    /// when its TTL is smaller than 2000 ledgers.
    fn extend_instance(&self) {
        self.i.extend_ttl(2000, 10000);
    }

    /// Extend the temporary entry TTL to become at least 7000 ledgers,
    /// when its TTL is smaller than 3000 ledgers.
    fn extend_temporary(&self) {
        self.t.extend_ttl(3000, 7000);
    }
}
