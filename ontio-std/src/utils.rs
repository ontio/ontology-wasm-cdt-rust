use crate::types::num;
use core::cmp;
use core::intrinsics::{unchecked_div, wrapping_add};
use core::mem::size_of;

#[macro_export]
macro_rules! implement_sqrt {
    ($name:ident, $ty:ty, $one:expr) => {
        /// calcuate sqrt of integer `n` using Newton method, return value `v`satisfy `v^2 <= n < (v+1)^2`.
        pub fn $name(n: $ty) -> $ty {
            if n <= $one {
                return n;
            }
            let mut xk = n >> ((8 * size_of::<$ty>() as u32 - n.leading_zeros()) / 2);
            xk = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;
            xk = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;
            let mut xkn = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;
            let (mut max, mut min) = (cmp::max(xk, xkn), cmp::min(xk, xkn));
            while max - min > $one {
                // div is safe since xkn will never be 0
                xk = wrapping_add(xkn, unsafe { unchecked_div(n, xkn) }) / 2;
                xk = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;
                xk = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;
                xkn = wrapping_add(xk, unsafe { unchecked_div(n, xk) }) / 2;

                max = cmp::max(xk, xkn);
                min = cmp::min(xk, xkn);
            }

            min
        }
    };
}

implement_sqrt!(sqrt128, u128, 1u128);
implement_sqrt!(sqrt64, u64, 1u64);
implement_sqrt!(sqrt32, u32, 1u32);
implement_sqrt!(sqrt256, num::u256::U256, num::u256::U256::one());

#[test]
fn test_isqrt() {
    for n in 0..100000 {
        let r = sqrt128(n);
        assert!(r * r <= n);
        assert!((r + 1) * (r + 1) > n);
    }
}

#[test]
fn test_sqrt256() {
    for n in 0..100000 {
        let r = sqrt128(U256::new(n as u128).0);
        assert!(r * r <= n);
        assert!((r + 1) * (r + 1) > n);
    }
}
