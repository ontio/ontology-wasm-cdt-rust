use super::*;
use ontio_std::contract::eth;
use ostd::types::U256;

pub fn balance_of_erc20(caller: &Address, target: &Address, user: &Address) -> U128 {
    let res = eth::evm_invoke(caller, target, gen_eth_balance_of_data(user).as_slice());
    if res.is_empty() {
        return U128::new(0);
    }
    let mut source = Source::new(res.as_slice());
    let h = source.read_h256().unwrap();
    let res = U256::from_big_endian(h.as_bytes());
    res.as_u128()
}

pub fn transfer_erc20(caller: &Address, target: &Address, to: &Address, amount: U128) {
    let res = eth::evm_invoke(caller, target, gen_eth_transfer_data(to, amount).as_slice());
    let mut source = Source::new(res.as_slice());
    let r: &H256 = source.read_h256().unwrap();
    assert!(!r.is_zero(), "transfer_erc20 failed");
}

pub fn transfer_from_erc20(
    caller: &Address, target: &Address, from: &Address, to: &Address, amount: U128,
) {
    let res =
        eth::evm_invoke(caller, target, gen_eth_transfer_from_data(from, to, amount).as_slice());
    let mut source = Source::new(res.as_slice());
    let r: &H256 = source.read_h256().unwrap();
    assert!(!r.is_zero(), "transfer_from_erc20 failed");
}

const TRANSFER_ID: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
const TRANSFER_FROM_ID: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
const BALANCEOF_ID: [u8; 4] = [0x70, 0xa0, 0x82, 0x31];

fn gen_eth_transfer_data(to: &Address, amount: U128) -> Vec<u8> {
    [TRANSFER_ID.as_ref(), format_addr(to).as_ref(), format_amount(amount).as_slice()].concat()
}

fn gen_eth_transfer_from_data(from_acct: &Address, to_acct: &Address, amount: U128) -> Vec<u8> {
    [
        TRANSFER_FROM_ID.as_ref(),
        format_addr(from_acct).as_ref(),
        format_addr(to_acct).as_ref(),
        format_amount(amount).as_slice(),
    ]
    .concat()
}

fn gen_eth_balance_of_data(addr: &Address) -> Vec<u8> {
    [BALANCEOF_ID.as_ref(), format_addr(addr).as_ref()].concat()
}

fn format_addr(addr: &Address) -> [u8; 32] {
    let mut res = [0; 32];
    res[12..].copy_from_slice(addr.as_bytes());
    res
}

fn format_amount(amt: U128) -> Vec<u8> {
    let bs = amt.to_be_bytes();
    let mut res = Vec::with_capacity(32);
    (0..16).into_iter().for_each(|_| res.push(0u8));
    res.extend_from_slice(bs.as_ref());
    res
}

#[test]
fn test() {
    let addr = &Address::repeat_byte(1);
    println!("{:?}", format_addr(&addr).as_ref());
    let data = gen_eth_transfer_data(addr, U128::new(1000));
    println!("{:?}", data.as_slice());
}
