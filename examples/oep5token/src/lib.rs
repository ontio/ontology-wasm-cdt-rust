#![cfg_attr(not(feature = "mock"), no_std)]
extern crate ontio_std as ostd;
use ostd::abi::Dispatcher;
use ostd::abi::Encoder;
use ostd::prelude::*;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const INITED: &'static str = "Initialized";
const PREFIX_INDEX: &'static str = "01";
const PREFIX_APPROVE: &'static str = "03";
const PREFIX_OWNER: &'static str = "04";
const PREFIX_TOKEN_ID: &'static str = "05";
const PREFIX_BALANCE: &'static str = "06";

#[ostd::abi_codegen::contract]
pub trait Oep5Token {
    fn initialize(&mut self, owner: &Address) -> bool;
    fn name(&self) -> String;
    fn total_supply(&self) -> U256;
    fn query_token_id_by_index(&self, idx: U256) -> String;
    fn query_token_by_id(&self, token_id: String) -> String;
    fn balance_of(&self, address: &Address) -> U256;
    fn owner_of(&self, token_id: String) -> Address;
    fn transfer(&mut self, to: &Address, token_id: String) -> bool;
    fn transfer_multi(&mut self, states: &[(Address, String)]) -> bool;
    fn approve(&mut self, to: &Address, token_id: String) -> bool;
    fn get_approved(&mut self, token_id: String) -> Address;
    fn take_ownership(&mut self, token_id: String) -> bool;
    fn create_multi_tokens(&mut self, owner: &Address) -> bool;
    fn create_one_token(
        &mut self, name: &str, url: &str, token_type: &str, owner: &Address,
    ) -> bool;
}

pub(crate) struct Oep5TokenInstance;

impl Oep5Token for Oep5TokenInstance {
    fn initialize(&mut self, owner: &Address) -> bool {
        if database::get::<_, bool>(INITED).unwrap_or_default() == false {
            database::put(INITED, true);
            if self.create_multi_tokens(owner) == true {
                return true;
            }
        }
        false
    }
    fn name(&self) -> String {
        "wasm_token".to_string()
    }
    fn total_supply(&self) -> U256 {
        database::get(KEY_TOTAL_SUPPLY).unwrap_or_default()
    }
    fn query_token_id_by_index(&self, idx: U256) -> String {
        database::get(&utils::concat(PREFIX_INDEX, &idx)).unwrap_or_default()
    }
    fn query_token_by_id(&self, token_id: String) -> String {
        let (_, _, image, _): (String, String, String, String) =
            database::get(&utils::concat(PREFIX_TOKEN_ID, &token_id)).unwrap_or_default();
        image
    }
    fn balance_of(&self, address: &Address) -> U256 {
        database::get(&utils::concat(PREFIX_BALANCE, address)).unwrap_or_default()
    }
    fn owner_of(&self, token_id: String) -> Address {
        database::get(&utils::concat(PREFIX_OWNER, &token_id)).unwrap_or(Address::zero())
    }
    fn transfer(&mut self, to: &Address, token_id: String) -> bool {
        let owner = self.owner_of(token_id.clone());
        if runtime::check_witness(&owner) == false {
            return false;
        }
        database::put(&utils::concat(PREFIX_OWNER, &token_id), to);
        true
    }
    fn transfer_multi(&mut self, states: &[(Address, String)]) -> bool {
        if states.is_empty() {
            return false;
        }
        for state in states.iter() {
            if self.transfer(&state.0, state.1.clone()) == false {
                panic!("transfer failed, to:{}, token_id:{}", state.0, state.1);
            }
        }
        true
    }
    fn approve(&mut self, to: &Address, token_id: String) -> bool {
        let owner = self.owner_of(token_id.clone());
        if runtime::check_witness(&owner) == false {
            return false;
        }
        database::put(&utils::concat(PREFIX_APPROVE, &token_id), to);
        true
    }
    fn get_approved(&mut self, token_id: String) -> Address {
        database::get(&utils::concat(PREFIX_APPROVE, token_id)).unwrap_or_default()
    }
    fn take_ownership(&mut self, token_id: String) -> bool {
        let to = self.get_approved(token_id.clone());
        if runtime::check_witness(&to) == false {
            return false;
        }
        database::put(&utils::concat(PREFIX_OWNER, &token_id), to);
        true
    }
    fn create_multi_tokens(&mut self, owner: &Address) -> bool {
        let cards = [
            ("HEART A", "http://images.com/hearta.jpg"),
            ("HEART 2", "http://images.com/hearta.jpg"),
        ];
        for card in cards.iter() {
            if self.create_one_token(card.0, card.1, "CARD", owner) == false {
                return false;
            }
        }
        true
    }
    fn create_one_token(
        &mut self, name: &str, url: &str, token_type: &str, owner: &Address,
    ) -> bool {
        let mut total_supply: U256 = database::get(KEY_TOTAL_SUPPLY).unwrap_or_default();
        total_supply = total_supply + U256::from(1);
        database::put(KEY_TOTAL_SUPPLY, &total_supply);
        let tmp = utils::concat(owner, &total_supply);
        let token_id = runtime::sha256(&tmp).to_hex_string();
        let token = (token_id.clone(), name, url, token_type);
        database::put(&utils::concat(PREFIX_INDEX, &total_supply), &token_id);
        database::put(&utils::concat(PREFIX_OWNER, &token_id), owner);
        database::put(&utils::concat(PREFIX_TOKEN_ID, &token_id), token);
        let mut balance = self.balance_of(owner);
        balance = balance + U256::from(1);
        database::put(&utils::concat(PREFIX_BALANCE, owner), balance);
        true
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher = Oep5TokenDispatcher::new(Oep5TokenInstance);
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
