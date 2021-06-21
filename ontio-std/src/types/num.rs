#![allow(clippy::ptr_offset_with_cast, clippy::assign_op_pattern)]
#![allow(clippy::manual_range_contains)]

use core::fmt::{Debug, Display, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Default, Ord)]
pub struct U128(u128);

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Default)]
pub struct I128(i128);

impl I128 {
    pub const fn new(val: i128) -> Self {
        Self(val)
    }
    pub const fn to_u128(self) -> U128 {
        U128(self.0 as u128)
    }

    pub const fn from_le_bytes(bs: [u8; 16]) -> Self {
        I128(i128::from_le_bytes(bs))
    }

    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    pub const fn raw(self) -> i128 {
        self.0
    }
}

impl U128 {
    pub const fn new(val: u128) -> Self {
        U128(val)
    }
    pub const fn from_le_bytes(bs: [u8; 16]) -> Self {
        U128(u128::from_le_bytes(bs))
    }

    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    pub const fn raw(self) -> u128 {
        self.0
    }

    pub const fn to_i128(self) -> I128 {
        I128(self.0 as i128)
    }
}

impl Sum for U128 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(U128::new(0), Add::add)
    }
}

impl<'a> Sum<&'a U128> for U128 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(U128::new(0), Add::add)
    }
}

impl<'a> Add<&'a U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn add(self, rhs: &'a U128) -> Self::Output {
        U128(self.0.checked_add(rhs.0).unwrap())
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
        if let Some(res) = self.0.checked_add(rhs) {
            return U128(res);
        }

        panic!("add overflow {} {}", self.0, rhs)
    }
}

impl Sub<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn sub(self, rhs: u128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs) {
            return U128(res);
        }

        panic!("sub overflow {} {} ", self.0, rhs)
    }
}

impl Sub<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn sub(self, rhs: U128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs.0) {
            return U128(res);
        }
        panic!("sub overflow {} {}", self.0, rhs.0)
    }
}

impl<'a> Sub<&'a U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn sub(self, rhs: &'a U128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs.0) {
            return U128(res);
        }
        panic!("sub overflow {} {}", self.0, rhs.0)
    }
}

impl Mul<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: U128) -> Self::Output {
        if let Some(res) = self.0.checked_mul(rhs.0) {
            return U128(res);
        }
        panic!("mul overflow {} {}", self.0, rhs.0)
    }
}

impl<'a> Mul<&'a U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: &'a U128) -> Self::Output {
        U128(self.0.checked_mul(rhs.0).unwrap())
    }
}

impl Mul<u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: u128) -> Self::Output {
        if let Some(res) = self.0.checked_mul(rhs) {
            return U128(res);
        }
        panic!("mul overflow {} {}", self.0, rhs)
    }
}

impl<'a> Mul<&'a u128> for U128 {
    type Output = U128;

    #[track_caller]
    fn mul(self, rhs: &'a u128) -> Self::Output {
        U128(self.0.checked_mul(*rhs).unwrap())
    }
}

impl Div<U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn div(self, rhs: U128) -> Self::Output {
        U128(self.0.checked_div(rhs.0).unwrap())
    }
}

impl<'a> Div<&'a U128> for U128 {
    type Output = U128;

    #[track_caller]
    fn div(self, rhs: &'a U128) -> Self::Output {
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

mod u256 {
    uint::construct_uint! {
        pub struct U256(4);
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Default, Ord)]
pub struct U256(u256::U256);

impl U256 {
    pub const MAX: U256 = U256(u256::U256::MAX);

    pub const fn new(value: u128) -> Self {
        let mut ret = [0; 4];
        ret[0] = value as u64;
        ret[1] = (value >> 64) as u64;
        U256(u256::U256(ret))
    }

    pub fn as_u128(&self) -> U128 {
        U128(self.0.as_u128())
    }

    pub fn from_little_endian(slice: &[u8]) -> Self {
        U256(u256::U256::from_little_endian(slice))
    }
    pub fn from_big_endian(slice: &[u8]) -> Self {
        U256(u256::U256::from_big_endian(slice))
    }

    pub fn to_be_bytes(self) -> [u8; 32] {
        let mut buf = [0; 32];
        self.0.to_big_endian(&mut buf);
        buf
    }

    pub fn to_le_bytes(&self) -> [u8; 32] {
        let mut buf = [0; 32];
        self.0.to_little_endian(&mut buf);
        buf
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for U256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<u128> for U256 {
    fn from(val: u128) -> Self {
        Self(From::from(val))
    }
}

impl From<U128> for U256 {
    fn from(val: U128) -> Self {
        From::from(val.raw())
    }
}

impl Sum for U256 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(U256::default(), Add::add)
    }
}

impl<'a> Sum<&'a U256> for U256 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(U256::default(), Add::add)
    }
}

impl<'a> Add<&'a U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn add(self, rhs: &'a U128) -> Self::Output {
        U256(self.0.checked_add(u256::U256::from(rhs.0)).unwrap())
    }
}

impl Add<U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn add(self, rhs: U128) -> Self::Output {
        U256(self.0.checked_add(From::from(rhs.0)).unwrap())
    }
}

impl Add<u128> for U256 {
    type Output = U256;

    #[track_caller]
    fn add(self, rhs: u128) -> Self::Output {
        if let Some(res) = self.0.checked_add(From::from(rhs)) {
            return U256(res);
        }
        panic!("add overflow {} {}", self.0, rhs)
    }
}

impl Add<U256> for U256 {
    type Output = U256;

