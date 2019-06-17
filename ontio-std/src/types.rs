use crate::vec::Vec;
use crate::String;
use core::mem;
use core::ops::{Deref, DerefMut};
use fixed_hash::construct_fixed_hash;

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

impl Address {
    pub fn to_hex_string(&self) -> String {
        return hexutil::to_hex(&self.0);
    }
}

pub fn to_neo_bytes(data: U256) -> Vec<u8> {
    let mut temp: [u8; 32] = [0; 32];
    data.to_little_endian(&mut temp);
    if let Some(pos) = temp.iter().rev().position(|v| *v != 0) {
        let mut res: Vec<u8> = Vec::new();
        let end = 32 - pos;
        res.extend_from_slice(&temp[0..end]);
        if temp[end - 1] >= 0x80 {
            res.push(0);
        }
        return res;
    } else {
        let mut res: Vec<u8> = Vec::new();
        res.push(0);
        return res;
    }
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
pub struct Hash {
    pub inner: [u8],
}

impl Hash {
    pub(crate) fn from_u8_slice(s: &[u8]) -> &Hash {
        assert_eq!(s.len(), 32);
        unsafe { mem::transmute(s) }
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl Deref for Hash {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Hash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct Addr {
    pub inner: [u8],
}

impl Addr {
    pub(crate) fn from_u8_slice(s: &[u8]) -> &Addr {
        assert_eq!(s.len(), 20);
        unsafe { mem::transmute(s) }
    }
}

impl Address {
    pub fn addr(&self) -> &Addr {
        &Addr::from_u8_slice(&self.0)
    }
}

impl AsRef<Addr> for Address {
    fn as_ref(&self) -> &Addr {
        self.addr()
    }
}

impl AsRef<[u8]> for Addr {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl AsRef<Addr> for Addr {
    fn as_ref(&self) -> &Addr {
        self
    }
}

impl Deref for Addr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Addr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[test]
fn test_to_neo_bytes() {
    use hexutil;
    let raw_data = [0, 128, 1024, 10000, 8380656, 8446192];
    let expected_data = ["0x00", "0x8000", "0x0004", "0x1027", "0xf0e07f", "0xf0e08000"];
    for i in 0..raw_data.len() {
        let data = U256::from(raw_data[i]);
        let res = to_neo_bytes(data);
        let r = hexutil::to_hex(res.as_slice());
        assert_eq!(r, expected_data[i]);
    }
}
