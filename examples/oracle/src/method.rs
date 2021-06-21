use ostd::abi::EventBuilder;
use ostd::prelude::*;
use ostd::runtime::check_witness;
use ostd::types::U128;

use crate::storage::*;

/**
 * @notice Initialize the money market
 * @param admin The address of the Oracle
 */
pub fn initialize(admin: &Address) -> bool {
    assert!(check_witness(admin), "check witness failed");
    assert!(!is_init(), "already init");
    put_admin(admin);
    init();
    true
}

pub fn put_underlying_price(key_list: Vec<&str>, price_list: Vec<U128>) -> bool {
    assert!(check_witness(&get_admin()), "check witness failed");
    assert_eq!(key_list.len(), price_list.len(), "invalid param");
    let mut event_builder = EventBuilder::new();
    event_builder = event_builder.string("PutUnderlyingPrice");
    for (key, price) in key_list.into_iter().zip(price_list.into_iter()) {
        put_price(key, price);
        event_builder = event_builder.string(key);
        event_builder = event_builder.number(price);
    }
    event_builder.notify();
    true
}
