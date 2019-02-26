#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Encoder, Sink, Source};
use ostd::prelude::*;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &str = "total_supply";
const NAME: &str = "wasm_token";
const SYMBOL: &str = "WTK";
const TOTAL_SUPPLY: U128 = 100_000_000_000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, TOTAL_SUPPLY);
    true
}

fn balance_of(owner: &Address) -> U128 {
    database::get(owner).unwrap_or(0)
}

fn transfer(from: &Address, to: &Address, amount: U128) -> bool {
    assert!(runtime::check_witness(from));

    let frmbal = balance_of(from);
    let tobal = balance_of(to);
    if amount == 0 || frmbal < amount {
        return false;
    }

    database::put(from, frmbal - amount);
    database::put(to, tobal + amount);
    notify(("Transfer", from, to, amount));
    true
}

fn total_supply() -> U128 {
    database::get(KEY_TOTAL_SUPPLY).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => sink.write(initialize()),
        "name" => sink.write(NAME),
        "symbol" => sink.write(SYMBOL),
        "totalSupply" => sink.write(total_supply()),
        "balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(balance_of(addr));
        }
        "transfer" => {
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