    #[track_caller]
    fn add(self, rhs: U256) -> Self::Output {
        if let Some(res) = self.0.checked_add(rhs.0) {
            return U256(res);
        }
        panic!("add overflow {} {}", self.0, rhs.0)
    }
}

impl<'a> Add<&'a U256> for U256 {
    type Output = U256;

    #[track_caller]
    fn add(self, rhs: &'a U256) -> Self::Output {
        if let Some(res) = self.0.checked_add(rhs.0) {
            return U256(res);
        }
        panic!("add overflow {} {}", self.0, rhs.0)
    }
}

impl Sub<u128> for U256 {
    type Output = U256;

    #[track_caller]
    fn sub(self, rhs: u128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(From::from(rhs)) {
            return U256(res);
        }
        panic!("sub overflow {} {} ", self.0, rhs)
    }
}

impl Sub<U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn sub(self, rhs: U128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(From::from(rhs.0)) {
            return U256(res);
        }
        panic!("sub overflow {} {}", self.0, rhs.0)
    }
}

impl<'a> Sub<&'a U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn sub(self, rhs: &'a U128) -> Self::Output {
        if let Some(res) = self.0.checked_sub(From::from(rhs.0)) {
            return U256(res);
        }
        panic!("sub overflow {} {}", self.0, rhs.0)
    }
}

impl Sub<U256> for U256 {
    type Output = U256;

    #[track_caller]
    fn sub(self, rhs: U256) -> Self::Output {
        if let Some(res) = self.0.checked_sub(rhs.0) {
            return U256(res);
        }
        panic!("sub overflow {} {}", self.0, rhs.0)
    }
}

impl Mul<U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn mul(self, rhs: U128) -> Self::Output {
        if let Some(res) = self.0.checked_mul(From::from(rhs.0)) {
            return U256(res);
        }
        panic!("mul overflow {} {}", self.0, rhs.0)
    }
}

impl<'a> Mul<&'a U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn mul(self, rhs: &'a U128) -> Self::Output {
        U256(self.0.checked_mul(From::from(rhs.0)).unwrap())
    }
}

impl Mul<u128> for U256 {
    type Output = U256;

    #[track_caller]
    fn mul(self, rhs: u128) -> Self::Output {
        if let Some(res) = self.0.checked_mul(From::from(rhs)) {
            return U256(res);
        }
        panic!("mul overflow {} {}", self.0, rhs)
    }
}

impl<'a> Mul<&'a u128> for U256 {
    type Output = U256;

    #[track_caller]
    fn mul(self, rhs: &'a u128) -> Self::Output {
        U256(self.0.checked_mul(From::from(*rhs)).unwrap())
    }
}

impl Mul<U256> for U256 {
    type Output = U256;

    #[track_caller]
    fn mul(self, rhs: U256) -> Self::Output {
        if let Some(res) = self.0.checked_mul(rhs.0) {
            return U256(res);
        }

        panic!("mul overflow {} {}", self.0, rhs.0)
    }
}

impl Div<U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn div(self, rhs: U128) -> Self::Output {
        U256(self.0.checked_div(From::from(rhs.0)).unwrap())
    }
}

impl<'a> Div<&'a U128> for U256 {
    type Output = U256;

    #[track_caller]
    fn div(self, rhs: &'a U128) -> Self::Output {
        U256(self.0.checked_div(From::from(rhs.0)).unwrap())
    }
}

impl Div<u128> for U256 {
    type Output = U256;

    #[track_caller]
    fn div(self, rhs: u128) -> Self::Output {
        U256(self.0.checked_div(From::from(rhs)).unwrap())
    }
}

impl Div<U256> for U256 {
    type Output = U256;

    #[track_caller]
    fn div(self, rhs: U256) -> Self::Output {
        U256(self.0.checked_div(rhs.0).unwrap())
    }
}

impl AddAssign<U128> for U256 {
    #[track_caller]
    fn add_assign(&mut self, rhs: U128) {
        self.0 = self.0.checked_add(From::from(rhs.0)).unwrap()
    }
}

impl AddAssign<U256> for U256 {
    #[track_caller]
    fn add_assign(&mut self, rhs: U256) {
        self.0 = self.0.checked_add(rhs.0).unwrap();
    }
}

impl SubAssign<U128> for U256 {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U128) {
        self.0 = self.0.checked_sub(From::from(rhs.0)).unwrap();
    }
}

impl SubAssign<U256> for U256 {
    #[track_caller]
    fn sub_assign(&mut self, rhs: U256) {
        self.0 = self.0.checked_sub(rhs.0).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::types::U256;

    #[test]
    fn smoke() {
        for _ in 0..100000 {
            let b = u128::max_value();
            let (a, c): (u128, u128) = rand::random();
            let sum = U256::from(a) + U256::from(b);
            let b2 = sum - a;
            assert_eq!(b2.as_u128().raw(), b);

            let a = U256::from(a);
            let mul = a * U256::from(c);
            let c2 = mul / a;
            assert_eq!(c2.as_u128().raw(), c);
        }
    }

    #[test]
    fn small_value() {
        for _ in 0..10000 {
            let a: u64 = rand::random();
            let b: u64 = rand::random();
            let a = a as u128;
            let b = b as u128;

            let sum = U256::from(a) * U256::from(b);

            assert_eq!(sum.as_u128().raw(), a * b, "{} {}", a, b);
        }
    }

    #[test]
    #[should_panic]
    fn test_as_u128_overflow() {
        let val = U256::from(u128::max_value()) + 1000;
        let _ = val.as_u128();
    }

    #[test]
    #[should_panic]
    fn test_u256_overflow() {
        let val = U256::MAX;
        let _ = val.as_u128();
    }
}
