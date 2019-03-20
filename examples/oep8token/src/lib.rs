#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::Dispatcher;
use ostd::abi::Encoder;
use ostd::prelude::*;
use ostd::{database, runtime};

const INITED: &'static str = "Initialized";
//const TOKEN_ID_LIST: [u8; 5]= [1,2,3,4,5];
const NAME: &'static str = "name";
const SYMBOL: &'static str = "Symbol";
const BALANCE: &'static str = "Balance";
const TOTAL_SUPPLY: &'static str = "TotalSupply";
const APPROVE: &'static str = "Approve";
const ADMIN: Address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");

#[ostd::abi_codegen::contract]
pub trait Oep8Token {
    fn init(&mut self) -> bool;
    fn name(&self, token_id: String) -> String;
    fn symbol(&self, token_id: String) -> String;
    fn total_supply(&self, token_id: String) -> U256;
    fn balance_of(&self, address: &Address, token_id: String) -> U256;
    fn transfer(&mut self, from: &Address, to: &Address, amount: U256, token_id: String) -> bool;
    fn transfer_multi(&mut self, states: &[(Address, Address, U256, String)]) -> bool;
    fn approve(
        &mut self, owner: &Address, spender: &Address, amount: U256, token_id: String,
    ) -> bool;
    fn allowance(&mut self, owner: &Address, spender: &Address, token_id: String) -> U256;
    fn transfer_from(
        &mut self, spender: &Address, from: &Address, to: &Address, amount: U256, token_id: String,
    ) -> bool;
    fn approve_multi(&mut self, obj: &[(Address, Address, U256, String)]) -> bool;
    fn transfer_from_multi(&mut self, obj: &[(Address, Address, Address, U256, String)]) -> bool;
    //optional
    fn create_multi_type_token(&mut self) -> bool;
    fn check_token_id(&self, token_id: String) -> bool;
    #[event]
    fn Transfer(&self, from: &Address, to: &Address, amount: U256, token_id: String) {}
    #[event]
    fn Approve(&self, approves: &Address, receiver: &Address, amount: U256, token_id: String) {}
}

pub(crate) struct Oep8TokenInstance;

