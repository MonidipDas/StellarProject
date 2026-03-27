#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct SimpleStorage;

#[contractimpl]
impl SimpleStorage {

    // Store a value
    pub fn set(env: Env, key: Symbol, value: i128) {
        env.storage().instance().set(&key, &value);
    }

    // Get a value
    pub fn get(env: Env, key: Symbol) -> i128 {
        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(0)
    }
}