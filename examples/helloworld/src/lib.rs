#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use crate::ostd::abi::{Decoder, Encoder};
use ostd::abi::{Sink, Source, ZeroCopySource};
use ostd::prelude::*;
use ostd::runtime;
use ostd::{str, String};

#[derive(Encoder, Decoder, Debug)]
struct Person {
    name: String,
    age: u64,
}

fn say_hello() -> String {
    let mut p = &Person { name: "test".to_string(), age: 10 };
    let mut sink = Sink::new(0);
    p.encode(&mut sink);
    println!("{:?}", sink.bytes());

    let mut source = Source::new(sink.bytes().to_vec());
    let p2: Person = source.read().unwrap();
    println!("{:?}", p2);
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
    say_hello();
    assert_eq!(1, 2);
}
