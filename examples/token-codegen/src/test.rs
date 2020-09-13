use crate::{MyToken, MyTokenInstance};
use ontio_std::mock::build_runtime;
use ontio_std::types::{Address, U128};

#[test]
fn get_total_supply() {
    let mut token = MyTokenInstance;
    let owner = &Address::random();
    assert!(token.initialize(owner));
    assert_eq!(token.initialize(owner), false);
    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total);
}

#[test]
fn transfer_no_witness() {
    let mut token = MyTokenInstance;
    let owner = &Address::random();
    let b = &Address::random();
    assert!(token.initialize(owner));
    assert_eq!(token.transfer(owner, b, U128::new(123)), false);
}

#[test]
fn transfer() {
    let owner = &Address::random();
    let b = &Address::random();
    build_runtime().witness(&[owner]);
    let mut token = MyTokenInstance;
    assert!(token.initialize(owner));

    assert_eq!(token.transfer(owner, b, U128::new(123)), true);
    assert_eq!(token.balance_of(b), U128::new(123));

    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total - U128::new(123));
}

#[test]
fn approve() {
    let owner = &Address::random();
    let alice = &Address::random();
    let mut token = MyTokenInstance;
    let handle = build_runtime();
    handle.witness(&[owner]);
    assert!(token.initialize(owner));
    assert!(token.approve(owner, alice, U128::new(100)));
    assert_eq!(token.allowance(owner, alice), U128::new(100));
    assert!(!token.transfer_from(alice, owner, U128::new(100)));
    handle.witness(&[alice]);
    assert!(token.transfer_from(alice, owner, U128::new(100)));
    assert_eq!(token.allowance(owner, alice), U128::new(0));
}

#[test]
fn transfer_multi() {
    let owner = &Address::random();
    let alice = &Address::random();
    let bob = &Address::random();
    build_runtime().witness(&[owner, alice]);
    let mut token = MyTokenInstance;
    assert!(token.initialize(owner));
    let states =
        [(owner.clone(), alice.clone(), U128::new(1)), (owner.clone(), bob.clone(), U128::new(2))];
    assert_eq!(token.transfer_multi(&states), true);
    assert_eq!(token.balance_of(&alice), U128::new(1));
    assert_eq!(token.balance_of(&bob), U128::new(2));
}
