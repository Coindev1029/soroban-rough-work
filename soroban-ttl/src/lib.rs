#![no_std]
/// This is a simple contract that just extends TTL for its keys.
/// It's main purpose is to demonstrate how TTL extension can be tested,
/// so the most interesting part here is `test.rs`.
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
pub enum DataKey {
    MyKey,
}

#[contract]
pub struct TtlContract;


pub(crate) const DAY_IN_LEDGERS: u32 = 17280;

pub(crate) const SHARED_BUMP_AMOUNT: u32 = 31 * DAY_IN_LEDGERS;
pub(crate) const SHARED_LIFETIME_THRESHOLD: u32 = SHARED_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contractimpl]
impl TtlContract {
    /// Creates a contract entry in every kind of storage.
    pub fn setup(env: Env) {
        env.storage().persistent().set(&DataKey::MyKey, &0);
        // env.storage().instance().set(&DataKey::MyKey, &1);
        // env.storage().temporary().set(&DataKey::MyKey, &2);
    }

    pub fn test1() {
        let _a = "onelove";
        // env.storage().instance().set(&DataKey::MyKey, &1);
        // env.storage().temporary().set(&DataKey::MyKey, &2);
    }


    // /// Extend the persistent entry TTL to 5000 ledgers, when its
    // /// TTL is smaller than 1000 ledgers.
    // pub fn extend_persistent(env: Env) {
    //     env.storage()
    //         .persistent()
    //         .extend_ttl(&DataKey::MyKey, 1000, 5000);
    // }

    /// Extend the instance entry TTL to become at least 10000 ledgers,
    /// when its TTL is smaller than 2000 ledgers.
    // pub fn extend_instance(env: Env) {
        // env.storage().instance().extend_ttl(2000, 10000);
    // }

    /// Extend the temporary entry TTL to become at least 7000 ledgers,
    /// when its TTL is smaller than 3000 ledgers.
    pub fn extend_temporary(env: Env) {
        env.storage()
            .temporary()
            .extend_ttl(&DataKey::MyKey, SHARED_LIFETIME_THRESHOLD, SHARED_BUMP_AMOUNT);
    }

    
}
