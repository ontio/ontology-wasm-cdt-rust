#![cfg_attr(not(feature="mock"), no_std)]
//#![no_std]
extern crate ontio_std as ostd;

use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const TOTAL_SUPPLY: u64 = 100000000000;
const KEY_BALANCE : &'static str = "b";
const KEY_APPROVE : &'static str = "a";

#[ostd::abi_codegen::contract]
pub trait MyToken {
    fn initialize(&mut self, owner: Address) -> bool;
    fn name(&self) -> String;
    fn balance_of(&self, owner: Address) -> U256;
    fn transfer(&mut self, from: Address, to: Address, amount: U256) -> bool;
    fn approve(&mut self, approves: Address, receiver: Address, amount:U256) -> bool;
    fn transfer_from(&mut self, receiver:Address,approves:Address, amount:U256) -> bool;
    fn allowance(&mut self, approves:Address, receiver:Address) -> U256;
    fn total_supply(&self) -> U256;

    #[event]
    fn Transfer(&self, from: Address, to: Address, amount: U256) {}
    #[event]
    fn Approve(&self, approves:Address, receiver:Address, amount:U256) {}
}

pub(crate) struct MyTokenInstance;

impl MyToken for MyTokenInstance {
    fn initialize(&mut self, owner:Address) -> bool {
        if database::get::<_, U256>(KEY_TOTAL_SUPPLY).is_some() {
            return false
        }
        database::put(KEY_TOTAL_SUPPLY, U256::from(TOTAL_SUPPLY));
        database::put(&utils::gen_balance_key(&owner), U256::from(TOTAL_SUPPLY));
        true
    }

    fn name(&self) ->String {
        "wasm_token".to_string()
    }

    fn balance_of(&self, owner: Address) -> U256 {
        database::get(&utils::gen_balance_key(&owner)).unwrap_or(U256::zero())
    }

    fn transfer(&mut self, from: Address, to: Address, amount: U256) -> bool {
        if runtime::check_witness(&from) == false {
            return false;
        }
        let mut frmbal = self.balance_of(from.clone());
        let mut tobal = self.balance_of(to.clone());
        if amount == 0.into() || frmbal < amount {
            false
        } else {
            frmbal = frmbal - amount;
            tobal = tobal + amount;
            database::put(&utils::gen_balance_key(&from), &frmbal);
            database::put(&utils::gen_balance_key(&to), &tobal);
            self.Transfer(from, to, amount);
            true
        }
    }
    fn approve(&mut self, approves: Address, receiver: Address, amount: U256) -> bool {
        if runtime::check_witness(&approves) == false {
            return false;
        }
        let apprbal = self.balance_of(approves.clone());
        if apprbal < amount {
            return false;
        } else {
            database::put(&utils::gen_approve_key(&approves, &receiver), amount);
            self.Approve(approves, receiver, amount);
            true
        }
    }
    fn transfer_from(&mut self, receiver: Address, approves: Address, amount: U256) -> bool {
        if runtime::check_witness(&receiver) == false {
            return false;
        }
        let mut allow = self.allowance(approves.clone(), receiver.clone());
        if allow < amount {
            return false;
        }
        let mut approbal = self.balance_of(approves.clone());
        if approbal < amount {
            return false;
        }
        let mut receivbal = self.balance_of(receiver.clone());
        receivbal = receivbal + amount;
        approbal = approbal - amount;
        allow = allow - amount;
        database::put(utils::gen_approve_key(&approves, &receiver), allow);
        database::put(utils::gen_balance_key(&approves), approbal);
        database::put(utils::gen_balance_key(&receiver), receivbal);
        true
    }
    fn allowance(&mut self, approves:Address, receiver:Address) -> U256 {
        database::get(&utils::gen_approve_key(&approves, &receiver)).unwrap_or(U256::zero())
    }
    fn total_supply(&self) -> U256 {
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
    pub fn gen_approve_key(approves:&Address, receiver:&Address) -> Vec<u8> {
        let mut key:Vec<u8> = KEY_APPROVE.as_bytes().to_vec();
        key.extend_from_slice(approves.as_ref());
        key.extend_from_slice(receiver.as_ref());
        key
    }
}

#[cfg(test)]
mod test;