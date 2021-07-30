#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use crate::erc20::{balance_of_erc20, transfer_erc20, transfer_from_erc20};
use crate::events::{
    erc20_to_oep4_event, new_admin_event, new_pending_admin_event, oep4_to_erc20_event,
};
use crate::oep4::{balance_of_neovm, transfer_neovm};
use ostd::abi::{Decoder, Encoder, Sink, Source};
use ostd::database::{delete, get, put};
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, contract_migrate, input, ret};

mod erc20;
mod events;
mod oep4;

const KEY_ADMIN: &[u8] = b"1";
const KEY_PENDING_ADMIN: &[u8] = b"2";
const PREFIX_TOKEN_PAIR: &[u8] = b"3";
const KEY_TOKEN_PAIR_NAME: &[u8] = b"4";

#[derive(Encoder, Decoder, Default)]
struct TokenPair {
    erc20: Address,
    erc20_decimals: u32,
    oep4: Address,
    oep4_decimals: u32,
}

fn initialize(admin: &Address) -> bool {
    assert!(get_admin().is_zero(), "has inited");
    assert!(check_witness(admin), "check admin signature failed");
    put(KEY_ADMIN, admin);
    true
}

fn get_admin() -> Address {
    get(KEY_ADMIN).unwrap_or_default()
}

fn set_pending_admin(new_admin: &Address) -> bool {
    assert!(check_witness(&get_admin()), "check admin signature failed");
    put(KEY_PENDING_ADMIN, new_admin);
    new_pending_admin_event(new_admin);
    true
}

fn get_pending_admin() -> Address {
    get(KEY_PENDING_ADMIN).unwrap_or_default()
}

fn accept_admin() -> bool {
    let pending_admin = get_pending_admin();
    assert!(check_witness(&get_pending_admin()), "check pending admin signature failed");
    let old_admin = get_admin();
    put(KEY_ADMIN, pending_admin);
    delete(KEY_PENDING_ADMIN);
    new_admin_event(&old_admin, &pending_admin);
    true
}

fn get_all_token_pair_name() -> Vec<Vec<u8>> {
    get(KEY_TOKEN_PAIR_NAME).unwrap_or_default()
}

fn register_token_pair(
    token_pair_name: &[u8], oep4_addr: &Address, oep4_decimals: U128, erc20_addr: &Address,
    erc20_decimals: U128,
) -> bool {
    assert!(check_witness(&get_admin()), "need admin signature");
    assert!(!oep4_addr.is_zero(), "invalid oep4 address");
    assert!(!erc20_addr.is_zero(), "invalid erc20 address");

    let pair_key = gen_key(PREFIX_TOKEN_PAIR, token_pair_name);
    let token_pair: Option<TokenPair> = get(pair_key.as_slice());
    assert!(token_pair.is_none(), "token pair name has registered");

    let mut names = get_all_token_pair_name();
    names.push(token_pair_name.to_vec());
    put(KEY_TOKEN_PAIR_NAME, names);

    put(
        pair_key.as_slice(),
        TokenPair {
            erc20: *erc20_addr,
            erc20_decimals: erc20_decimals.raw() as u32,
            oep4: *oep4_addr,
            oep4_decimals: oep4_decimals.raw() as u32,
        },
    );
    true
}

fn update_token_pair(
    token_pair_name: &[u8], oep4_addr: &Address, oep4_decimals: U128, erc20_addr: &Address,
    erc20_decimals: U128, eth_acct: &Address, ont_acct: &Address,
) -> bool {
    assert!(check_witness(&get_admin()), "need admin signature");
    let pair_key = gen_key(PREFIX_TOKEN_PAIR, token_pair_name);
    let mut pair: TokenPair = get(pair_key.as_slice()).expect("token pair name has not registered");
    let this = &address();
    if &pair.oep4 != oep4_addr && !oep4_addr.is_zero() {
        assert!(!ont_acct.is_zero(), "ont acct should not be nil");
        let ba = balance_of_neovm(&pair.oep4, this);
        transfer_neovm(&pair.oep4, this, ont_acct, ba);
        pair.oep4 = *oep4_addr;
        pair.oep4_decimals = oep4_decimals.raw() as u32;
    }
    if &pair.erc20 != erc20_addr && !erc20_addr.is_zero() {
        assert!(!eth_acct.is_zero(), "eth acct should not be nil");
        let ba = balance_of_erc20(this, &pair.erc20, this);
        transfer_erc20(this, &pair.erc20, eth_acct, ba);
        pair.erc20 = *erc20_addr;
        pair.erc20_decimals = erc20_decimals.raw() as u32;
    }
    put(pair_key.as_slice(), pair);
    true
}

fn unregister_token_pair(token_pair_name: &[u8], ont_acct: &Address, eth_acct: &Address) -> bool {
    assert!(check_witness(&get_admin()), "need admin signature");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_pair_name).as_slice());
    if let Some(pair) = token_pair {
        let this = address();
        let oep4_balance = balance_of_neovm(&pair.oep4, &this);
        transfer_neovm(&pair.oep4, &this, ont_acct, oep4_balance);
        let erc20_balance = balance_of_erc20(&this, &pair.erc20, &this);
        transfer_erc20(&this, &pair.erc20, eth_acct, erc20_balance);
        let mut all_token_pair_name = get_all_token_pair_name();
        let index = all_token_pair_name.iter().position(|item| item == token_pair_name).unwrap();
        all_token_pair_name.remove(index);
        put(KEY_TOKEN_PAIR_NAME, all_token_pair_name);
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
    assert!(check_witness(&get_admin()), "check admin signature failed");
    let this = &address();
    let all_token_pair_name = get_all_token_pair_name();
    let token_pairs: Vec<TokenPair> =
        all_token_pair_name.iter().map(|name| get_token_pair(name)).collect();

    let new_addr = contract_migrate(code, vm_type, name, version, author, email, desc);
    assert!(!new_addr.is_zero());
    token_pairs.iter().for_each(|pair| {
        let oep4_balance = balance_of_neovm(&pair.oep4, this);
        if !oep4_balance.is_zero() {
            transfer_neovm(&pair.oep4, this, &new_addr, oep4_balance);
        }
        let erc20_balance = balance_of_erc20(this, &pair.erc20, this);
        if !erc20_balance.is_zero() {
            transfer_erc20(this, &pair.erc20, &new_addr, erc20_balance);
        }
    });
    true
}

