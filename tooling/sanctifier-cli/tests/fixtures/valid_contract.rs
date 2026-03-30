#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct CleanContract;

#[contractimpl]
impl CleanContract {
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        env.storage().persistent().set(&to, &amount);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&id).unwrap_or(0)
    }
}