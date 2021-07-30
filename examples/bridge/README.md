# 接口文档

请采用wasm合约的交互方式调用此合约

1. getAllTokenPairName

查询所有支持兑换的TokenPair对的名字

* 参数： 无

* 返回值
    * Vec<Vec<u8>>, 二维字节数组， 可以用utf8编码转换成可视化的tokenPairName, 例如 pUSDT-USDT

2. getTokenPair

查询tokenPair信息

* 参数

|参数名|参数类型|
|:---|:---|
|token_pair_name|&[u8], 字节数组|

* 返回值
    * TokenPair, 其定义如下,请按照结构体定义进行解析

```
#[derive(Encoder, Decoder, Default)]
struct TokenPair {
    erc20: Address,
    erc20_decimals: u32,
    oep4: Address,
    oep4_decimals: u32,
}
```    

3. oep4ToErc20

oep4 资产转换成erc20资产

* 参数

|参数名|参数类型|参数描述|
|:---|:---|:---|
|ont_acct|Address|用户的ontology账户地址|
|eth_acct|Address|用户的ethereum账户地址|
|amount|U128|要兑换的oep4资产的数量|
|token_pair_name|&[u8]|要兑换的tokenPair对的名字|


注意： 会校验ont_acct的签名



* 事件

|参数名|参数描述|
|:---|:---|
|oep4ToErc20|方法名|
|ont_acct|用户的ontology账户地址|
|eth_acct|用户的ethereum账户地址|
|amount|要兑换的oep4资产的数量|
|erc20_amt|兑换到的erc20的数量|
|oep4_addr|oep4合约地址|
|erc20_addr|erc20合约地址|



4. erc20ToOep4

erc20资产兑换成oep4资产

* 参数

|参数名|参数类型|参数描述|
|:---|:---|:---|
|ont_acct|Address|用户的ontology账户地址|
|eth_acct|Address|用户的ethereum账户地址|
|amount|U128|要兑换的oep4资产的数量|
|token_pair_name|&[u8]|要兑换的tokenPair对的名字|

注意： 会校验ont_acct的签名


* 事件

|参数名|参数描述|
|:---|:---|
|erc20ToOep4|方法名|
|ont_acct|用户的ontology账户地址|
|eth_acct|用户的ethereum账户地址|
|amount|要兑换的erc20资产的数量|
|oep4_amt|兑换到的oep4_amt的数量|
|oep4_addr|oep4合约地址|
|erc20_addr|erc20合约地址|