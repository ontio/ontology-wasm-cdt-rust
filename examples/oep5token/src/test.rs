use crate::{Oep5Token, Oep5TokenInstance};
use ontio_std::mock::build_runtime;
use ontio_std::types::{Address, U128};

#[test]
fn initialize() {
    let mut token = Oep5TokenInstance;
    let owner = Address::zero();
    assert_eq!(token.initialize(&owner), true);
    assert_eq!(token.total_supply(), token.balance_of(&owner))
}

#[test]
fn query_token_id_by_index() {
    let mut token = Oep5TokenInstance;
    let owner = Address::zero();
    assert_eq!(token.initialize(&owner), true);
    let token_id = token.query_token_id_by_index(U128::new(1));
    println!("token_id: {}", token_id);
    assert_eq!("http://images.com/hearta.jpg", token.query_token_by_id(token_id));
}

#[test]
fn transfer() {
    let mut token = Oep5TokenInstance;
    let owner = Address::zero();
    assert_eq!(token.initialize(&owner), true);
    assert_eq!(token.total_supply(), U128::new(2));
    let token_id = token.query_token_id_by_index(U128::new(1));
    assert_eq!(token.owner_of(token_id.clone()), owner.clone());
    let alice = Address::repeat_byte(1);
    build_runtime().witness(&[owner]);
    assert_eq!(token.transfer(&alice, token_id.clone()), true);
    assert_eq!(token.owner_of(token_id.clone()), alice);
}

#[test]
fn transfer_multi() {
    let mut token = Oep5TokenInstance;
    let owner = Address::zero();
    build_runtime().witness(&[owner]);
    assert_eq!(token.initialize(&owner), true);
    let alice = Address::repeat_byte(1);
    let bob = Address::repeat_byte(2);
    let token_id_1 = token.query_token_id_by_index(U128::new(1));
    let token_id_2 = token.query_token_id_by_index(U128::new(2));
    assert_eq!(token.owner_of(token_id_1.clone()), owner);
    assert_eq!(token.owner_of(token_id_2.clone()), owner);
    let states = [(alice.clone(), token_id_1.clone()), (bob.clone(), token_id_2.clone())];
    assert_eq!(token.transfer_multi(&states), true);
    assert_eq!(token.owner_of(token_id_1.clone()), alice);
    assert_eq!(token.owner_of(token_id_2.clone()), bob);
}

#[test]
fn approve() {
    let mut token = Oep5TokenInstance;
    let owner = Address::zero();
    let alice = Address::repeat_byte(1);
    build_runtime().witness(&[owner, alice]);
    assert_eq!(token.initialize(&owner), true);
    let token_id = token.query_token_id_by_index(U128::new(1));
    assert_eq!(token.approve(&alice, token_id.clone()), true);
    assert_eq!(token.get_approved(token_id.clone()), alice.clone());
    assert_eq!(token.owner_of(token_id.clone()), owner.clone());
    assert_eq!(token.take_ownership(token_id.clone()), true);
    assert_eq!(token.owner_of(token_id.clone()), alice.clone());
}
