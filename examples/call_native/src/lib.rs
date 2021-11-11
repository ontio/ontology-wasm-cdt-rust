#![no_std]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::contract::ong;
use ostd::contract::ont;
use ostd::runtime;
use ostd::runtime::panic;

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"ontTransferV2" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ont::v2::transfer(from, to, amount));
        }
        b"ontBalanceOfV2" => {
            let from = source.read().unwrap();
            sink.write(ont::v2::balance_of(from));
        }
        b"ontApproveV2" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ont::v2::approve(from, to, amount));
        }
        b"ontAllowanceV2" => {
            let (from, to) = source.read().unwrap();
            sink.write(ont::v2::allowance(from, to));
        }
        b"ontTransferFromV2" => {
            let (spender, from, to, amount) = source.read().unwrap();
            sink.write(ont::v2::transfer_from(spender, from, to, amount));
        }

        b"ongTransferV2" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ong::v2::transfer(from, to, amount));
        }
        b"ongBalanceOfV2" => {
            let from = source.read().unwrap();
            sink.write(ong::v2::balance_of(from));
        }
        b"ongApproveV2" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(ong::v2::approve(from, to, amount));
        }
        b"ongAllowanceV2" => {
            let (from, to) = source.read().unwrap();
            sink.write(ong::v2::allowance(from, to));
        }
        b"ongTransferFromV2" => {
            let (spender, from, to, amount) = source.read().unwrap();
            sink.write(ong::v2::transfer_from(spender, from, to, amount));
        }
        _ => panic("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}