fn oep4_to_erc20(
    ont_acct: &Address, eth_acct: &Address, mut amount: U128, token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(ont_acct));
    assert!(!amount.is_zero(), "amount should be more than 0");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_pair_name));
    if let Some(pair) = token_pair {
        // 由大精度到小精度转换 会有精度丢失
        let decimals_delta = if pair.erc20_decimals < pair.oep4_decimals {
            let decimals_delta = pair.oep4_decimals - pair.erc20_decimals;
            let remainder = amount.raw() % 10u128.pow(decimals_delta);
            if remainder != 0 {
                amount -= U128::new(remainder);
            }
            decimals_delta
        } else {
            pair.erc20_decimals - pair.oep4_decimals
        };
        let this = &address();
        let before = balance_of_neovm(&pair.oep4, this);
        transfer_neovm(&pair.oep4, ont_acct, this, amount);
        let after = balance_of_neovm(&pair.oep4, this);
        let delta = after - before;
        let erc20_amt = if delta.is_zero() {
            U128::new(0)
        } else {
            let erc20_amt = if pair.erc20_decimals < pair.oep4_decimals {
                delta / U128::new(10u128.pow(decimals_delta))
            } else {
                delta * U128::new(10u128.pow(decimals_delta))
            };
            transfer_erc20(this, &pair.erc20, eth_acct, erc20_amt);
            erc20_amt
        };
        oep4_to_erc20_event(ont_acct, eth_acct, amount, erc20_amt, &pair.oep4, &pair.erc20);
        true
    } else {
        false
    }
}

fn erc20_to_oep4(
    ont_acct: &Address, eth_acct: &Address, mut amount: U128, token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(ont_acct));
    assert!(!amount.is_zero(), "amount should be more than 0");
    let token_pair: Option<TokenPair> = get(gen_key(PREFIX_TOKEN_PAIR, token_pair_name));
    if let Some(pair) = token_pair {
        // 由大精度到小精度转换 会有精度丢失
        let decimals_delta = if pair.erc20_decimals > pair.oep4_decimals {
            let decimals_delta = pair.erc20_decimals - pair.oep4_decimals;
            let remainder = amount.raw() % 10u128.pow(decimals_delta);
            if remainder != 0 {
                amount -= U128::new(remainder);
            }
            decimals_delta
        } else {
            pair.oep4_decimals - pair.erc20_decimals
        };
        let this = &address();
        let before = balance_of_erc20(this, &pair.erc20, this);
        transfer_from_erc20(ont_acct, &pair.erc20, eth_acct, this, amount);
        let after = balance_of_erc20(this, &pair.erc20, this);
        assert!(after >= before);
        let delta = after - before;
        let oep4_amt = if delta.is_zero() {
            U128::new(0)
        } else {
            let oep4_amt = if pair.erc20_decimals > pair.oep4_decimals {
                delta / U128::new(10u128.pow(decimals_delta))
            } else {
                delta * U128::new(10u128.pow(decimals_delta))
            };
            transfer_neovm(&pair.oep4, this, ont_acct, oep4_amt);
            oep4_amt
        };
        erc20_to_oep4_event(ont_acct, eth_acct, amount, oep4_amt, &pair.oep4, &pair.erc20);
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
    let input = input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin))
        }
        "getAdmin" => {
            sink.write(get_admin());
        }
        "setPendingAdmin" => {
            let new_admin = source.read().unwrap();
            sink.write(set_pending_admin(new_admin));
        }
        "getPendingAdmin" => {
            sink.write(get_pending_admin());
        }
        "acceptAdmin" => {
            sink.write(accept_admin());
        }
        "registerTokenPair" => {
            let (token_pair_name, oep4_addr, oep4_decimals, erc20_addr, erc20_decimals) =
                source.read().unwrap();
            sink.write(register_token_pair(
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
            ))
        }
        "updateTokenPair" => {
            let (
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
                eth_acct,
                ont_acct,
            ) = source.read().unwrap();
            sink.write(update_token_pair(
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
                eth_acct,
                ont_acct,
            ))
        }
        "unregisterTokenPair" => {
            let (token_pair_name, ont_acct, eth_acct) = source.read().unwrap();
            sink.write(unregister_token_pair(token_pair_name, ont_acct, eth_acct))
        }
        "getAllTokenPairName" => {
            sink.write(get_all_token_pair_name());
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
        "erc20ToOep4" => {
            let (ont_acct, eth_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(erc20_to_oep4(ont_acct, eth_acct, amount, token_pair_name));
        }
        "oep4ToErc20" => {
            let (ont_acct, eth_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(oep4_to_erc20(ont_acct, eth_acct, amount, token_pair_name));
        }
        _ => panic!("unsupported action!"),
    }

    ret(sink.bytes())
}
