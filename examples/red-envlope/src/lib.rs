#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Decoder, Encoder};
use ostd::abi::{Sink, Source};
use ostd::contract::{ong, ont};
use ostd::database;
use ostd::macros::base58;
use ostd::macros::event;
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
    amount: u128,
}

#[derive(Encoder, Decoder)]
struct EnvlopeStruct {
    token_addr: Address,
    total_amount: u128,
    total_package_count: u128,
    remain_amount: u128,
    remain_package_count: u128,
    records: Vec<ReceiveRecord>,
}

fn create_red_envlope(owner: Address, pack_count: u128, amount: u128, token_addr: Address) -> bool {
    if !runtime::check_witness(&owner) {
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
        let res = ont::transfer(&owner, &self_addr, amount as u128);
        if !res {
            return false;
        }
    } else if is_ong_address(&token_addr) {
        let res = ong::transfer(&owner, &self_addr, amount as u128);
        if !res {
            return false;
        }
    } else {
        let mut sink = Sink::new(16);
        sink.write(("transfer", self_addr, owner, amount as u128));
        let res = runtime::call_contract(&token_addr, sink.bytes());
        if res.is_none() {
            return false;
        }
    }
    let es = EnvlopeStruct {
        token_addr,
        total_amount: amount,
        total_package_count: pack_count,
        remain_amount: amount,
        remain_package_count: pack_count,
        records: Vec::new(),
    };
    database::put(&re_key, es);
    create_red_envlope_event(
        owner.as_ref(),
        pack_count as U128,
        amount as U128,
        token_addr.as_ref(),
    );
    true
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

    "".to_string()
}

fn claim_envlope(account: &Address, hash: &str) -> bool {
    if !runtime::check_witness(account) {
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
    if est.remain_amount == 0 {
        return false;
    }
    if est.remain_package_count == 0 {
        return false;
    }
    let mut record = ReceiveRecord { account: *account, amount: 0 };
    let mut claim_amount = 0;
    if est.remain_package_count == 1 {
        claim_amount = est.remain_amount;
        record.amount = claim_amount;
    } else {
        let random = runtime::current_blockhash();
        let mut part = [0u8; 16];
        part.copy_from_slice(&random.as_bytes()[..8]);
        let random_num = U128::from_le_bytes(part) as u64;
        let percent = random_num % 100 + 1;
        let mut claim_amount = est.remain_amount * percent as u128 / 100;

        if claim_amount == 0 {
            claim_amount = 1;
        } else if is_ont_address(&est.token_addr)
            && est.remain_amount - claim_amount < est.remain_package_count - 1
        {
            claim_amount = est.remain_amount - est.remain_package_count;
        }
        record.amount = claim_amount;
    }
    est.remain_amount -= claim_amount;
    est.remain_package_count -= 1;
    est.records.push(record);
    let self_addr = runtime::address();
    if is_ont_address(&est.token_addr) {
        return ont::transfer(&self_addr, &account, claim_amount as u128);
    } else if is_ong_address(&est.token_addr) {
        return ong::transfer(&self_addr, &account, claim_amount as u128);
    } else {
        let mut sink = Sink::new(16);
        sink.write(("transfer", self_addr, account, claim_amount));
        let res = runtime::call_contract(&est.token_addr, sink.bytes());
        if res.is_none() {
            return false;
        }
    }
    database::put(claim_key, claim_amount);
    database::put(re_key, est);
    claim_envlope_event(account, hash);
    true
}

fn is_ong_address(contract_addr: &Address) -> bool {
    contract_addr == &ONG_CONTRACT_ADDRESS
}

fn is_ont_address(contract_addr: &Address) -> bool {
    contract_addr == &ONT_CONTRACT_ADDRESS
}

#[event]
fn create_red_envlope_event(owner: &Address, pack_count: U128, amount: U128, token_addr: &Address) {
}

#[event]
fn claim_envlope_event(account: &Address, hash: &str) {}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
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
