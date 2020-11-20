extern crate ontio_std as ostd;
extern crate oracle;

use ostd::types::U128;
use ostd::{mock::build_runtime, prelude::*};

use oracle::method;
use oracle::storage;

#[test]
fn test_oracle() {
    let runtime = &build_runtime();
    let admin = Address::repeat_byte(1);
    runtime.witness(&[admin.clone()]);
    method::initialize(&admin);
    storage::put_decimal(9);
    assert_eq!(9, storage::get_decimal());
    method::put_underlying_price(
        vec!["ONT", "BTC", "ETH", "DAI"],
        vec![U128::new(100), U128::new(10000), U128::new(400), U128::new(1)],
    );
    assert_eq!(U128::new(100), storage::get_price("ONT"));
    assert_eq!(U128::new(10000), storage::get_price("BTC"));
    assert_eq!(U128::new(400), storage::get_price("ETH"));
    assert_eq!(U128::new(1), storage::get_price("DAI"));
}

#[test]
#[should_panic]
fn test_init_twice() {
    let runtime = &build_runtime();
    let admin = Address::repeat_byte(1);
    runtime.witness(&[admin.clone()]);
    method::initialize(&admin);
    method::initialize(&admin);
}

#[test]
fn test_set_decimal_twice() {
    let runtime = &build_runtime();
    let admin = Address::repeat_byte(1);
    runtime.witness(&[admin.clone()]);
    method::initialize(&admin);
    storage::put_decimal(9);
    storage::put_decimal(7);
}
