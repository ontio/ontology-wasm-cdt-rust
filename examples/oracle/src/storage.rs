use ostd::database;
use ostd::prelude::*;
use ostd::types::U128;

/**
 * @notice Administrator for this contract
 */
const ADMIN_KEY: &str = "admin";

pub fn put_admin(admin: &Address) {
    database::put(ADMIN_KEY, admin);
}

pub fn get_admin() -> Address {
    database::get(ADMIN_KEY).unwrap()
}

const INIT: &str = "init";

pub fn init() {
    database::put(INIT, true);
}

pub fn is_init() -> bool {
    database::get(INIT).unwrap_or(false)
}

/**
 * @notice Price data for this contract
 */
const PRICE: &str = "price";

pub fn put_price(key: &str, price: U128) {
    database::put(PRICE.to_string() + key, price);
}

pub fn get_price(key: &str) -> U128 {
    database::get(PRICE.to_string() + key).unwrap()
}

/**
 * @notice Decimal for this contract
 */
const DECIMAL: &str = "decimal";

pub fn put_decimal(decimal: u8) -> bool {
    database::put(DECIMAL, decimal);
    true
}

pub fn get_decimal() -> u8 {
    database::get(DECIMAL).unwrap()
}
