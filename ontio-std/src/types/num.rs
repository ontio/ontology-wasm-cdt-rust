use core::fmt::{Debug, Display, Result};
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use fixed_hash::static_assertions::_core::fmt::Formatter;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Default)]
pub struct U128(u128);

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Default)]
pub struct I128(i128);

impl I128 {
    pub const fn new(val: i128) -> Self {
        Self(val)
    }
    pub const fn to_u128(self) -> U128 {
        U128(self.0 as u128)
    }

    pub fn from_le_bytes(bs: [u8; 16]) -> Self {
        I128(i128::from_le_bytes(bs))
    }

    pub fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    pub fn raw(self) -> i128 {
        self.0
    }
}

impl U128 {
    pub const fn new(val: u128) -> Self {
        U128(val)
    }
    pub fn from_le_bytes(bs: [u8; 16]) -> Self {
        U128(u128::from_le_bytes(bs))
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    pub fn raw(self) -> u128 {
        self.0
    }

    pub fn to_i128(self) -> I128 {
        I128(self.0 as i128)
    }
}

impl Add<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn add(self, rhs: U128) -> Self::Output {
        U128(self.0.checked_add(rhs.0).unwrap())
    }
}

impl Add<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn add(self, rhs: u128) -> Self::Output {
        U128(self.0.checked_add(rhs).unwrap())
    }
}

impl Sub<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn sub(self, rhs: u128) -> Self::Output {
        U128(self.0.checked_sub(rhs).unwrap())
    }
}

impl Sub<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn sub(self, rhs: U128) -> Self::Output {
        U128(self.0.checked_sub(rhs.0).unwrap())
    }
}

impl Mul<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: U128) -> Self::Output {
        U128(self.0.checked_mul(rhs.0).unwrap())
    }
}

impl Mul<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: u128) -> Self::Output {
        U128(self.0.checked_mul(rhs).unwrap())
    }
}

impl Div<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn div(self, rhs: U128) -> Self::Output {
        U128(self.0.checked_div(rhs.0).unwrap())
    }
}

impl Div<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn div(self, rhs: u128) -> Self::Output {
        U128(self.0.checked_div(rhs).unwrap())
    }
}

impl AddAssign<U128> for U128 {
    #[track_caller]
    fn add_assign(&mut self, rhs: U128) {
        self.0 = self.0.checked_add(rhs.0).unwrap()
    }
}

impl AddAssign<u128> for U128 {
    #[track_caller]
    fn add_assign(&mut self, rhs: u128) {
        self.0 = self.0.checked_add(rhs).unwrap();
    }
}

impl SubAssign<U128> for U128 {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U128) {
        self.0 = self.0.checked_sub(rhs.0).unwrap();
    }
}

impl SubAssign<u128> for U128 {
    #[track_caller]
    fn sub_assign(&mut self, rhs: u128) {
        self.0 = self.0.checked_sub(rhs).unwrap();
    }
}

impl Display for U128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for U128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}
