#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, Source};
use ostd::contract::{ong, ont};
use ostd::runtime;

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"ong_transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ong::transfer(from, to, amount));
        }
        b"ong_balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(ong::balance_of(addr));
        }
        b"ong_approve" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ong::approve(from, to, amount));
        }
        b"ong_allowance" => {
            let (from, to) = source.read().unwrap();
            sink.write(ong::allowance(from, to));
        }
        b"ong_transfer_from" => {
            let (sender, from, to, amount) = source.read().unwrap();
            sink.write(ong::transfer_from(sender, from, to, amount));
        }
        b"ont_transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ont::transfer(from, to, amount));
        }
        b"ont_balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(ont::balance_of(addr));
        }
        b"ont_approve" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ont::approve(from, to, amount));
        }
        b"ont_allowance" => {
            let (from, to) = source.read().unwrap();
            sink.write(ont::allowance(from, to));
        }
        b"ont_transfer_from" => {
            let (sender, from, to, amount) = source.read().unwrap();
            sink.write(ont::transfer_from(sender, from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
