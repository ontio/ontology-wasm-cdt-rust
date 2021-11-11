#![no_std]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, Source};
use ostd::contract::governance::un_authorize_for_peer;
use ostd::contract::{governance, ont};
use ostd::prelude::*;
use ostd::runtime;
use ostd::runtime::address;

fn authorize_for_peer_transfer_from(user: &Address, amt: U128, peer_pub_key: &str) -> bool {
    let this = address();
    ont::transfer(user, &this, amt);
    governance::authorize_for_peer_transfer_from(&this, amt, peer_pub_key)
}

fn authorize_for_peer(user: &Address, amt: U128, peer_pub_key: &str) -> bool {
    governance::authorize_for_peer(user, amt, peer_pub_key)
}

fn withdraw(user: &Address, amt: U128, peer_pub_key: &str) -> bool {
    governance::withdraw(user, amt, peer_pub_key)
}

fn withdraw_ong(user: &Address) -> bool {
    governance::withdraw_ong(user)
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "authorize_for_peer_transfer_from" => {
            let (user, amt, peer_pub_key) = source.read().unwrap();
            sink.write(authorize_for_peer_transfer_from(user, amt, peer_pub_key));
        }
        "authorize_for_peer" => {
            let (user, amt, peer_pub_key) = source.read().unwrap();
            sink.write(authorize_for_peer(user, amt, peer_pub_key))
        }
        "un_authorize_for_peer" => {
            let (user, amt, peer_pub_key) = source.read().unwrap();
            sink.write(un_authorize_for_peer(user, amt, peer_pub_key))
        }
        "withdraw" => {
            let (user, amt, peer_pub_key) = source.read().unwrap();
            sink.write(withdraw(user, amt, peer_pub_key));
        }
        "withdraw_ong" => {
            let user = source.read().unwrap();
            sink.write(withdraw_ong(user));
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
