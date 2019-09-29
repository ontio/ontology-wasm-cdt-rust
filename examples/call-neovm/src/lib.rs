#![feature(proc_macro_hygiene)]
#![no_std]
extern crate ontio_std as ostd;
use ostd::abi::{EventBuilder, Sink, Source, VmValueDecoder, VmValueEncoder, VmValueParser};
use ostd::contract::{neo, ont};
use ostd::prelude::*;
use ostd::runtime;
use ostd::types::{u128_from_neo_bytes, U128};
extern crate alloc;
use alloc::collections::BTreeMap;
use ostd::console::debug;

pub mod neovm;
use neovm::{Neo_Contract, Neo_Contract_Addr};

pub struct TestContext<'a> {
    admin: &'a Address,
    map: BTreeMap<String, &'a Address>,
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"contract_create" => {
            let code = hexutil::read_hex(Neo_Contract);
            let code_bs = code.unwrap_or(Vec::new());
            let contract_addr = runtime::contract_create(
                code_bs.as_slice(),
                1,
                "oep4",
                "1.0",
                "author",
                "email",
                "desc",
            );
            sink.write(contract_addr);
        }

        b"init" => {
            let res = neo::call_contract(&Neo_Contract_Addr, ("init", ()));
            if let res2 = res.unwrap() {
                let mut parser = VmValueParser::new(res2.as_slice());
                let r = parser.bool();
                sink.write(r.unwrap_or(false));
            } else {
                sink.write(false);
            }
        }
        b"name" => {
            let res = neo::call_contract(&Neo_Contract_Addr, ("name", ()));
            if let res2 = res.unwrap() {
                let mut parser = VmValueParser::new(res2.as_slice());
                let r = parser.bytearray();
                sink.write(r.unwrap_or(b""));
            } else {
                sink.write("");
            }
        }
        b"balanceOf" => {
            let addr: Address = source.read().unwrap();
            let res = neo::call_contract(&Neo_Contract_Addr, ("balanceOf", (addr,)));
            if let res2 = res.unwrap() {
                let mut parser = VmValueParser::new(&res2);
                let r = parser.bytearray().unwrap_or(b"0");
                sink.write(u128_from_neo_bytes(r));
            } else {
                sink.write(0 as u128);
            }
        }
        b"transfer" => {
            let from_addr: Address = source.read().unwrap();
            let to_addr: Address = source.read().unwrap();
            let amount: U128 = source.read().unwrap();

            let res =
                neo::call_contract(&Neo_Contract_Addr, ("transfer", (from_addr, to_addr, amount)));

            if res.is_some() {
                let data = res.unwrap();
                let mut parser = VmValueParser::new(&data);
                let boo = parser.bool().unwrap_or(false);
                sink.write(boo);
            } else {
                sink.write(false);
            }
        }
        b"testcase" => sink.write(testcase()),
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}

fn get_tc<'a>(source: &mut Source<'a>) -> TestContext<'a> {
    let mut map = BTreeMap::new();
    let admin = source.read().unwrap();
    let n = source.read_varuint().unwrap_or(0);

    TestContext { admin, map }
}

fn testcase() -> String {
    r#"
    [
        [{"method":"contract_create","expected":"address:ALMkVG1pDhKHfe5q44acX2W5qXX2FqYQhu"},
        {"method":"init","expected":"bool:true"},
        {"method":"name","expected":"string:MyToken"},
        {"method":"balanceOf", "param":"address:AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP","expected":"int:100000000000000000"},
        {"env":{"witness":["AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP"]}, "method":"transfer", "param":"address:AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP,address:AWJNqh9W4NDGmFSCHR4Mp5G9VBKR5r2juF, int:100","expected":"bool:true"},
        {"method":"balanceOf", "param":"address:AWJNqh9W4NDGmFSCHR4Mp5G9VBKR5r2juF","expected":"int:100"}
        ]
    ]
        "#
        .to_string()
}
