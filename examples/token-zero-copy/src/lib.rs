#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Encoder, Sink, Source, ZeroCopySource};
use ostd::types::{Addr, Address, U256};
use ostd::{database, runtime};
use ostd::{string::ToString, String};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const NAME: &'static str = "wasm_token";
const SYMBOL: &'static str = "WTK";
const TOTAL_SUPPLY: u64 = 100000000000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, U256::from(TOTAL_SUPPLY));
    true
}

fn balance_of(owner: &Addr) -> U256 {
    database::get(owner).unwrap_or(U256::zero())
}

fn transfer(from: &Addr, to: &Addr, amount: U256) -> bool {
    if runtime::check_witness(&from) == false {
        return false;
    }
    let mut frmbal = balance_of(from);
    let mut tobal = balance_of(to);
    if amount == 0.into() || frmbal < amount {
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

fn total_supply() -> U256 {
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
            let from = source.read().unwrap();
            let to = source.read().unwrap();
            let amount = U256::zero();
            sink.write(transfer(from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(&sink.into())
}

fn notify<T: Encoder>(msg: T) {
    let mut sink = Sink::new(16);
    sink.write(msg);
    runtime::notify(&sink.into());
}
