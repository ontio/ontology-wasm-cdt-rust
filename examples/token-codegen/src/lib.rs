#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use ostd::abi::Dispatcher;
use ostd::prelude::*;
use ostd::{database, runtime};

const _ADDR_EMPTY: Address = ostd::macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");

const KEY_TOTAL_SUPPLY: &str = "total_supply";
const TOTAL_SUPPLY: U128 = U128::new(100_000_000_000);
const KEY_BALANCE: &str = "b";
const KEY_APPROVE: &str = "a";

#[ostd::macros::contract]
pub trait MyToken {
    fn initialize(&mut self, owner: &Address) -> bool;
    fn name(&self) -> String;
    fn balance_of(&self, owner: &Address) -> U128;
    fn transfer(&mut self, from: &Address, to: &Address, amount: U128) -> bool;
    fn transfer_multi(&mut self, states: &[(Address, Address, U128)]) -> bool;
    fn approve(&mut self, approves: &Address, receiver: &Address, amount: U128) -> bool;
    fn transfer_from(&mut self, receiver: &Address, approves: &Address, amount: U128) -> bool;
    fn allowance(&mut self, approves: &Address, receiver: &Address) -> U128;
    fn total_supply(&self) -> U128;

    #[event]
    fn Transfer(&self, from: &Address, to: &Address, amount: U128) {}
    #[event]
    fn Approve(&self, approves: &Address, receiver: &Address, amount: U128) {}
}

pub(crate) struct MyTokenInstance;

impl MyToken for MyTokenInstance {
    fn initialize(&mut self, owner: &Address) -> bool {
        if database::get::<_, U128>(KEY_TOTAL_SUPPLY).is_some() {
            return false;
        }
        database::put(KEY_TOTAL_SUPPLY, TOTAL_SUPPLY);
        database::put(&utils::gen_balance_key(owner), TOTAL_SUPPLY);
        true
    }

    fn name(&self) -> String {
        "wasm_token".to_string()
    }

    fn balance_of(&self, owner: &Address) -> U128 {
        database::get(&utils::gen_balance_key(owner)).unwrap_or_default()
    }

    fn transfer(&mut self, from: &Address, to: &Address, amount: U128) -> bool {
        if !runtime::check_witness(from) {
            return false;
        }
        let mut frmbal = self.balance_of(from);
        let mut tobal = self.balance_of(to);
        if amount.is_zero() || frmbal < amount {
            false
        } else {
            frmbal -= amount;
            tobal += amount;
            database::put(&utils::gen_balance_key(from), &frmbal);
            database::put(&utils::gen_balance_key(to), &tobal);
            self.Transfer(from, to, amount);
            true
        }
    }
    fn transfer_multi(&mut self, states: &[(Address, Address, U128)]) -> bool {
        if states.is_empty() {
            return false;
        }
        for state in states.iter() {
            if !self.transfer(&state.0, &state.1, state.2) {
                panic!("transfer failed, from:{}, to:{}, amount:{}", state.0, state.1, state.2);
            }
        }
        true
    }

    fn approve(&mut self, approves: &Address, receiver: &Address, amount: U128) -> bool {
        if !runtime::check_witness(approves) {
            return false;
        }
        let apprbal = self.balance_of(approves);
        if apprbal < amount {
            false
        } else {
            database::put(&utils::gen_approve_key(approves, receiver), amount);
            self.Approve(approves, receiver, amount);
            true
        }
    }
    fn transfer_from(&mut self, receiver: &Address, approves: &Address, amount: U128) -> bool {
        if !runtime::check_witness(receiver) {
            return false;
        }
        let mut allow = self.allowance(approves, receiver);
        if allow < amount {
            return false;
        }
        let mut approbal = self.balance_of(approves);
        if approbal < amount {
            return false;
        }
        let mut receivbal = self.balance_of(receiver);
        receivbal += amount;
        approbal -= amount;
        allow -= amount;
        database::put(utils::gen_approve_key(approves, receiver), allow);
        database::put(utils::gen_balance_key(approves), approbal);
        database::put(utils::gen_balance_key(receiver), receivbal);
        true
    }
    fn allowance(&mut self, approves: &Address, receiver: &Address) -> U128 {
        database::get(&utils::gen_approve_key(approves, receiver)).unwrap_or_default()
    }

    fn total_supply(&self) -> U128 {
        database::get(KEY_TOTAL_SUPPLY).unwrap()
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher = MyTokenDispatcher::new(MyTokenInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

mod utils {
    use super::*;
    pub fn gen_balance_key(addr: &Address) -> Vec<u8> {
        [KEY_BALANCE.as_bytes(), addr.as_ref()].concat()
    }
    pub fn gen_approve_key(approves: &Address, receiver: &Address) -> Vec<u8> {
        let mut key: Vec<u8> = KEY_APPROVE.as_bytes().to_vec();
        key.extend_from_slice(approves.as_ref());
        key.extend_from_slice(receiver.as_ref());
        key
    }
}

#[cfg(test)]
mod test;
