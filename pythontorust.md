# Python合约和rust合约对比

## 新建合约
新建Python版本合约，建议开发者使用smartx在线开发工具进行开发测试。

rust版本合约可以使用cargo命令新建合约。
```rust
//创建rust合约示例
cargo new --lib helloworld
```
## 合约入口函数
python 合约中main函数实现函数跳转

rust 合约中invoke函数根据参数不同调用指定的函数

示例

python 版本合约main函数示例
```python
def main(operation, args):
    if operation = "init":
        return init()
    else:
        return False
```
rust版本合约示例
```rust
#[no_mangle]
pub fn invoke() {
    //Oep8TokenDispatcher是使用abi_codegen::contract自动生成的类，实现了对合约请求的自动派发和结果的序列化操作
    let mut dispatcher = Oep8TokenDispatcher::new(Oep8TokenInstance);//通过代码生成器，生成Oep8TokenDispatcher对象实例
    runtime::ret(&dispatcher.dispatch(&runtime::input()));//
}
```
## 包引用
Python版本合约需要引用boa包目录下的库函数
示例
```python
from boa.interop.System.Storage import Get, GetContext, Put
```
rust版本合约需要引用ontio-std库下面的库函数
示例
```rust
extern crate ontio_std as ostd;
use ostd::{database, runtime};
```

## 常量定义
python 合约中常量定义
```python
INITIALIZED = "init"
```
rust合约中常量定义
```rust
const INITED: &str = "Initialized";
```

## 数据存储
python 合约需要引入`Put` 和 `Get`保存和读取数据。
示例
```python
Put(GetContext(), KEY, value)//GetContext用于获取合约上下文信息
```

rust合约需要引入`database`模块中的`put`和`get`方法，此外，rust合约支持`ListStore`和`HashmapStore`数据类型，
`ListStore`和`HashMapStore`是对`Put`和`Get`的进一步封装，均支持遍历所有数据，方法执行结束自动保存功能。

rust合约中`put`和`get`方法示例
```rust
database::put(INITED, true);
let val :bool = database::get(INITED).unwrap_or_default();
```
ListStore使用示例
```rust
//引用database
use database::ListStore;
fn init(){
    let mut list: ListStore<String> = ListStore::open("key".to_string());//新建List实例
    list.push("value".to_string());
    list.push("sss".to_string());
}
```
HashMapStore使用示例
```rust
use database::HashMapStore;
let mut m = HashMapStore::open("test".to_string());
m.put("hello", "world");
```

## 合约编译成字节码
python合约通过smartx编译成字节码

rust合约编译成字节码使用下面的命令
```rust
cargo build --release --target wasm32-unknown-unknown
```
>注意：在使用上面命令之前，请先通过下面的命令安装`wasm32-unknown-unknown`插件，
```rust
rustup target add wasm32-unknown-unknown
```
## 合约测试
smartx工具支持python合约单步调试等功能

rust合约测试比较方便，在合约文件中添加下面代码
```rust
#[cfg(test)]
mod test;
```
然后就可以编写单独的测试文件，详情请查看`examples`目录下的合约例子

## rust合约常用函数示例
1. base58编码的地址转换成Address对象实例
```rust
const _ADDR_EMPTY: Address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
```
2. u32转换成U256类型的数据
```rust
U256::from(1)
```
3. 校验签名

```rust
use ostd::{database, runtime};
runtime::check_witness(&owner);
```
4. 合约中可以直接用`assert_eq!`和`assert!`等判断条件是true还是false。
5. rust合约中ListStore的使用介绍
* 新建或者打开已经存在的一个ListStore
```rust
let mut list: ListStore<String> = ListStore::open("key".to_string());
```
>Note:如果数据库

* 添加元素
```rust
list.push("value".to_string());
```
ListStore中添加的元素，需要调用flush方法才会保存到数据库中，当执行list的合约方法结束的时候合约会自动调用flush方法将list中的数据保存到数据库。

* 删除元素
按照索引删除元素，所以需要用户知道要删除的元素的索引
```rust
list.remove(1);
```
* 查询元素
根据索引查询元素
```rust
let x = list.get(1);
```
* 打开已经存在List
```rust
let list: ListStore<String> = ListStore::open("key".to_string());
```
* 遍历list
```rust
while let Some(data) = iter.next() {
   println!("{}", data);
}
```

6. HashMapStore使用介绍
* 新建一个HashMapStore
```rust
let mut m:HashMapStore<String, String> = HashMapStore::open("test".to_string());
```

* 添加元素
```rust
m.put(format!("hello{}", i), format!("world{}", i));
```
* 查询元素
根据key获得value
```rust
m.get(&"hello1".to_string()).unwrap();
```
* 访问已经存在数据库中的hashmap格式的数据
```rust
let mut m2: HashMap<String, String> = HashMap::open("test".to_string());
```

* 删除元素
根据key删除元素
```rust
m.remove("hello0");
```

* 遍历HashMapStore
```rust
let mut iter = m.iter();
let mut ind = 0;
while let Some((k, v)) = iter.next() {
    assert_eq!(k, &format!("hello{}", ind));
    assert_eq!(v, format!("world{}", ind));
    ind += 1;
}
```
