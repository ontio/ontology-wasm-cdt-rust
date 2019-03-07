extern crate hexutil;
extern crate ontio_std as ostd;
use crate::{ApiTest,ApiTestInstance};
use ostd::types::Address;
use ostd::abi::{Sink, Source};
use ostd::types::{U256,to_neo_bytes};
use ostd::vec::Vec;

const _from: Address = ostd::base58!("AeJGmTDUdSzMdrSHU2pa8rLMo23AAs53LM");

const _to: Address = ostd::base58!("AbPRaepcpBAFHz9zCj4619qch4Aq5hJARA");

#[test]
fn call_trasnfer2() {
    let mut api = ApiTestInstance;
    api.call_native_transfer2(1u8,&_from,&_to,U256::from(500));
    assert_eq!(false, true)
}

#[test]
fn call_neovm_transfer() {
    let amount = U256::from(500);
    let mut sink = Sink::new(16);
    sink.write(u256_to_neo_bytes(amount));
    sink.write_varuint(20);
    sink.write(_to);
    sink.write_varuint(20);
    sink.write(_from);
    sink.write(83u8);
    sink.write(193u8);
    sink.write("transfer".to_string());
    let d = sink.into();
    println!("res:{}", hexutil::to_hex(d.as_slice()));
    assert_eq!(false, true)
}

#[test]
fn call_native2() {
    let mut amount = U256::from(500);
    let mut sink = Sink::new(16);
    sink.write_native_varuint(1u64);
    sink.write_native_address(&_from);
    sink.write_native_address(&_to);
    sink.write(u256_to_neo_bytes(amount));
    let data = sink.into();
    println!("{:?}", data);
//    println!("{}", hex::encode(data.as_slice()));
    assert_eq!(false, true);
}
#[test]
fn call_native() {
    let mut amount = U256::from(500);
    let mut sink2 = Sink::new(16);

    //state length
    sink2.write_varuint(1);
    sink2.write_varuint(1u64);
    sink2.write_varuint(20);
    sink2.write(_from);
    sink2.write_varuint(20);
    sink2.write(_to);
    sink2.write(u256_to_neo_bytes(amount));

    let data = sink2.into();
    println!("{:?}", data);
    println!("{}", hex::encode(data.as_slice()));
    assert_eq!(false, true);
}

fn u256_to_neo_bytes(data: U256) -> Vec<u8> {
    let mut res:Vec<u8> = Vec::new();
    if data.is_zero() {
        res.push(0);
        return res;
    }
    let mut temp = [0u8;32];
    data.to_big_endian(&mut temp);
    let mut f = false;
    for i in temp.iter() {
        if res.len() ==0 && *i>240u8 {
            f = true;
        }
        if res.len()!=0 || *i != 0u8 {
            res.push(*i);
        }
    }
    res.reverse();
    if f {
        res.push(0);
    }
    res
}
#[test]
fn te() {
    let mut a = U256::from(255);
    let res = u256_to_neo_bytes(a);
    let res2 = to_neo_bytes(a);
    assert_eq!(res, res2);
}

#[test]
fn timestamp(){
//    let data = U256::from(255);
    let  data = U256::from_dec_str("90123123981293054321").unwrap();
    let mut temp = [0u8;32];
    data.to_little_endian(&mut temp);
    temp.reverse();
//    println!("temp:{}", hex::encode(temp.to_vec()));
    let mut res:Vec<u8> = Vec::new();
    if data.is_zero() {
        res.push(0);
//        println!("res:{}", hex::encode(res));
        assert_eq!(false, true);
        return;
    }
    let mut f = false;
    for i in temp.iter() {
        if res.len() ==0 && *i>240u8 {
            f = true;
        }
        if res.len()!=0 || *i != 0u8 {
            res.push(*i);
        }
    }
    if f {
        res.push(0);
    }

    assert_eq!(false, true);
}