impl Oep8Token for Oep8TokenInstance {
    fn init(&mut self) -> bool {
        if database::get::<_, bool>(INITED).unwrap_or_default() == true {
            return false;
        } else {
            assert!(runtime::check_witness(&ADMIN));
            assert!(self.create_multi_type_token());
            database::put(INITED, true)
        }
        true
    }
    fn name(&self, token_id: String) -> String {
        database::get(&utils::concat(token_id, NAME)).unwrap_or_default()
    }
    fn symbol(&self, token_id: String) -> String {
        database::get(&utils::concat(token_id, SYMBOL)).unwrap_or_default()
    }
    fn total_supply(&self, token_id: String) -> U256 {
        database::get(&utils::concat(token_id, TOTAL_SUPPLY)).unwrap_or_default()
    }
    fn balance_of(&self, address: &Address, token_id: String) -> U256 {
        database::get(&utils::concat(token_id, (BALANCE, address))).unwrap_or_default()
    }
    fn transfer(&mut self, from: &Address, to: &Address, amount: U256, token_id: String) -> bool {
        assert!(U256::zero() < amount);
        assert_eq!(self.check_token_id(token_id.clone()), true);
        assert!(runtime::check_witness(from));
        let balance_key = utils::concat(token_id.clone(), BALANCE);
        let from_key = utils::concat(&balance_key, from);
        let from_balance = database::get(&from_key).unwrap_or(U256::zero());
        if amount > from_balance {
            return false;
        }
        if amount == from_balance {
            database::delete(from_key);
        } else {
            database::put(from_key, from_balance - amount)
        }
        let to_key = utils::concat(&balance_key, to);
        let to_balance = database::get(&to_key).unwrap_or(U256::zero());
        database::put(&to_key, to_balance + amount);
        self.Transfer(&from, &to, amount, token_id);
        true
    }
    fn transfer_multi(&mut self, states: &[(Address, Address, U256, String)]) -> bool {
        if states.is_empty() {
            return false;
        }
        for state in states.iter() {
            if self.transfer(&state.0, &state.1, state.2, state.3.clone()) == false {
                panic!(
                    "transfer failed, from:{},to:{},amount:{},token_id:{}",
                    state.0, state.1, state.2, state.3
                );
            }
        }
        true
    }
    fn approve(
        &mut self, owner: &Address, spender: &Address, amount: U256, token_id: String,
    ) -> bool {
        assert_eq!(runtime::check_witness(owner), true);
        assert_eq!(self.check_token_id(token_id.clone()), true);
        let owner_balance = self.balance_of(owner, token_id.clone());
        assert_eq!(owner_balance >= amount, true);
        assert_eq!(amount > U256::zero(), true);
        let approve_key = utils::concat(token_id.clone(), (APPROVE, owner, spender));
        database::put(&approve_key, amount);
        self.Approve(owner, spender, amount, token_id);
        true
    }
    fn allowance(&mut self, owner: &Address, spender: &Address, token_id: String) -> U256 {
        let approve_key = utils::concat(token_id, (APPROVE, owner, spender));
        database::get(&approve_key).unwrap_or(U256::zero())
    }
    fn transfer_from(
        &mut self, spender: &Address, from: &Address, to: &Address, amount: U256, token_id: String,
    ) -> bool {
        assert!(amount > U256::zero());
        assert_eq!(runtime::check_witness(spender), true);
        let approval = self.allowance(from, spender, token_id.clone());
        assert!(amount <= approval);
        let fromval = self.balance_of(from, token_id.clone());
        database::put(&utils::concat(token_id.clone(), (BALANCE, from)), fromval - amount);
        let toval = self.balance_of(to, token_id.clone());
        database::put(&utils::concat(token_id.clone(), (BALANCE, to)), toval + amount);
        let approve_key = utils::concat(token_id, (APPROVE, from, spender));
        database::put(&approve_key, approval - amount);
        true
    }
    fn approve_multi(&mut self, obj: &[(Address, Address, U256, String)]) -> bool {
        if obj.is_empty() {
            return false;
        }
        for o in obj.iter() {
            if self.approve(&o.0, &o.1, o.2, o.3.clone()) == false {
                panic!(
                    "approve_multi failed! from:{}, to:{}, amount: {},token_id:{}",
                    &o.0, &o.1, o.2, o.3
                );
            }
        }
        true
    }
    fn transfer_from_multi(&mut self, obj: &[(Address, Address, Address, U256, String)]) -> bool {
        if obj.is_empty() {
            return false;
        }
        for o in obj.iter() {
            if self.transfer_from(&o.0, &o.1, &o.2, o.3, o.4.clone()) == false {
                panic!(
                    "transfer_from_multi failed, spender:{}, from:{}, to:{}, amount:{},token_id:{}",
                    &o.0, &o.1, &o.2, o.3, o.4
                );
            }
        }
        true
    }

    //optional
    fn create_multi_type_token(&mut self) -> bool {
        let token_name_list = [
            "TokenNameFirst",
            "TokenNameSecond",
            "TokenNameThird",
            "TokenNameFourth",
            "TokenNameFifth",
        ];
        let token_symbol_list = ["TNF", "TNS", "TNH", "TNO", "TNI"];
        let token_supply_list = [
            U256::from(100000),
            U256::from(200000),
            U256::from(300000),
            U256::from(400000),
            U256::from(500000),
        ];
        for index in 0..5 {
            let token_name = token_name_list[index];
            let token_symbol = token_symbol_list[index];
            let token_total_supply = token_supply_list[index];
            let token_id = format!("{}", index + 1);
            database::put(&utils::concat(token_id.clone(), NAME), token_name);
            database::put(&utils::concat(token_id.clone(), SYMBOL), token_symbol);
            database::put(&utils::concat(token_id.clone(), TOTAL_SUPPLY), token_total_supply);
            database::put(&utils::concat(token_id.clone(), (BALANCE, ADMIN)), token_total_supply);
            self.Transfer(&ADMIN, &ADMIN, token_total_supply, token_id);
        }
        true
    }
    fn check_token_id(&self, token_id: String) -> bool {
        if database::get::<_, String>(&utils::concat(token_id, NAME)).is_some() == false {
            return false;
        }
        true
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher = Oep8TokenDispatcher::new(Oep8TokenInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

mod utils {
    use super::*;
    pub fn concat<K: AsRef<[u8]>, T: Encoder>(prefix: K, key: T) -> Vec<u8> {
        let mut sink = ostd::abi::Sink::new(1);
        sink.write(key);
        [prefix.as_ref(), sink.bytes()].concat()
    }
}

#[cfg(test)]
mod test;
