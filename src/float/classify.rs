use super::Float;
use crate::Byte;
use crate::Uint;
use core::num::FpCategory;

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
        self.to_signed_bits().is_negative() // check if sign bit (MSB) is set
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
        let bytes = self.bits.bytes;
        let mut i = 0;
        while i < W - 1 {
            if bytes[i] != 0 {
                return false;
            }
            i += 1;
        }
        let last = bytes[W - 1];
        last.trailing_zeros() >= Byte::BITS - 1
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
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest>::is_sign_positive(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_sign_negative(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_finite(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_infinite(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_nan(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_subnormal(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::is_normal(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::classify(a: FTestBase)
        }

        #[test]
        fn is_zero() {
            let z1 = FTest::ZERO;
            let z2 = FTest::NEG_ZERO;
            assert!(z1.is_zero());
            assert!(z2.is_zero());
            assert!(!FTest::ONE.is_zero());
        }
    }
}
