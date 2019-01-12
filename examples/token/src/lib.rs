#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::types::{Address, U256};
use ostd::{database, runtime};
use ostd::{string::ToString, String};

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const NAME: &'static str = "wasm_token";
const SYMBOL: &'static str = "WTK";
const TOTAL_SUPPLY: u64 = 100000000000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY.as_bytes(), U256::from(TOTAL_SUPPLY));
    true
}

fn balance_of(owner: &Address) -> U256 {
    database::get(owner.as_ref()).unwrap_or(U256::zero())
}

fn transfer(from: Address, to: Address, amount: U256) -> bool {
    if runtime::check_witness(&from) == false {
        return false;
    }
    let mut frmbal = balance_of(&from);
    let mut tobal = balance_of(&to);
    if amount == 0.into() || frmbal < amount {
        false
    } else {
        frmbal = frmbal - amount;
        tobal = tobal + amount;
        database::put(from.as_ref(), frmbal);
        database::put(to.as_ref(), tobal);
        true
    }
}

fn total_supply() -> U256 {
    database::get(KEY_TOTAL_SUPPLY.as_bytes()).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let mut source = Source::new(runtime::input());
    let action = source.read::<String>().unwrap();
    let mut sink = Sink::new(12);
    match action.as_str() {
        "init" => sink.write(initialize()),
        "name" => sink.write(NAME.to_string()),
        "symbol" => sink.write(SYMBOL.to_string()),
        "totalSupply" => sink.write(total_supply()),
        "balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(balance_of(&addr));
        }
        "transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(transfer(from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(&sink.into())
}
