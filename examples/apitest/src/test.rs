extern crate hexutil;
extern crate ontio_std as ostd;
use crate::{ApiTest, ApiTestInstance};
use ostd::mock::build_runtime;
use ostd::types::{Address, H256};
use ostd::vec::Vec;

const _from: Address = ostd::base58!("AeJGmTDUdSzMdrSHU2pa8rLMo23AAs53LM");

const _to: Address = ostd::base58!("AbPRaepcpBAFHz9zCj4619qch4Aq5hJARA");

#[test]
fn call_transfer() {
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

    let current_block_hash = H256::new([0u8; 32]);
    build_runtime().current_blockhash(&current_block_hash);
    assert_eq!(api.get_current_blockhash(), current_block_hash);

    let current_tx_hash = H256::new([0u8; 32]);
    build_runtime().current_txhash(&current_tx_hash);
    assert_eq!(api.get_current_txhash(), current_tx_hash);
}
