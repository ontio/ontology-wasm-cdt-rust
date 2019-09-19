extern crate hexutil;
extern crate ontio_std as ostd;
use crate::{ApiTest, ApiTestInstance};
use ostd::mock::build_runtime;
use ostd::prelude::*;
use ostd::types::{Address, H256};

const _from: Address = ostd::macros::base58!("AeJGmTDUdSzMdrSHU2pa8rLMo23AAs53LM");

const _to: Address = ostd::macros::base58!("AbPRaepcpBAFHz9zCj4619qch4Aq5hJARA");

#[test]
fn test_runtime_api() {
    let mut api = ApiTestInstance;
    build_runtime().timestamp(100);
    assert_eq!(api.timestamp(), 100);

    build_runtime().block_height(100);
    assert_eq!(api.block_height(), 100);

    let self_address = Address::random();
    build_runtime().address(&self_address);
    assert_eq!(api.self_address(), self_address);

    let caller_address = Address::random();
    build_runtime().caller(&caller_address);
    assert_eq!(api.caller_address(), caller_address);

    let entry_address = Address::random();
    build_runtime().entry_address(&entry_address);
    assert_eq!(api.entry_address(), entry_address);

    let current_block_hash = H256::random();
    build_runtime().current_blockhash(&current_block_hash);
    assert_eq!(api.get_current_blockhash(), current_block_hash);

    let current_tx_hash = H256::random();
    build_runtime().current_txhash(&current_tx_hash);
    assert_eq!(api.get_current_txhash(), current_tx_hash);

    let addr = Address::zero();
    assert_eq!(
        api.sha256(&[0; 20]).to_hex_string(),
        "906fd3cbc4401b7ffac44063f02c2693c332e653c3f2b5db00d3b87eb2c947de"
    );
}
