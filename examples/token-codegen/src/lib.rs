#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::Dispatcher;
use ostd::types::{Address, U256};
use ostd::{database, runtime};
use ostd::{string::ToString, String};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const TOTAL_SUPPLY: u64 = 100000000000;

#[ostd::abi_codegen::contract]
pub trait MyToken {
    fn initialize(&mut self) -> bool;
    fn name(&self) -> String;
    fn balance_of(&self, owner: Address) -> U256;
    fn transfer(&mut self, from: Address, to: Address, amount: U256) -> bool;
    fn total_supply(&self) -> U256;

    #[event]
    fn Transfer(&self, from: Address, to: Address, amount: U256);
}

struct MyTokenInstance;

impl MyToken for MyTokenInstance {
    fn initialize(&mut self) -> bool {
        database::put(KEY_TOTAL_SUPPLY, U256::from(TOTAL_SUPPLY));
        true
    }

    fn name(&self) ->String {
        "wasm_token".to_string()
    }

    fn balance_of(&self, owner: Address) -> U256 {
        database::get(&owner).unwrap_or(U256::zero())
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
            database::put(&from, frmbal);
            database::put(&to, tobal);
            self.Transfer(from, to, amount);
            true
        }
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
