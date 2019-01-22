use crate::{MyToken, MyTokenInstance};
use ontio_std::types::{U256, Address};
use ontio_std::mock::{setup_runtime, RuntimeBuilder};

#[test]
fn get_total_supply() {
    let mut token = MyTokenInstance;
    let owner = Address::zero();
    assert!(token.initialize(owner.clone()));
    assert_eq!(token.initialize(owner.clone()), false);
    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total);
}

#[test]
fn transfer_nowitness() {
    let mut token = MyTokenInstance;
    let owner = Address::zero();
    let b = Address::repeat_byte(1);
    assert!(token.initialize(owner.clone()));
    assert_eq!(token.transfer(owner.clone(), b.clone(), U256::from(123)), false);
}

#[test]
fn transfer() {
    let owner = Address::zero();
    let b = Address::repeat_byte(1);
    setup_runtime(RuntimeBuilder::default().append_witness(&owner).build());
    let mut token = MyTokenInstance;
    assert!(token.initialize(owner.clone()));

    assert_eq!(token.transfer(owner.clone(), b.clone(), U256::from(123)), true);
    assert_eq!(token.balance_of(b.clone()), U256::from(123));

    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total - U256::from(123));
}
