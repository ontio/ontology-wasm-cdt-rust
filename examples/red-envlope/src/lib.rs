#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Decoder, Encoder};
use ostd::abi::{Sink, ZeroCopySource};
use ostd::base58;
use ostd::contract::{ong, ont};
use ostd::database;
use ostd::prelude::*;
use ostd::runtime;

const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
const ONG_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");

const RE_PREFIX: &str = "RE_PREFIX_";
const SENT_PREFIX: &str = "SENT_COUNT_";
const CLAIM_PREFIX: &str = "CLAIM_PREFIX_";

#[derive(Encoder, Decoder)]
struct ReceiveRecord {
    account: Address,
    amount: u64,
}
#[derive(Encoder, Decoder)]
struct EnvlopeStruct {
    token_addr: Address,
    total_amount: u64,
    total_package_count: u64,
    remain_amount: u64,
    remain_package_count: u64,
    records: Vec<ReceiveRecord>,
}

fn create_red_envlope(owner: Address, pack_count: u64, amount: u64, token_addr: Address) -> bool {
    if runtime::check_witness(&owner) == false {
        return false;
    }
    if is_ont_address(&token_addr) {
        assert!(amount >= pack_count);
    }
    let key = [SENT_PREFIX.as_bytes(), owner.as_ref()].concat();
    let mut sent_count = database::get(&key).unwrap_or(0u64);
    sent_count += 1;
    database::put(&key, sent_count);
    let hash_key = [owner.as_ref(), format!("{}", sent_count).as_bytes()].concat();
    let hash = format!("{:?}", runtime::sha256(hash_key));
    let hash_bytes = hash.as_bytes();
    let re_key = [RE_PREFIX.as_bytes(), hash_bytes].concat();
    let self_addr = runtime::address();
    if is_ont_address(&token_addr) {
        let state = ont::State { from: owner.clone(), to: self_addr, amount: U256::from(amount) };
        let res = ont::transfer(&[state]);
        if !res {
            return false;
        }
    } else if is_ong_address(&token_addr) {
        let state = ont::State { from: owner.clone(), to: self_addr, amount: U256::from(amount) };
        let res = ong::transfer(&[state]);
        if !res {
            return false;
        }
    } else {
        let mut sink = Sink::new(16);
        sink.write(("transfer", self_addr, owner, U256::from(amount)));
        let res = runtime::call_contract(&token_addr, sink.bytes());
        if res.is_none() {
            return false;
        }
    }
    let es = EnvlopeStruct {
        token_addr: token_addr.clone(),
        total_amount: amount,
        total_package_count: pack_count,
        remain_amount: amount,
        remain_package_count: pack_count,
        records: Vec::new(),
    };
    database::put(re_key.clone(), es);
    runtime::notify(hash_bytes);
    return true;
}

fn query_envlope(hash: &str) -> String {
    let re_key = [RE_PREFIX, hash].concat();
    let res: Option<EnvlopeStruct> = database::get::<_, EnvlopeStruct>(re_key);
    if let Some(r) = res {
        let mut records: Vec<String> = Vec::new();
        for x in r.records.iter() {
            records.push(format!("account: {}, amount: {}", x.account.to_hex_string(), x.amount));
        }
        return format!("token_addr:{}, total_amount: {}, total_package_count: {}, remain_amount: {}, remain_package_count: {},\
        records:[{:?}]", r.token_addr.to_hex_string(), r.total_amount,r.total_package_count,r.remain_amount,r.remain_package_count,records);
    }
    return "".to_string();
}

fn claim_envlope(account: Address, hash: &str) -> bool {
    if runtime::check_witness(account) == false {
        return false;
    }
    let claim_key = [CLAIM_PREFIX.as_bytes(), hash.as_bytes(), account.as_ref()].concat();
    let claimed = database::get(claim_key.clone()).unwrap_or(0u64);
    if claimed != 0 {
        return false;
    }
    let re_key = [RE_PREFIX, hash].concat();
    let es = database::get::<_, EnvlopeStruct>(re_key.clone());
    if es.is_none() {
        return false;
    }
    let mut est = es.unwrap();
    if est.remain_amount <= 0 {
        return false;
    }
    if est.remain_package_count <= 0 {
        return false;
    }
    let mut record = ReceiveRecord { account: account.clone(), amount: 0 };
    let mut claim_amount = 0;
    if est.remain_package_count == 1 {
        claim_amount = est.remain_amount;
        record.amount = claim_amount;
    } else {
        let random = runtime::current_blockhash();
        let mut part = [0u8; 8];
        part.copy_from_slice(&random.as_bytes()[..8]);
        let random_num = U256::from_little_endian(&part).as_u64();
        let percent = random_num % 100 + 1;
        let mut claim_amount = est.remain_amount * percent / 100;

        if claim_amount == 0 {
            claim_amount = 1;
        } else if is_ont_address(&est.token_addr) {
            if est.remain_amount - claim_amount < est.remain_package_count - 1 {
                claim_amount = est.remain_amount - est.remain_package_count;
            }
        }
        record.amount = claim_amount;
    }
    est.remain_amount -= claim_amount;
    est.remain_package_count -= 1;
    est.records.push(record);
    let self_addr = runtime::address();
    if is_ont_address(&est.token_addr) {
        let state =
            ont::State { from: self_addr, to: account.clone(), amount: U256::from(claim_amount) };
        return ont::transfer(&[state]);
    } else if is_ong_address(&est.token_addr) {
        let state =
            ont::State { from: self_addr, to: account.clone(), amount: U256::from(claim_amount) };
        return ong::transfer(&[state]);
    } else {
        let mut sink = Sink::new(16);
        sink.write(("transfer", self_addr, account, U256::from(claim_amount)));
        let res = runtime::call_contract(&est.token_addr, sink.bytes());
        if res.is_none() {
            return false;
        }
    }
    database::put(claim_key, claim_amount);
    database::put(re_key, est);
    return true;
}

fn is_ong_address(contract_addr: &Address) -> bool {
    contract_addr == &ONG_CONTRACT_ADDRESS
}

fn is_ont_address(contract_addr: &Address) -> bool {
    contract_addr == &ONT_CONTRACT_ADDRESS
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = ZeroCopySource::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"create_red_envlope" => {
            let (owner, pack_count, amount, token_addr) = source.read().unwrap();
            sink.write(create_red_envlope(owner, pack_count, amount, token_addr));
        }
        b"query_envlope" => {
            let hash = source.read().unwrap();
            sink.write(query_envlope(hash));
        }
        b"claim_envlope" => {
            let (account, hash) = source.read().unwrap();
            sink.write(claim_envlope(account, hash));
        }
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}
