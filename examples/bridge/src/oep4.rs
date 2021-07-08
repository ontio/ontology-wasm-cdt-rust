use ostd::abi::{VmValueBuilder, VmValueParser};
use ostd::contract::{ong, ont};
use ostd::macros;
use ostd::prelude::*;
use ostd::runtime;

pub const ONT_CONTRACT_ADDRESS: Address = macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
pub const ONG_CONTRACT_ADDRESS: Address = macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");

pub fn balance_of_neovm(contract: &Address, account: &Address) -> U128 {
    if contract == &ONT_CONTRACT_ADDRESS {
        return ont::balance_of(account);
    }
    if contract == &ONG_CONTRACT_ADDRESS {
        return ong::balance_of(account);
    }
    let mut builder = VmValueBuilder::new();
    builder.string("balanceOf");
    let mut nested = builder.list();
    nested.address(account);
    nested.finish();
    call_neovm_num(contract, builder.bytes().as_slice())
}

pub fn transfer_neovm(contract: &Address, from: &Address, to: &Address, amount: U128) {
    if contract == &ONT_CONTRACT_ADDRESS {
        assert!(ont::transfer(from, to, amount), "ont transfer failed");
    } else if contract == &ONG_CONTRACT_ADDRESS {
        assert!(ong::transfer(from, to, amount), "ong transfer failed");
    } else {
        let mut builder = VmValueBuilder::new();
        builder.string("transfer");
        let mut nested = builder.list();
        nested.address(from);
        nested.address(to);
        nested.number(amount);
        nested.finish();
        assert!(call_neovm_bool(contract, builder.bytes().as_slice()), "oep4 transfer failed");
    }
}

#[track_caller]
pub fn call_neovm_num(address: &Address, param: &[u8]) -> U128 {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    source.number().unwrap()
}

#[track_caller]
pub fn call_neovm_bool(address: &Address, param: &[u8]) -> bool {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    source.bool().unwrap()
}
