#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::runtime;

#[no_mangle]
pub fn invoke() {
    runtime::ret("hello world".as_bytes())
}
