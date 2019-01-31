use sha2::{Sha256, Digest};
use num::bigint::{ToBigUint, BigUint};
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

pub fn decode_base58(val: &str) -> Result<[u8;20], String> {
    let mut temp = val.as_bytes().to_vec();
    let new_val = String::from_utf8(temp).unwrap();
    let b58 = BigUint::from(58u32);
    let mut bigint: BigUint = Zero::zero();
    for c in new_val.chars() {
        match CHARS.find(c) {
            None => {
                return Err("invalid char".to_string())
            },
            Some(x) => {
                bigint = bigint * &b58 + x.to_biguint().unwrap();
            }
        }
    }
    let b256 = BigUint::from(256u32);
    let mut origin_data = Vec::with_capacity(25);
    loop {
        let (left, c) = bigint.div_rem(&b256);
        bigint = left;
        origin_data.push(c.to_u8().unwrap_or_default());
        if bigint.is_zero() {
            break
        }
    }
    origin_data.reverse();
    if origin_data.len() != 25 {
        return Err("error length".to_string());
    }
    let hash = dhash256(&origin_data[..21]);
    if &origin_data[21..] != &hash[0..4] {
        return Err("Hash check failed".to_string());
    }
    let mut res = [0u8;20];
    res.copy_from_slice(&origin_data[1..21]);
    Ok(res)
}

#[test]
fn base58_encode() {
    assert_eq!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM", encode_base58(&[0;20]));
    assert_eq!([0;20], decode_base58(encode_base58(&[0;20]).as_str()).unwrap());

}
