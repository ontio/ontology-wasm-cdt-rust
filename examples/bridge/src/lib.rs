#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

mod erc20;
mod events;
mod oep4;

extern crate ontio_std as ostd;

use crate::erc20::{balance_of_erc20, transfer_erc20, transfer_from_erc20};
use crate::events::{erc20_to_oep4_event, oep4_to_erc20_event};
use crate::oep4::{balance_of_neovm, transfer_neovm};
use ostd::abi::{Sink, Source};
use ostd::macros::base58;
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, contract_migrate};
use ostd::{database, runtime};

const KEY_OEP4_TOKEN_ADDR: &[u8] = b"1";
const KEY_ERC20_TOKEN_ADDR: &[u8] = b"2";

const ZERO_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
const ADMIN_ADDRESS: Address = base58!("ARGK44mXXZfU6vcdSfFKMzjaabWxyog1qb");

fn initialize(ont_token_addr: &Address, eth_token_addr: &Address) -> bool {
    assert_ne!(ont_token_addr, &ZERO_ADDRESS);
    assert_ne!(eth_token_addr, &ZERO_ADDRESS);

    assert_eq!(get_oep4_token_addr(), ZERO_ADDRESS, "has inited");
    assert_eq!(get_erc20_token_addr(), ZERO_ADDRESS, "has inited");

    database::put(KEY_OEP4_TOKEN_ADDR, ont_token_addr);
    database::put(KEY_ERC20_TOKEN_ADDR, eth_token_addr);
    true
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

fn oep4_to_erc20(ont_acct: &Address, eth_acct: &Address, amount: U128) -> bool {
    assert!(check_witness(ont_acct));
    let oep4_addr = &get_oep4_token_addr();
    assert_ne!(oep4_addr, &ZERO_ADDRESS);
    let erc20_addr = &get_erc20_token_addr();
    assert_ne!(erc20_addr, &ZERO_ADDRESS);
    let this = &address();
    let before = balance_of_neovm(oep4_addr, this);
    assert!(transfer_neovm(oep4_addr, ont_acct, this, amount));
    let after = balance_of_neovm(oep4_addr, this);
    let delta = after - before;
    if !delta.is_zero() {
        transfer_erc20(this, erc20_addr, eth_acct, delta);
    }
    oep4_to_erc20_event(ont_acct, eth_acct, amount, oep4_addr, erc20_addr);
    true
}

fn erc20_to_oep4(eth_acct: &Address, ont_acct: &Address, amount: U128) -> bool {
    assert!(check_witness(eth_acct));
    let oep4_addr = &get_oep4_token_addr();
    assert_ne!(oep4_addr, &ZERO_ADDRESS);
    let erc20_addr = &get_erc20_token_addr();
    assert_ne!(erc20_addr, &ZERO_ADDRESS);
    let this = &address();
    let before = balance_of_erc20(this, erc20_addr, this);

    transfer_from_erc20(this, erc20_addr, eth_acct, this, amount);
    let after = balance_of_erc20(this, erc20_addr, this);
    assert!(after >= before);
    let delta = after - before;
    if !delta.is_zero() {
        transfer_neovm(oep4_addr, this, ont_acct, delta);
    }
    erc20_to_oep4_event(eth_acct, ont_acct, amount, oep4_addr, erc20_addr);
    true
}

fn get_oep4_token_addr() -> Address {
    database::get(KEY_OEP4_TOKEN_ADDR).unwrap_or_default()
}

fn get_erc20_token_addr() -> Address {
    database::get(KEY_ERC20_TOKEN_ADDR).unwrap_or_default()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => {
            let (ont_token_addr, eth_token_addr) = source.read().unwrap();
            sink.write(initialize(ont_token_addr, eth_token_addr))
        }
        "get_oep4_address" => sink.write(get_oep4_token_addr()),
        "get_erc20_address" => sink.write(get_erc20_token_addr()),
        "migrate" => {
            let (code, vm_type, name, version, author, email, desc) = source.read().unwrap();
            let vm_type: U128 = vm_type;
            sink.write(migrate(code, vm_type.raw() as u32, name, version, author, email, desc));
        }
        "oep4ToErc20" => {
            let (ont_acct, eth_acct, amount) = source.read().unwrap();
            sink.write(oep4_to_erc20(ont_acct, eth_acct, amount));
        }
        "erc20ToOep4" => {
            let (eth_acct, ont_acct, amount) = source.read().unwrap();
            sink.write(erc20_to_oep4(eth_acct, ont_acct, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}
