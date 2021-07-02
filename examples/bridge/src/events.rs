use crate::{Address, U128};
use ontio_std::abi::EventBuilder;

pub fn new_pending_admin_event(new_pending_admin: &Address) {
    EventBuilder::new().address(new_pending_admin).notify();
}

pub fn new_admin_event(old_admin: &Address, new_pending_admin: &Address) {
    EventBuilder::new().address(old_admin).address(new_pending_admin).notify();
}

pub fn oep4_to_erc20_event(
    ont_acct: &Address, eth_acct: &Address, amount: U128, delta: U128, oep4_addr: &Address,
    erc20_addr: &Address,
) {
    EventBuilder::new()
        .address(ont_acct)
        .address(eth_acct)
        .number(amount)
        .number(delta)
        .address(oep4_addr)
        .address(erc20_addr)
        .notify();
}

pub fn erc20_to_oep4_event(
    ont_acct: &Address, eth_acct: &Address, amount: U128, delta: U128, oep4_addr: &Address,
    erc20_addr: &Address,
) {
    EventBuilder::new()
        .address(eth_acct)
        .address(ont_acct)
        .number(amount)
        .number(delta)
        .address(oep4_addr)
        .address(erc20_addr)
        .notify();
}
