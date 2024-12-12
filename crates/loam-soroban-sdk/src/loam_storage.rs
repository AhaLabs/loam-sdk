use core::marker::PhantomData;

use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

use crate::env;

pub trait LoamKey {
    fn to_key(&self) -> Val;
}

#[derive(Clone)]
pub struct PersistentMap<K, V, W = K>
where
    K: Into<W>,
    W: LoamKey,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    k: PhantomData<K>,
    v: PhantomData<V>,
    w: PhantomData<W>,
}

impl<K, V, W> PersistentMap<K, V, W>
where
    K: Into<W>,
    W: LoamKey,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    pub fn new() -> Self {
        Self {
            k: PhantomData,
            v: PhantomData,
            w: PhantomData,
        }
    }

    pub fn get(&self, key: K) -> Option<V> {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().get(&k)
    }

    pub fn set(&mut self, key: K, value: V) {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().set(&k, &value);
    }

    pub fn has(&self, key: K) -> bool {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().has(&k)
    }

    pub fn update(&self, key: K, f: impl FnOnce(Option<V>) -> V) -> V {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().update(&k, f)
    }

    pub fn try_update<E>(&self, key: K, f: impl FnOnce(Option<V>) -> Result<V, E>) -> Result<V, E> {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().try_update(&k, f)
    }

    pub fn extend_ttl(&self, key: K, threshold: u32, extend_to: u32) {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().extend_ttl(&k, threshold, extend_to)
    }

    pub fn remove(&self, key: K) {
        let w: W = key.into();
        let k = w.to_key();
        env().storage().persistent().remove(&k)
    }
}

impl<K, V, W> Default for PersistentMap<K, V, W>
where
    K: Into<W>,
    W: LoamKey,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct PersistentStore<V, K>
where
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    K: LoamKey + Default,
{
    v: PhantomData<V>,
    k: PhantomData<K>,
}

impl<V, K> PersistentStore<V, K>
where
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    K: LoamKey + Default,
{
    pub fn new() -> Self {
        Self { v: PhantomData, k: PhantomData }
    }

    pub fn get(&self) -> Option<V> {
        let key = K::default().to_key();
        env().storage().persistent().get(&key)
    }

    pub fn set(&mut self, value: V) {
        let key = K::default().to_key();
        env().storage().persistent().set(&key, &value);
    }

    pub fn has(&self) -> bool {
        let key = K::default().to_key();
        env().storage().persistent().has(&key)
    }

    pub fn update(&self, f: impl FnOnce(Option<V>) -> V) -> V {
        let key = K::default().to_key();
        env().storage().persistent().update(&key, f)
    }

    pub fn try_update<E>(&self, f: impl FnOnce(Option<V>) -> Result<V, E>) -> Result<V, E> {
        let key = K::default().to_key();
        env().storage().persistent().try_update(&key, f)
    }

    pub fn extend_ttl(&self, threshold: u32, extend_to: u32) {
        let key = K::default().to_key();
        env().storage().persistent().extend_ttl(&key, threshold, extend_to)
    }

    pub fn remove(&self) {
        let key = K::default().to_key();
        env().storage().persistent().remove(&key)
    }
}

impl<V, K> Default for PersistentStore<V, K>
where
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    K: LoamKey + Default,
{
    fn default() -> Self {
        Self::new()
    }
}