
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

examples
```
let u = U256::from(1);
let n = u.as_u64();
```
- `u64` convert to `string`
examples
```
let s = format!("{}", 123);
```
- `base58` convert to `Address`
examples
```
let address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
```

3. Verification signature in the contract
examples
```
let flag = runtime::check_witness(&from);
```

4. Contract and contract interaction
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

examples
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

examples
```
let mut sink = Sink::new(16);
sink.write(83u8);
sink.write("transfer".to_string());
```

- `Source`: used to deserialize of data in contract
For data types that implement the `Decoder` interface type, you can deserialize directly with the `source.read().unwrap()` method.

examples
```
let input = runtime::input();
let mut source = ZeroCopySource::new(&input);
let (from, to, amount) = source.read().unwrap();
```

3. console

- `debug`：Used to print log information in contracts

examples
```
 console::debug("debug");
```

4. contract
- `ong`：Encapsulates related operations that call ong in the contract, such as transferring, checking balances, and so on.。
 - `allowance(from: &Address, to: &Address)` query allowance balance
examples
```
use ostd::contract::ont;
ont::allowance(from, to)
```
 - `approve(from: &Address, to: &Address, amount: U256)` one address approve another address transfer assets
examples
```
use ostd::contract::ont;
ont::approve(from, to, amount)
```
 - `balance_of` query balance
 examples
 ```
 use ostd::contract::ont;
 ong::balance_of(address)
 ```
 - `transfer`
examples
```
let state = ont::State { from: from.clone(), to: to.clone(), amount: amount };
ont::transfer(&[state])
```
 - `transfer_from`
examples
```
ont::transfer_from(sender, from, to, amount)
```
- `ont`:Encapsulates related operations that call ont in the contract, Similar to ong。

5. database
- `delete`: delete data by key
- `get`   : query data by key
- `put`   : store data by key

examples
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
- `address`: get the current contract address
- `block_height`:get the current block height
- `call_contract`:invoke another contract
- `caller`: get caller's contract address
- `check_witness`: verify signature
- `contract_migrate`：contract upgrade
- `current_blockhash`:get contract hash
- `current_txhash`: get transaction hash
- `notify`:Events pushed in the contract
- `ret`: Called at the end of the contract execution, returning the execution result
- `timestamp`: Get the timestamp of the current block
