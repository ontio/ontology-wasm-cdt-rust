
use crate::{Oep8Token, Oep8TokenInstance};
use ontio_std::types::Address;
use ontio_std::mock::build_runtime;

#[test]
fn init() {
    let mut token = Oep8TokenInstance;
    let owner = Address::random();
    build_runtime().witness(&[owner]);
    assert!(token.init());
}
