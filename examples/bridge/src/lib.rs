#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use crate::erc20::{balance_of_erc20, transfer_erc20, transfer_from_erc20};
use crate::events::{erc20_to_oep4_event, oep4_to_erc20_event};
use crate::oep4::{balance_of_neovm, transfer_neovm};
use ostd::abi::{Decoder, Encoder, Sink, Source};
use ostd::database::{get, put};
use ostd::macros::base58;
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, contract_migrate};
use ostd::{database, runtime};
use std::fs::read;

mod erc20;
mod events;
mod oep4;

const KEY_ADMIN: &[u8] = b"1";
const PREFIX_TOKEN_PAIR: &[u8] = b"2";
const KEY_TOKEN_PAIR_NAME: &[u8] = b"3";

const ZERO_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
const ADMIN_ADDRESS: Address = base58!("ARGK44mXXZfU6vcdSfFKMzjaabWxyog1qb");

#[derive(Encoder, Decoder, Default)]
struct TokenPair {
    erc20: Address,
    oep4: Address,
}

fn initialize(admin: &Address) -> bool {
    assert_eq!(get_admin(), ZERO_ADDRESS, "has inited");
    assert!(check_witness(admin), "check admin signature failed");
    put(KEY_ADMIN, admin);
    true
}

fn get_admin() -> Address {
    get(KEY_ADMIN).unwrap_or_default()
}

fn get_all_token_pair_name() -> Vec<Vec<u8>> {
    get(KEY_TOKEN_PAIR_NAME).unwrap_or_default()
}

fn register_token_pair(
    token_pair_name: &[u8], ont_token_addr: &Address, eth_token_addr: &Address,
) -> bool {
    assert!(check_witness(&get_admin()), "need admin signature");
    let pair_key = gen_key(PREFIX_TOKEN_PAIR, token_pair_name);
    let token_pair: Option<TokenPair> = get(pair_key);
    assert!(token_pair.is_none(), "token pair name has registered");

    let mut names = get_all_token_pair_name();
    names.push(token_pair_name.to_vec());
    put(KEY_TOKEN_PAIR_NAME, names);

    assert_ne!(ont_token_addr, &ZERO_ADDRESS);
    assert_ne!(eth_token_addr, &ZERO_ADDRESS);

    database::put(
        pair_key.as_slice(),
        TokenPair { erc20: eth_token_addr.clone(), oep4: ont_token_addr.clone() },
    );
    true
}

fn get_next_token_id() -> u32 {
    get(KEY_NEXT_TOKEN_ID).unwrap_or_default()
}

fn un_register_token_pair(token_name: &[u8], ont_acct: &Address, eth_acct: &Address) -> bool {
    assert!(check_witness(&get_admin()), "need admin signature");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_name).as_slice());
    if let Some(pair) = token_pair {
        let this = address();
        let oep4_balance = balance_of_neovm(&pair.oep4, &this);
        transfer_neovm(&pair.oep4, &this, ont_acct, oep4_balance);
        let erc20_balance = balance_of_erc20(&this, &pair.erc20, &this);
        transfer_erc20(&this, &pair.erc20, eth_acct, erc20_balance);
        true
    } else {
        false
    }
}

fn get_token_pair(token_name: &[u8]) -> TokenPair {
    get(gen_key(PREFIX_TOKEN_PAIR, token_name).as_slice()).unwrap_or_default()
}

fn migrate(
    code: &[u8], vm_type: u32, name: &str, version: &str, author: &str, email: &str, desc: &str,
) -> bool {
    assert!(check_witness(&ADMIN_ADDRESS));
    let oep4_addr = get_oep4_token_addr();
    let erc20_addr = &get_erc20_token_addr();
    let this = &address();
    let addr = contract_migrate(code, vm_type, name, version, author, email, desc);
    assert_ne!(addr, ZERO_ADDRESS);
    let oep4_balance = balance_of_neovm(&oep4_addr, this);
    if !oep4_balance.is_zero() {
        transfer_neovm(&oep4_addr, this, &addr, oep4_balance);
    }
    let erc20_balance = balance_of_erc20(this, erc20_addr, this);
    if !erc20_balance.is_zero() {
        transfer_erc20(this, erc20_addr, &addr, erc20_balance);
    }
    true
}

fn oep4_to_erc20(
    ont_acct: &Address, eth_acct: &Address, amount: U128, token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(ont_acct));
    assert!(!amount.is_zero(), "amount should be more than 0");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_pair_name));
    if let Some(pair) = token_pair {
        let this = &address();
        let before = balance_of_neovm(&pair.oep4, this);
        transfer_neovm(&pair.oep4, ont_acct, this, amount);
        let after = balance_of_neovm(&pair.oep4, this);
        let delta = after - before;
        if !delta.is_zero() {
            transfer_erc20(this, &pair.erc20, eth_acct, delta);
        }
        oep4_to_erc20_event(ont_acct, eth_acct, amount, delta, oep4_addr, erc20_addr);
        true
    } else {
        false
    }
}

fn erc20_to_oep4(
    eth_acct: &Address, ont_acct: &Address, amount: U128, token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(eth_acct));
    assert!(!amount.is_zero(), "amount should be more than 0");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_pair_name));
    if let Some(pair) = token_pair {
        let this = &address();
        let before = balance_of_erc20(this, &pair.erc20, this);
        transfer_from_erc20(this, &pair.erc20, eth_acct, this, amount);
        let after = balance_of_erc20(this, &pair.erc20, this);
        assert!(after >= before);
        let delta = after - before;
        if !delta.is_zero() {
            transfer_neovm(&pair.oep4, this, ont_acct, delta);
        }
        erc20_to_oep4_event(eth_acct, ont_acct, amount, delta, oep4_addr, erc20_addr);
        true
    } else {
        false
    }
}

fn gen_key<T: Encoder>(prefix: &[u8], post: T) -> Vec<u8> {
    let mut sink = Sink::new(64);
    sink.write(prefix);
    sink.write(post);
    sink.bytes().to_vec()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin))
        }
        "registerTokenPair" => {
            let (token_pair_name, ont_token_addr, eth_token_addr) = source.read().unwrap();
            sink.write(register_token_pair(token_pair_name, ont_token_addr, eth_token_addr))
        }
        "unRegisterTokenPair" => {
            let (token_pair_name, ont_token_addr, eth_token_addr) = source.read().unwrap();
            sink.write(un_register_token_pair(token_pair_name, ont_token_addr, eth_token_addr))
        }
        "getTokenPair" => {
            let token_pair_name = source.read().unwrap();
            sink.write(get_token_pair(token_pair_name));
        }
        "migrate" => {
            let (code, vm_type, name, version, author, email, desc) = source.read().unwrap();
            let vm_type: U128 = vm_type;
            sink.write(migrate(code, vm_type.raw() as u32, name, version, author, email, desc));
        }
        "oep4ToErc20" => {
            let (ont_acct, eth_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(oep4_to_erc20(ont_acct, eth_acct, amount, token_pair_name));
        }
        "erc20ToOep4" => {
            let (eth_acct, ont_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(erc20_to_oep4(eth_acct, ont_acct, amount, token_pair_name));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}
