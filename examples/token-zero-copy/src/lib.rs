#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Encoder, Sink, ZeroCopySource};
use ostd::prelude::*;
use ostd::string::ToString;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const NAME: &'static str = "wasm_token";
const SYMBOL: &'static str = "WTK";
const TOTAL_SUPPLY: u128 = 100000000000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, TOTAL_SUPPLY);
    true
}

fn balance_of(owner: &Addr) -> U128 {
    database::get(owner).unwrap_or(0)
}

fn transfer(from: &Addr, to: &Addr, amount: U128) -> bool {
    if runtime::check_witness(&from) == false {
        return false;
    }
    let mut frmbal = balance_of(from);
    let mut tobal = balance_of(to);
    if amount == 0 || frmbal < amount {
        false
    } else {
        frmbal = frmbal - amount;
        tobal = tobal + amount;
        database::put(from, frmbal);
        database::put(to, tobal);
        notify(("Transfer".to_string(), from, to, amount));
        true
    }
}

fn total_supply() -> U128 {
    database::get(KEY_TOTAL_SUPPLY).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = ZeroCopySource::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"init" => sink.write(initialize()),
        b"name" => sink.write(NAME.to_string()),
        b"symbol" => sink.write(SYMBOL.to_string()),
        b"totalSupply" => sink.write(total_supply()),
        b"balanceOf" => {
            let addr = source.read_addr().unwrap();
            sink.write(balance_of(addr));
        }
        b"transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(transfer(from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}

fn notify<T: Encoder>(msg: T) {
    let mut sink = Sink::new(16);
    sink.write(msg);
    runtime::notify(sink.bytes());
}
