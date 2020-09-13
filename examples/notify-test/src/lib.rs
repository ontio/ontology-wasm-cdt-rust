#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{EventBuilder, Sink, Source};
use ostd::prelude::*;
use ostd::runtime;
const _ADDR: Address = ostd::macros::base58!("AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP");

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"notify_string" => {
            EventBuilder::new().string("my name is lucas").notify();
        }
        b"notify_bytearray" => {
            EventBuilder::new().bytearray(_ADDR.as_ref()).notify();
        }
        b"notify_address" => {
            EventBuilder::new().address(&_ADDR).notify();
        }
        b"notify_number" => {
            EventBuilder::new().number(U128::new(128)).notify();
        }
        b"notify_bool" => {
            EventBuilder::new().bool(true).notify();
        }
        b"notify_hash" => {
            let h = runtime::sha256("test");
            EventBuilder::new().bytearray(h.as_bytes()).notify();
            EventBuilder::new().h256(&h).notify();
        }
        b"notify_list" => {
            EventBuilder::new()
                .string("my name is lucas")
                .bool(false)
                .address(&_ADDR)
                .number(U128::new(128))
                .notify();
        }
        _ => panic!("unsupported action!"),
    }
    sink.write(true);
    runtime::ret(sink.bytes())
}
