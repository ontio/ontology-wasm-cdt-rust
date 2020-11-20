#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{check_witness, input, ret};

use method::*;
use storage::*;

pub mod method;
pub mod storage;

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin));
        }
        b"isPriceOracle" => {
            sink.write(true);
        }
        b"putUnderlyingPrice" => {
            let (key_list, price_list) = source.read().unwrap();
            sink.write(put_underlying_price(key_list, price_list));
        }
        b"getUnderlyingPrice" => {
            let key = source.read().unwrap();
            sink.write(get_price(key));
        }
        b"setDecimal" => {
            let decimal = source.read().unwrap();
            assert!(check_witness(&get_admin()), "check witness failed");
            sink.write(put_decimal(decimal));
        }
        b"getDecimal" => {
            sink.write(get_decimal());
        }
        _ => {
            let method = str::from_utf8(action).ok().unwrap();
            panic!("not support method:{}", method)
        }
    }
    ret(sink.bytes());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
