#![no_std]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{EventBuilder, Sink, Source};
use ostd::macros::base58;
use ostd::macros::event;
use ostd::prelude::*;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &str = "total_supply";
const NAME: &str = "wasm_token";
const SYMBOL: &str = "WTK";
const TOTAL_SUPPLY: U128 = 100_000_000_000;

const ADMIN: Address = base58!("AQf4Mzu1YJrhz9f3aRkkwSm9n3qhXGSh4p");

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, TOTAL_SUPPLY);
    database::put(ADMIN, TOTAL_SUPPLY);
    true
}

fn balance_of(owner: &Address) -> U128 {
    database::get(owner).unwrap_or(0)
}

fn transfer(from: &Address, to: &Address, amount: U128) -> bool {
    assert!(runtime::check_witness(from));

    let frmbal = balance_of(from);
    if amount == 0 || frmbal < amount {
        return false;
    }
    database::put(from, frmbal - amount);
    let tobal = balance_of(to);
    database::put(to, tobal + amount);
    EventBuilder::new().string("Transfer").address(from).address(to).number(amount).notify();
    notify::transfer(from, to, amount);
    notify::transfer_name(from, to, amount);
    notify::transfer_test(from, to, amount);
    let h = runtime::sha256("test");
    notify::event_test(true, b"test", "test", &h);
    true
}

fn total_supply() -> U128 {
    database::get(KEY_TOTAL_SUPPLY).unwrap()
}

mod notify {
    use super::*;
    #[event]
    pub fn transfer(from: &Address, to: &Address, amount: U128) {}

    #[event(name = mytransfer)]
    pub fn transfer_name(from: &Address, to: &Address, amount: U128) {}

    #[event]
    pub fn transfer_test(from: &Address, to: &Address, amount: U128) {}

    #[event]
    pub fn event_test(boo: bool, bs: &[u8], ss: &str, h: &H256) {}
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
