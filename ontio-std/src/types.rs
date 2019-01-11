/// implement common types
use fixed_hash::construct_fixed_hash;

construct_fixed_hash! {
    pub struct H256(32);
}

construct_fixed_hash! {
    pub struct H160(20);
}

pub type Address = H160;

pub use bigint::U256;
