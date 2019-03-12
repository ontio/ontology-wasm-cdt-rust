/// implement common types
use fixed_hash::construct_fixed_hash;
use crate::vec::Vec;

construct_fixed_hash! {
    pub struct H256(32);
}

construct_fixed_hash! {
    pub struct H160(20);
}

impl AsRef<H160> for H160 {
    fn as_ref(&self) -> &H160 {
        return self;
    }
}

impl AsRef<H256> for H256 {
    fn as_ref(&self) -> &H256 {
        return self;
    }
}

pub type Address = H160;

pub use bigint::U256;

pub fn to_neo_bytes(data: U256) -> Vec<u8> {
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

impl H160 {
    pub const fn new(val: [u8; 20]) -> Self {
        H160(val)
    }
}

impl H256 {
    pub const fn new(val: [u8; 32]) -> Self {
        H256(val)
    }
}


#[test]
fn test_to_neo_bytes() {
    use hexutil;
    let data = U256::from(8380656);
    let res = to_neo_bytes(data);
    let r = hexutil::to_hex(res.as_slice());
    assert_eq!(r, "0xf0e07f");

    let data = U256::from(8446192);
    let res = to_neo_bytes(data);
    let r = hexutil::to_hex(res.as_slice());
    assert_eq!(r, "0xf0e08000");

    let data = U256::from(128);
    let res = to_neo_bytes(data);
    let r = hexutil::to_hex(res.as_slice());
    assert_eq!(r, "0x8000");
}