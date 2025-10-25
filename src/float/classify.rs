use super::Float;
use crate::Uint;
use crate::doc;
use core::num::FpCategory;

type Digit = u8;

struct Masks<const W: usize, const MB: usize>;

impl<const W: usize, const MB: usize> Masks<W, MB> {
    const FINITE_MASK: Uint<W> = Float::<W, MB>::INFINITY.to_bits();
}

/// Classification methods: used to determine mathematical properties of the number.
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    pub const fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }

    #[inline(always)]
    pub const fn is_sign_negative(self) -> bool {
        const A: Digit = (1 as Digit).rotate_right(1);
        self.bits.digits[W - 1] >= A
    }

    #[inline]
    pub const fn is_finite(self) -> bool {
        self.to_bits()
            .bitand(Masks::<W, MB>::FINITE_MASK)
            .ne(&Masks::<W, MB>::FINITE_MASK)
    }

    #[inline]
    pub const fn is_infinite(self) -> bool {
        self.abs().to_bits().eq(&Masks::<W, MB>::FINITE_MASK)
    }

    #[inline]
    pub const fn is_nan(self) -> bool {
        !self.is_finite() && self.to_bits().trailing_zeros() < Self::MB
    }

    #[inline]
    pub const fn is_subnormal(self) -> bool {
        let lz = self.abs().to_bits().leading_zeros();
        lz < Self::BITS && lz > Self::EXPONENT_BITS
    }

    #[inline]
    pub const fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        let bytes = self.bits.digits;
        let mut i = 0;
        while i < W - 1 {
            if bytes[i] != 0 {
                return false;
            }
            i += 1;
        }
        let last = bytes[W - 1];
        last.trailing_zeros() >= Digit::BITS - 1
    }

    #[inline]
    pub const fn classify(self) -> FpCategory {
        let u = self.abs().to_bits();
        if u.is_zero() {
            FpCategory::Zero
        } else if u.eq(&Self::INFINITY.to_bits()) {
            FpCategory::Infinite
        } else {
            let u = u.bitand(Masks::<W, MB>::FINITE_MASK);
            if u.is_zero() {
                FpCategory::Subnormal
            } else if u.eq(&Masks::<W, MB>::FINITE_MASK) {
                FpCategory::Nan
            } else {
                FpCategory::Normal
            }
        }
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <ftest>::is_sign_positive(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_sign_negative(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_finite(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_infinite(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_nan(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_subnormal(a: ftest)
    }
    test_bignum! {
        function: <ftest>::is_normal(a: ftest)
    }
    test_bignum! {
        function: <ftest>::classify(a: ftest)
    }

    #[test]
    fn is_zero() {
        let z1 = FTEST::ZERO;
        let z2 = FTEST::NEG_ZERO;
        assert!(z1.is_zero());
        assert!(z2.is_zero());
        assert!(!FTEST::ONE.is_zero());
    }
}
