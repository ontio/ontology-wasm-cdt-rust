## ontio-cdk

[![Build Status](https://travis-ci.com/laizy/ontio-cdk.svg?branch=master)](https://travis-ci.com/laizy/ontio-cdk)

English | [中文](README_CN.md)

A suite of tools for ontology WebAssembly smart contract development using `rust`.
 
## Features
 
 - Runtime api for blockchain interaction
 - Contract level storage management
 - Contract testing framwork
 - Abi and client code generation

## Build development environment

1. install [rustup](https://rustup.rs/), 
Non-windows environment can be directly execute the following code：
```
curl https://sh.rustup.rs -sSf | sh
```
2. Install the rust compiler
```
rustup install nightly
```
And set the default to compile with nightly version
```
rustup default nightly
```
3. Install the wasm32 compilation target
```
rustup target add wasm32-unknown-unknown
```
4. Integrated Development Environment

Choose programming IDE or editor, such as IntelliJ, VSCode, vim, etc.

## How to write contract

1. create a project
```
cargo new --lib mycontract
```

2. edit `Cargo.toml`，add `ontio-cdk` dependencies

```toml
[package]
name = "mycontract"
version = "0.1.0"
authors = ["laizy <aochyi@126.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"] #Compile as a dynamic link library

[dependencies]
ontio-std = {git = "https://github.com/ontio/ontology-wasm-cdt-rust"}

[features]
mock = ["ontio-std/mock"]
```
3. Develop a contract in `src/lib.rs`, the basic structure of the contract is as follows：

```rust
#![no_std]
use ontio_std::runtime;

// The entry function of the contract, using no_mangle to make it a export function of the wasm contract after compilation.
#[no_mangle]
pub fn invoke() {
    runtime::ret(b"hello, world");
}
```
 a simple token contract is as follows:
 ```rust
#![no_std]
extern crate ontio_std as ostd;

use ostd::abi::{Encoder, Sink, ZeroCopySource};
use ostd::prelude::*;
use ostd::{database, runtime};

const KEY_TOTAL_SUPPLY: &str = "total_supply";
const NAME: &str = "wasm_token";
const SYMBOL: &str = "WTK";
const TOTAL_SUPPLY: u64 = 100000000000;

fn initialize() -> bool {
    database::put(KEY_TOTAL_SUPPLY, U256::from(TOTAL_SUPPLY));
    true
}

fn balance_of(owner: &Addr) -> U256 {
    database::get(owner).unwrap_or(U256::zero())
}

fn transfer(from: &Addr, to: &Addr, amount: U256) -> bool {
    assert!(runtime::check_witness(from));

    let mut frmbal = balance_of(from);
    let mut tobal = balance_of(to);
    if amount == U256::zero() || frmbal < amount {
        return false;
    }
    
    database::put(from, frmbal - amount);
    database::put(to, tobal + amount);
    notify(("Transfer", from, to, amount));
    true
}

fn total_supply() -> U256 {
    database::get(KEY_TOTAL_SUPPLY).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = ZeroCopySource::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => sink.write(initialize()),
        "name" => sink.write(NAME),
        "symbol" => sink.write(SYMBOL),
        "totalSupply" => sink.write(total_supply()),
        "balanceOf" => {
            let addr = source.read().unwrap();
            sink.write(balance_of(addr));
        }
        "transfer" => {
            let (from, to, amount) = source.read().unwrap();
            sink.write(transfer(from, to, amount));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}

fn notify<T: Encoder>(msg: T) {
    let mut sink = Sink::new(16);
    sink.write(msg);
    runtime::notify(sink.bytes());
}
 ```

4. compile contract：
since the default stack size is 1M, which is too large for contract, we need reduce it, 32kb is enough for most usecase.
```
RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
```

## Procedural Macros

The contract is usually written from the byte array of the input parameter to parse the specific call method and call 
parameters, then jump to the corresponding function to execute, and finally serialize the execution result into a byte
array and return. This is similar to how the web server retrieves the byte stream from the network, parses out the 
specific request, executes the corresponding handler function, and serializes the result into a byte stream that is
sent back to the network. Therefore, such cumbersome work can be handled in the same way as the web development
framework, so that the contract developers focus on the development of the contract itself. `ontio_std` provides code 
generation macros that automatically generate auxiliary code at compile time based on the contract interface. The basic 
contract structure of a code generation macro is as follows:

```rust
#[ontio_std::abi_codegen::contract]
pub trait MyToken {
    //Define the external interface of the contract
    fn initialize(&mut self, owner: &Address) -> bool;
    fn name(&self) -> String;
    fn balance_of(&self, owner: &Address) -> U256;
    fn transfer(&mut self, from: &Address, to: &Address, amount: U256) -> bool;
    fn approve(&mut self, approves: &Address, receiver: &Address, amount:U256) -> bool;
    fn transfer_from(&mut self, receiver: &Address,approves: &Address, amount:U256) -> bool;
    fn allowance(&mut self, approves: &Address, receiver: &Address) -> U256;
    fn total_supply(&self) -> U256;

    //defining Event of the contract
    #[event]
    fn Transfer(&self, from: &Address, to: &Address, amount: U256) {}
    #[event]
    fn Approve(&self, approves:&Address, receiver: &Address, amount: U256) {}
}

pub(crate) struct MyTokenInstance;

#[no_mangle]
pub fn invoke() {
    // MyTokenDispatcher is auto generated by abi_codegen::contract，Implements automatic dispatch of contract requests and serialization of results
    let mut dispatcher = MyTokenDispatcher::new(MyTokenInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

//The specific logic of the realization of the contract
impl MyToken for MyTokenInstance {
    fn initialize(&mut self, owner:&Address) -> bool {
        ///....
    }

    fn balance_of(&self, owner: &Address) -> U256 {
        //...
    }
    //... Implementation of other functions
}
```

## Contract test

`ontio_std::mock` is the contract's testing framework that provides a simulation of the api interaction with the chain, 
allowing contract developers to easily write contract test code without interacting with the actual chain.

To use the test function, you need to set the feature in Cargo.toml:
```toml
[features]
mock = ["ontio-std/mock"]
```
After writing the test case, run the contract test using `cargo test --features=mock`.

## License

This project is licensed under the [MIT license](LICENSE).

### Third party software

To quickly explore the feasibility of wasm contract development, initial development is based on the work make by third parties:

* `contract` macro and some api design is based on
  [pwasm-std](https://github.com/paritytech/pwasm-std) licensed under the MIT license or the Apache License (Version 2.0).
* contract test feature includes copies and modifications of [pwasm-test](https://github.com/paritytech/pwasm-test) 

See the source code files for more details.

Copies of third party licenses can be found in [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).

