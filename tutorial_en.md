
English | [中文](tutorial_cn.md)

# QuickStart

## Related tools
- cargo
- rust
- ontio-cdk

## Start
1. generate ontio-cdk api doc

clone `https://github.com/ontio/ontology-wasm-cdt-rust.git` to local, go to the project root directory, execute `cargo doc` to generate ontio-cdk api doc。

2. Data type conversion in contract
- `u32`,`u64` and other basic data types are converted to U256
- `U256` are converted to `u64`、`u32` and other basic data types

For details, please refer to the method of `U256` in the api documentation.

example
```
let u = U256::from(1);
let n = u.as_u64();
```
- `u64` convert to `string`

example
```
let s = 123.to_string();
```
- `base58` convert to `Address`

example
```
let address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
```

3. Verification signature in the contract

example
```
let flag = runtime::check_witness(&from);
```

4. Interaction between contracts

When calling other contracts in a contract, you need to serialize the parameters according to the parameters serialize standard of the target contract.
- `wasm`contract invoke `neovm` contract
   - `U256` should convert to byte array by `types::to_neo_bytes()` firstly, then invoke `sink.write()` to serialize;
   - `Address` should be serialized by `sink.write_neovm_address()`。
   - parameter serialize step
    1. The method parameters of the target contract are serialized in reverse order.
    2. After the method parameters are serialized, serialize the `83u8` and `193u8` bytecodes.
    3. serialize function name
    4. serialize `103u8`
    5. serialize contract address

    example
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
- `wasm` invoke `native`
   - The contract module encapsulates the method of calling the native contract

## ontio-std instruction

1. `abi` module
- `Sink`  : used to serialization data in contract

For the data type that implements the `Encoder` interface, you can serialize it directly with the `sink.write()` method.
When initializing `sink`, a Vec is initialized and its initialization size needs to be specified.。

example
```
let mut sink = Sink::new(16);
sink.write(83u8);
sink.write("transfer".to_string());
```

- `Source`: used to deserialize of data in contract

For data types that implement the `Decoder` interface type, you can deserialize directly with the `source.read().unwrap()` method.

example
```
let input = runtime::input();
let mut source = ZeroCopySource::new(&input);
let (from, to, amount) = source.read().unwrap();
```

3. console

- `debug`：Used to print log information in contracts

example
```
 console::debug("debug");
```

4. contract
- `ong`：Encapsulates related operations that call ong in the contract, such as transferring, checking balances, and so on.。
  - `allowance(from: &Address, to: &Address)` query allowance balance
    example
    ```
    use ostd::contract::ont;
    ont::allowance(from, to)
    ```
  - `approve(from: &Address, to: &Address, amount: U256)` one address approve another address transfer assets
    example
    ```
    use ostd::contract::ont;
    ont::approve(from, to, amount)
    ```
  - `balance_of` query balance

     example
     ```
     use ostd::contract::ont;
     ong::balance_of(address)
     ```
  - `transfer`

    example
    ```
    let state = ont::State { from: from.clone(), to: to.clone(), amount: amount };
    ont::transfer(&[state])
    ```
  - `transfer_from`
    example
    ```
    ont::transfer_from(sender, from, to, amount)
    ```
- `ont`:Encapsulates related operations that call ont in the contract, Similar to ong。

5. database
- `delete`: delete data by key
- `get`   : query data by key
- `put`   : store data by key

example
```
use ostd::database;
database::put(from, frmbal);
let balance = database::get(owner).unwrap_or(U256::zero());
```

6. types
- `Address`: address is a byte array of length 20
- `U256`   : small endian large integer。

7. runtime

This module encapsulates the API for contract and chain interaction.

- `timestamp() -> u64` get current timestamp

example
```
runtime::timestamp()
```
- `block_height() -> u32` get current block height

example
```
runtime::block_height()
```
- `address() -> Address` get current contract address

example
```
runtime::address()
```

- `caller() -> Address` get caller contract address

example
```
runtime::caller()
```
- `current_blockhash() -> H256` get current block hash

example
```
runtime::current_blockhash()
```
- `current_txhash() -> H256` get current transaction hash

example
```
runtime::current_txhash()
```
- `check_witness<T: AsRef<Addr>>(addr: T) -> bool` verify signature

example
```
runtime::check_witness(addr)
```
- `ret(data: &[u8]) -> !` Called at the end of the contract execution, returning the execution result

example
```
let mut dispatcher = ApiTestDispatcher::new(ApiTestInstance);
runtime::ret(&dispatcher.dispatch(&runtime::input()));
```
- `notify(data: &[u8])` Events pushed in the contract

example
```
runtime::notify("success".as_bytes());
```


## contract examples

[oep4](examples/token-zero-copy)

[oep5](examples/oep5token)

[oep8](examples/oep8token)
