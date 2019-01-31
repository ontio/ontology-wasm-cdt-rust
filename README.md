[![Build Status](https://travis-ci.com/laizy/ontio-cdk.svg?branch=master)](https://travis-ci.com/laizy/ontio-cdk)

[English](README_EN.md) | 中文

`ontio-cdk`是用于使用rust开发面向ontology的WebAssembly智能合约工具套件, 包含合约编写的标准库，链上交互的运行时api，合约接口abi生成插件，
合约测试框架等。

## 构建开发环境
1. 安装[rustup](https://rustup.rs/), 非windows环境可直接： `curl https://sh.rustup.rs -sSf | sh`
2. 安装rust编译器:`rustup install nightly`， 并设置默认采用nightly版本编译：`rustup default nightly`
3. 安装wasm32编译目标: `rustup target add wasm32-unknown-unknown`
4. 根据个人喜好选择编程ide或者editor，比如IntelliJ，VSCode，vim等

## 合约编写
1. 项目构建，`cargo new --lib mycontract`;
2. 编辑`Cargo.toml`，添加`ontio-cdk`依赖：
```toml
[package]
name = "mycontract"
version = "0.1.0"
authors = ["laizy <aochyi@126.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"] #编译为动态链接库

[dependencies]
ontio-std = {git = "https://github.com/laizy/ontio-cdk"}

[features]
mock = ["ontio-std/mock"]
```
3. 在src/lib.rs中开发合约，合约的基本结构如下：
```rust
#![no_std]
use ontio_std::runtime;

// 合约的入口函数，使用no_mangle使其在编译后作为wasm合约的导出函数
#[no_mangle]
pub fn invoke() {
    runtime::ret(b"hello, world");
}
```
4. 合约编译：`cargo build --release --target wasm32-unknown-unknown`

## 代码生成宏
合约的编写往往是先从输入参数的字节数组中解析出具体调用的方法和调用参数，然后跳转到对应的函数中执行，最终将执行结果序列化为字节数组返回。这和web服务器从网络上获取字节流，解析出具体的请求，执行对应的处理函数并将结果序列化为字节流发送回网络的工作方式是类似的。因此可以像web开发框架一样将这样的固定又繁琐工作统一处理，使合约的开发者专注于合约本身功能的开发。`ontio_std`提供了代码生成宏，可以根据合约接口在编译期自动生成辅助性代码。采用代码生成宏的合约基本结构如下：
```rust
#[ontio_std::abi_codegen::contract]
pub trait MyToken {
    //定义合约对外的接口
    fn initialize(&mut self, owner: &Address) -> bool;
    fn name(&self) -> String;
    fn balance_of(&self, owner: &Address) -> U256;
    fn transfer(&mut self, from: &Address, to: &Address, amount: U256) -> bool;
    fn approve(&mut self, approves: &Address, receiver: &Address, amount:U256) -> bool;
    fn transfer_from(&mut self, receiver: &Address,approves: &Address, amount:U256) -> bool;
    fn allowance(&mut self, approves: &Address, receiver: &Address) -> U256;
    fn total_supply(&self) -> U256;

    //定义合约的事件
    #[event]
    fn Transfer(&self, from: &Address, to: &Address, amount: U256) {}
    #[event]
    fn Approve(&self, approves:&Address, receiver: &Address, amount: U256) {}
}

pub(crate) struct MyTokenInstance;

#[no_mangle]
pub fn invoke() {
    // MyTokenDispatcher是使用abi_codegen::contract自动生成的类，实现了对合约请求的自动派发和结果的序列化操作
    let mut dispatcher = MyTokenDispatcher::new(MyTokenInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

//实现合约的具体逻辑
impl MyToken for MyTokenInstance {
    fn initialize(&mut self, owner:&Address) -> bool {
        ///....
    }

    fn balance_of(&self, owner: &Address) -> U256 {
        //...
    }
    //... 其他函数的实现
}
```
## 合约测试
如果合约的测试需要开发者搭建区块链节点，构造发送合约deploy和invoke交易等大量和合约本身业务无关的操作，那么这样的测试过程是十分低效和不可靠的。`ontio_std::mock`是合约的测试框架，提供了和链交互api的模拟，使合约开发者不需要和实际的链交互就可以方便地编写合约测试代码。
要使用测试功能，需要在Cargo.toml中设置feature：
```toml
[features]
mock = ["ontio-std/mock"]
```
在编写好测试用例后，使用`cargo test --features=mock`运行合约测试。
