#![no_std]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, ZeroCopySource};
use ostd::prelude::*;
use ostd::runtime;
use ostd::{str, String};

fn say_hello() -> String {
    return "hello world".to_string();
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = ZeroCopySource::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"hello" => sink.write(say_hello()),
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}
#[test]
fn test_hello() {
    hello();
}
