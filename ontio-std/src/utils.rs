macro_rules! implement_sqrt {
    ($name:ident, $ty:ty) => {
        /// calcuate sqrt of integer `n` using Newton method, return value `v`satisfy `v^2 <= n < (v+1)^2`.
        #[deprecated(since = "0.6.0", note = "use `isqrt` from stdlib instead")]
        pub fn $name(n: $ty) -> $ty {
            n.isqrt()
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
