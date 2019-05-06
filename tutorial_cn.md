# 快速入门

## 相关工具
- cargo
- rust
- ontio-cdk

## 开始
1. 生成ontio-cdk api文档

克隆`https://github.com/ontio/ontology-wasm-cdt-rust.git`项目到本地，然后进入项目根目录，执行`cargo doc`命令生成api文档。

2. 合约中数据类型转换
- `u32`、`u64`等基本的数据类型转换成`U256`
- `U256`转换成`u64`、`u32`等数据类型

详情请参考api文档中`U256`类型的方法

示例
```
let u = U256::from(1);
let n = u.as_u64();
```
- `u64`转换成`string`
示例
```
let s = format!("{}", 123);
```
- `base58`编码的地址转换成`Address`
示例
```
let address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
```
3. 合约中验证签名

示例
```
let flag = runtime::check_witness(&from);
```

4. 合约与合约交互

在合约中调用其他合约时，需要按照目标合约的参数序列化标准序列化参数。
- `wasm`合约调用`neovm`合约
  - `U256`数据类型的序列化，需要先调用`types::to_neo_bytes()`方法转换成字节数组，然后在调用``。
  - `Address`数据类型要用`sink.write_neovm_address()`方法序列化。
  - 参数序列化步骤
    1. 先序列化要调用合约中方法的参数，目标合约的方法参数要按照倒序的方式序列化
    2. 方法参数序列化完成后，再序列化`83u8`和`193u8`字节码
    3. 序列化方法名
    4. 序列化字节码`103u8`
    5. 序列化合约地址
    ```
    let mut sink = Sink::new(16);
    sink.write(to_neo_bytes(amount));
    sink.write_neovm_address(to);
    sink.write_neovm_address(from);
    sink.write(83u8);
    sink.write(193u8);
    sink.write("transfer".to_string());
    sink.write(103u8);
    sink.write(contract_address);
    let res = runtime::call_contract(contract_address, sink.bytes());
    ```
- `wasm`调用`native`
   - `wasm`调用`ont`和`ong`中的方法请参考contract模块

## ontio-std介绍

1. abi 模块
- `Sink`  : 用于合约中数据类型的序列化
对于实现`Encoder`接口的数据类型都可以直接用`sink.write()`方法进行序列化,
`sink`进行初始化的时候,会初始化一个Vec,需要指定其初始化大小。

示例
```
let mut sink = Sink::new(16);
sink.write(83u8);
sink.write("transfer".to_string());
```

- `Source`: 用于合约中数据类型的反序列化

对于实现`Decoder`接口类型的数据类型可以直接用`source.read().unwrap()`方法进行反序列化

示例
```
let input = runtime::input();
let mut source = ZeroCopySource::new(&input);
let (from, to, amount) = source.read().unwrap();
```

3. console 模块

- `debug`：用于在合约中打印调试信息

示例
```
 console::debug("debug");
```

4. contract模块
- `ong`：封装了在合约中调用ong的相关操作，例如转账、查询余额等。
   - `allowance(from: &Address, to: &Address)` 查询allowance余额
     示例
    ```
    use ostd::contract::ont;
    ont::allowance(from, to)
    ```
   - `approve(from: &Address, to: &Address, amount: U256)` 一个地址允许另一个地址转移多少资产

     示例
    ```
    use ostd::contract::ont;
    ont::approve(from, to, amount)
    ```
   - `balance_of` 查询余额

     示例：
     ```
     use ostd::contract::ont;
     ong::balance_of(address)
     ```
   - `transfer` 转账

     示例
    ```
    let state = ont::State { from: from.clone(), to: to.clone(), amount: amount };
    ont::transfer(&[state])
    ```
   - `transfer_from`

     示例
    ```
    ont::transfer_from(sender, from, to, amount)
    ```
- `ont`:封装了在合约中调用ont的相关操作,调用方法和ong类似。



5. database 模块
- `delete`: 根据key删除数据库中的数据
- `get`   : 根据key查询数据
- `put`   : 根据key存储数据

示例：
```
use ostd::database;
database::put(from, frmbal);
let balance = database::get(owner).unwrap_or(U256::zero());
```

6. types 模块
- `Address`: 地址，是长度为20的字节数组
- `U256`   : 小端序的大整数。

7. runtime 模块

该模块封装了合约和链交互的api

|名称|参数|返回值|描述|
|:--|:--|:--|:--|
|timestamp||u64|获得当前时间戳|
|address||Address|获得当前合约地址|
|block_height||u32|获得当前区块高度|
|caller||Address|获得调用者的合约地址|
|call_contract|addr: &T,<br /> input: &[u8]|Option<Vec<u8>>|调用另一个合约|
|check_witness|<T: AsRef<Addr>>addr: T|bool|校验签名|
|contract_migrate|code: &[u8], <br />vm_type: u32, <br />name: &str, <br />version: &str, <br />author: &str, <br />email: &str, <br />desc: &str,|Option<Address>|合约升级|
|current_blockhash||H256|获得当前区块hash|
|current_txhash||H256|获得当前交易的hash|
|notify|data: &[u8]||合约中推送事件|
|ret|data: &[u8]||合约执行结束时调用，返回执行结果|