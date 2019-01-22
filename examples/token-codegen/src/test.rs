use crate::{MyToken, MyTokenInstance};
use ontio_std::types::Address;

#[test]
fn get_total_supply() {
    let mut token = MyTokenInstance;
    let owner = Address::zero();
    assert!(token.initialize(owner.clone()));
    assert_eq!(token.initialize(owner.clone()), false);
    let total = token.total_supply();
    assert_eq!(token.balance_of(owner), total);
}
