/// implement common types
use fixed_hash::construct_fixed_hash;

construct_fixed_hash! {
    pub struct H256(32);
}

construct_fixed_hash! {
    pub struct H160(20);
}

impl AsRef<H160> for H160 {
    fn as_ref(&self) -> &H160 {
        return self
    }
}

impl AsRef<H256> for H256 {
    fn as_ref(&self) -> &H256 {
        return self
    }
}

pub type Address = H160;

pub use bigint::U256;
