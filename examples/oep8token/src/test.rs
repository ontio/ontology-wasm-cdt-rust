
use crate::{Oep8Token, Oep8TokenInstance};
use ontio_std::types::{Address, U256};
use ontio_std::mock::build_runtime;

#[test]
fn init() {
    let mut token = Oep8TokenInstance;
    let owner = Address::zero();
    build_runtime().witness(&[owner]);
    assert!(token.init());
    let token_id_1 = format!("{}", 1);
    let token_id_2 = format!("{}", 2);
    let token_id_3 = format!("{}", 3);
    let token_id_4 = format!("{}", 4);
    let token_id_5 = format!("{}", 5);
    assert_eq!(token.balance_of(&owner, token_id_1.clone()), U256::from(100000));
    assert_eq!(token.balance_of(&owner, token_id_2.clone()), U256::from(200000));
    assert_eq!(token.balance_of(&owner, token_id_3.clone()), U256::from(300000));
    assert_eq!(token.balance_of(&owner, token_id_4.clone()), U256::from(400000));
    assert_eq!(token.balance_of(&owner, token_id_5.clone()), U256::from(500000));

    assert_eq!(token.total_supply(token_id_1.clone()), U256::from(100000));
    assert_eq!(token.total_supply(token_id_2.clone()), U256::from(200000));
    assert_eq!(token.total_supply(token_id_3.clone()), U256::from(300000));
    assert_eq!(token.total_supply(token_id_4.clone()), U256::from(400000));
    assert_eq!(token.total_supply(token_id_5.clone()), U256::from(500000));

    assert_eq!(token.name(token_id_1.clone()), "TokenNameFirst");
    assert_eq!(token.name(token_id_2.clone()), "TokenNameSecond");
    assert_eq!(token.name(token_id_3.clone()), "TokenNameThird");
    assert_eq!(token.name(token_id_4.clone()), "TokenNameFourth");
    assert_eq!(token.name(token_id_5.clone()), "TokenNameFifth");

    assert_eq!(token.symbol(token_id_1.clone()), "TNF");
}

#[test]
fn transfer() {
    let mut token = Oep8TokenInstance;
    let owner = Address::zero();
    build_runtime().witness(&[owner]);
    assert!(token.init());
    let token_id_1 = format!("{}", 1);
    let alice = Address::random();
    assert!(token.transfer(&owner, &alice, U256::from(10), token_id_1.clone()));
    assert_eq!(token.balance_of(&owner, token_id_1.clone()), U256::from(100000) - U256::from(10));
    assert_eq!(token.balance_of(&alice, token_id_1.clone()), U256::from(10));
    let bob = Address::random();
    let states = [(owner, alice,U256::from(1000), token_id_1.clone()), (owner, bob,U256::from(1000), token_id_1.clone())];
    assert!(token.transfer_multi(&states));
    assert_eq!(token.balance_of(&bob, token_id_1.clone()), U256::from(1000));
}

#[test]
fn approve() {
    let mut token = Oep8TokenInstance;
    let owner = Address::zero();
    let alice = Address::random();
    build_runtime().witness(&[owner, alice]);
    assert!(token.init());
    let token_id_1 = format!("{}", 1);
    assert!(token.approve(&owner, &alice, U256::from(10), token_id_1.clone()));
    assert_eq!(token.allowance(&owner,&alice, token_id_1.clone()), U256::from(10));
    assert!(token.transfer_from(&alice,&owner, &alice, U256::from(10), token_id_1.clone()));
    assert_eq!(token.allowance(&owner,&alice, token_id_1.clone()), U256::from(0));
    assert_eq!(token.balance_of(&alice, token_id_1.clone()), U256::from(10));
}

#[test]
fn approve_multi() {
    let mut token = Oep8TokenInstance;
    let owner = Address::zero();
    let alice = Address::random();
    build_runtime().witness(&[owner, alice]);
    assert!(token.init());
    let token_id_1 = format!("{}", 1);
    let states = [(owner, alice,U256::from(10000), token_id_1.clone())];
    assert!(token.approve_multi(&states));
    let transfer_from_state = [(alice, owner,alice,U256::from(100), token_id_1.clone())];
    assert!(token.transfer_from_multi(&transfer_from_state));
    assert_eq!(token.balance_of(&alice, token_id_1.clone()), U256::from(100));
}




