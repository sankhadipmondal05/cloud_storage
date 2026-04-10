#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, String, Address, Map};

#[contract]
pub struct CloudStorageContract;

#[contractimpl]
impl CloudStorageContract {

    // Store data for a user
    pub fn upload(env: Env, user: Address, key: Symbol, value: String) {
        // Authenticate user
        user.require_auth();

        let mut storage: Map<Symbol, String> =
            env.storage().persistent().get(&user).unwrap_or(Map::new(&env));

        storage.set(key, value);

        env.storage().persistent().set(&user, &storage);
    }

    // Retrieve data
    pub fn get(env: Env, user: Address, key: Symbol) -> String {
        let storage: Map<Symbol, String> =
            env.storage().persistent().get(&user).unwrap();

        storage.get(key).unwrap()
    }

    // Delete stored data
    pub fn delete(env: Env, user: Address, key: Symbol) {
        user.require_auth();

        let mut storage: Map<Symbol, String> =
            env.storage().persistent().get(&user).unwrap();

        storage.remove(key);

        env.storage().persistent().set(&user, &storage);
    }
}