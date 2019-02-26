use crate::prelude::*;
use fixed_hash::construct_fixed_hash;

construct_fixed_hash! {
    pub struct H256(32);
}

construct_fixed_hash! {
    pub struct H160(20);
}

impl AsRef<H160> for H160 {
    fn as_ref(&self) -> &H160 {
        self
    }
}

impl AsRef<H256> for H256 {
    fn as_ref(&self) -> &H256 {
        self
    }
}
impl H256 {
    pub fn to_hex_string(&self) -> String {
        to_hex_string_reverse(&self.0)
    }
}

fn to_hex_string_reverse(data: &[u8]) -> String {
    use core::fmt::Write;
    let mut s = String::with_capacity(data.len() * 2);
    for v in data.iter().rev() {
        write!(s, "{:02x}", *v).unwrap();
    }
    s
}

#[allow(unused)]
fn to_hex_string(data: &[u8]) -> String {
    use core::fmt::Write;
    let mut s = String::with_capacity(data.len() * 2);
    for v in data {
        write!(s, "{:02x}", *v).unwrap();
    }
    s
}

pub type Address = H160;

pub type U128 = u128;
pub type I128 = i128;

impl Address {
    pub fn to_hex_string(&self) -> String {
        to_hex_string_reverse(&self.0)
    }
}

pub fn u128_to_neo_bytes(data: U128) -> Vec<u8> {
    let temp = data.to_le_bytes();
    if let Some(pos) = temp.iter().rev().position(|v| *v != 0) {
        let mut res: Vec<u8> = Vec::new();
        let end = temp.len() - pos;
        res.extend_from_slice(&temp[0..end]);
        if temp[end - 1] >= 0x80 {
            res.push(0);
        }
        res
    } else {
        vec![0]
    }
}

pub fn i128_to_neo_bytes(data: I128) -> Vec<u8> {
    if data >= 0 {
        return u128_to_neo_bytes(data as u128);
    }
    let temp = data.to_le_bytes();
    if let Some(pos) = temp.iter().rev().position(|v| *v != 255) {
        let mut res: Vec<u8> = Vec::new();
        let end = temp.len() - pos;
        res.extend_from_slice(&temp[0..end]);
        if temp[end - 1] < 0x80 {
            res.push(255);
        }

        res
    } else {
        vec![255]
    }
}

pub fn u128_from_neo_bytes(buf: &[u8]) -> U128 {
    if buf.is_empty() {
        return 0;
    }
    let neg = buf[buf.len() - 1] >= 0x80;
    let default = if neg { i128::min_value() as u128 } else { i128::max_value() as u128 };

    let mut result = [0u8; 16];
    if (buf.len() > 16 && neg) || (buf.len() > 17 && !neg) {
        return default;
    }
    if buf.len() == 17 && buf[16] != 0 {
        return default;
    }

    let copy = cmp::min(buf.len(), 16);
    {
        let (left, right) = result.split_at_mut(copy);
        left.copy_from_slice(&buf[0..copy]);
        if neg {
            right.iter_mut().for_each(|v| *v = 255);
        }
    }

    U128::from_le_bytes(result)
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
    let case_data = [
        (0i128, "00"),
        (128, "8000"),
        (1024, "0004"),
        (10000, "1027"),
        (8380656, "f0e07f"),
        (8446192, "f0e08000"),
        (-1, "ff"),
        (1, "01"),
        (120, "78"),
        (128, "8000"),
        (255, "ff00"),
        (1024, "0004"),
        (-9223372036854775808, "0000000000000080"),
        (9223372036854775807, "ffffffffffffff7f"),
        (90123123981293054321, "71e975a9c4a7b5e204"),
    ];

    for (data, exp) in case_data.into_iter() {
        let res = i128_to_neo_bytes(*data);
        let r = to_hex_string(res.as_slice());
        assert_eq!(r, exp.to_string());

        let u = u128_from_neo_bytes(&res);
        assert_eq!(u, *data as u128);
    }
}

#[test]
fn test_from_neo_bytes() {
    for _i in 0..100000 {
        let v: i128 = rand::random();
        let bs = u128_to_neo_bytes(v as U128);

        let u = u128_from_neo_bytes(&bs);
        assert_eq!(v as U128, u);
    }

    for _i in 0..100000 {
        let v: u128 = rand::random();
        let bs = u128_to_neo_bytes(v);

        let u = u128_from_neo_bytes(&bs);
        assert_eq!(v as U128, u);
    }
}
