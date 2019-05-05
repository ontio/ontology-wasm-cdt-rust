extern crate wabt;
extern crate parity_wasm;
use crate::check_wasm::{check, check_module};
use std::io::BufRead;
use parity_wasm::elements::Module;
extern crate wasmi;

#[test]
fn check_f32_test() {
    // Parse WAT (WebAssembly Text format) into wasm bytecode.
    let wasm_binary: Vec<u8> =
        wabt::wat2wasm(
            r#"
            (module
                (type (;0;) (func (result f32)))
                (import "env" "input_length" (func (;0;) (type 0)))
            )
            "#,
        ).expect("failed to parse wat");
    check_test(wasm_binary);
}
#[test]
fn check_f64_test() {
    let wasm_binary: Vec<u8> =
        wabt::wat2wasm(
            r#"
            (module
                (type (;0;) (func (result f64)))
                (import "env" "input_length" (func (;0;) (type 0)))
            )
            "#,
        ).expect("failed to parse wat");
    check_test(wasm_binary);
}

#[test]
fn check_env_test() {
    let wasm_binary: Vec<u8> =
        wabt::wat2wasm(
            r#"
            (module
                (type (;0;) (func (param i32 i32 i32) (result i64)))
                (import "env" "input_length" (func (;0;) (type 0)))
            )
            "#,
        ).expect("failed to parse wat");
    check_test(wasm_binary);
}

#[test]
fn check_global_section() {
    let wasm_binary: Vec<u8> =
        wabt::wat2wasm(
            r#"
            (module
                (global (;0;) (mut f32) (f32.const 32768))
            )
            "#,
        ).expect("failed to parse wat");
    check_test(wasm_binary);
}


fn check_test(wasm_binary: Vec<u8>) {
    let parity_module = parity_wasm::deserialize_buffer(wasm_binary.as_slice());
    let wasmi_module = wasmi::Module::from_parity_wasm_module(parity_module).unwrap();
    wasmi_module.deny_floating_point();
    match parity_module {
        Ok(parity_modu) => {
            let res = check_module(&parity_modu);
            match res.clone() {
                Ok(r) => {
                }
                Err(e) => {
                    println!("err: {}", e);
                }
            }
            assert!( res.is_err());
        }
        Err(e) => {}
    }
}


#[test]
fn check_wasm_test() {
    check("./token.wasm");
}