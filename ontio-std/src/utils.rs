use core::cmp;
use core::intrinsics::unchecked_div;
use core::mem::size_of;

macro_rules! implement_sqrt {
    ($name:ident, $ty:ty) => {
        /// calcuate sqrt of integer `n` using Newton method, return value `v`satisfy `v^2 <= n < (v+1)^2`.
        pub fn $name(n: $ty) -> $ty {
            if n < 2 {
                return n;
            }
            let mut xk = n >> ((8 * size_of::<$ty>() as u32 - n.leading_zeros()) / 2);
            xk = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;
            xk = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;
            let mut xkn = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;
            let (mut max, mut min) = (cmp::max(xk, xkn), cmp::min(xk, xkn));
            while max - min > 1 {
                // div is safe since xkn will never be 0
                xk = xkn.wrapping_add(unsafe { unchecked_div(n, xkn) }) / 2;
                xk = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;
                xk = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;
                xkn = xk.wrapping_add(unsafe { unchecked_div(n, xk) }) / 2;

                max = cmp::max(xk, xkn);
                min = cmp::min(xk, xkn);
            }

            min
        }
    };
}

implement_sqrt!(sqrt128, u128);
implement_sqrt!(sqrt64, u64);
implement_sqrt!(sqrt32, u32);

#[test]
fn test_isqrt() {
    for n in 0..100000 {
        let r = sqrt128(n);
        assert!(r * r <= n);
        assert!((r + 1) * (r + 1) > n);
    }
}
