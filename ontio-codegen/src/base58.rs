use sha2::{Sha256, Digest};
use num::bigint::BigUint;
use num::traits::Zero;
use num::traits::cast::ToPrimitive;
use num::integer::Integer;

pub fn dhash256<D: AsRef<[u8]>>(data: D) -> [u8;32] {
    let mut hash = [0;32];
    let dhash =  Sha256::digest(&Sha256::digest(data.as_ref()));
    hash[..].copy_from_slice(&dhash);
    hash
}

const CHARS: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub fn encode_base58(val : &[u8;20]) -> String {
    let mut data = [0u8;25];
    data[0] = 23;
    data[1..21].copy_from_slice(val);
    let hash = dhash256(&data[..21]);
    data[21..].copy_from_slice(&hash[0..4]);
    let b256 = BigUint::from(256u32);
    let mut bigint = data.iter().fold(BigUint::from(0u32), |sum, v| {
        sum*&b256 + v
    });

    let b58 = BigUint::from(58u32);
    let mut chars = Vec::with_capacity(20);
    loop {
        let (left, c) = bigint.div_rem(&b58);
        bigint = left;
        chars.push(CHARS.as_bytes()[c.to_u64().unwrap() as usize]);
        if bigint.is_zero() {
            break
        }
    }
    chars.reverse();

    String::from_utf8(chars).unwrap()
}

pub fn decode_base58(val: &str) -> Option<[u8;20]> {
    //todo
    None
}

#[test]
fn base58_encode() {
    assert_eq!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM", encode_base58(&[0;20]));
}
