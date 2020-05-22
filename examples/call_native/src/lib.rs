#![no_std]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, Source};
use ostd::contract::ontid;
use ostd::runtime;

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"verifySignature" => {
            let (ont_id, index) = source.read().unwrap();
            sink.write(ontid::verify_signature(ont_id, index));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}
