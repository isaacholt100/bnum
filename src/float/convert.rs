use super::{Float, FloatExponent, UnsignedFloatExponent};
use crate::{BIntD8, BUintD8, ExpType};
use crate::cast::float::ConvertFloatParts;

type Digit = u8;

impl<const W: usize> BUintD8<W> {
    #[inline]
    pub(crate) const fn is_even(&self) -> bool {
        self.digits[0] & 1 == 0
    }

    #[inline]
    pub(crate) const fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    pub const fn to_bits(self) -> BUintD8<W> {
        self.bits
    }

    #[inline(always)]
    pub const fn from_bits(v: BUintD8<W>) -> Self {
        Self { bits: v }
    }

    #[inline(always)]
    pub(crate) const fn from_words(words: [Digit; W]) -> Self {
        Self::from_bits(BUintD8::from_digits(words))
    }

    #[inline(always)]
    pub(crate) const fn words(&self) -> &[Digit; W] {
        &self.bits.digits
    }

    #[inline(always)]
    pub(crate) const fn to_signed_bits(self) -> BIntD8<W> {
        BIntD8::from_bits(self.to_bits())
    }
}

impl<const W: usize, const MB: usize> ConvertFloatParts for Float<W, MB> {
    type Mantissa = BUintD8<W>;
    type SignedExp = FloatExponent;
    type UnsignedExp = UnsignedFloatExponent;

    #[inline]
    fn into_raw_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_raw_parts(self)
    }

    #[inline]
    fn into_biased_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_biased_parts(self)
    }

    #[inline]
    fn into_signed_biased_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_biased_parts(self)
    }

    #[inline]
    fn into_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_parts(self)
    }

    #[inline]
    fn into_normalised_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_normalised_signed_parts(self)
    }

    #[inline]
    fn from_raw_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_raw_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_biased_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_biased_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_signed_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_signed_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn round_exponent_mantissa<const TIES_EVEN: bool>(exponent: Self::SignedExp, mantissa: Self::Mantissa, shift: ExpType) -> (Self::SignedExp, Self::Mantissa) {
        Self::round_exponent_mantissa::<TIES_EVEN>(exponent, mantissa, shift)
    }

    #[inline]
    fn from_normalised_signed_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_normalised_signed_parts(sign, exponent, mantissa)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    const _ASSERT_1: () = assert!(Self::EXPONENT_BITS <= 127);
    const _ASSERT_2: () = assert!(Self::MIN_EXP.checked_sub(MB as FloatExponent + 1).is_some()); // ensures into_normalised_signed_parts won't panic

    // split into sign, exponent and mantissa
    #[inline]
    pub(crate) const fn into_raw_parts(self) -> (bool, UnsignedFloatExponent, BUintD8<W>) {
        let sign = self.is_sign_negative();
        let exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let mant = self.bits.bitand(Self::MANTISSA_MASK);

        (sign, exp.cast_to_unsigned_float_exponent(), mant)
    }

    /// construct float from sign, exponent and mantissa
    #[inline]
    pub(crate) const fn from_raw_parts(sign: bool, exponent: UnsignedFloatExponent, mantissa: BUintD8<W>) -> Self {
        debug_assert!(mantissa.bits() <= Self::MB);
        let mut bits = BUintD8::cast_from_unsigned_float_exponent(exponent).shl(Self::MB).bitor(mantissa);
        if sign {
            bits.digits[W - 1] |= 1 << (Digit::BITS - 1);
        }
        Self::from_bits(bits)
    }

    /// split into sign, exponent and mantissa and adjust to reflect actual numerical represenation, but without taking exponent bias into account
    #[inline]
    pub(crate) const fn into_biased_parts(self) -> (bool, UnsignedFloatExponent, BUintD8<W>) {
        let (sign, exp, mant) = self.into_raw_parts();
        if exp == 0 {
            (sign, 1, mant)
        } else {
            (sign, exp, mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK))
        }
    }

    #[inline]
    pub(crate) const fn from_biased_parts(sign: bool, mut exponent: UnsignedFloatExponent, mut mantissa: BUintD8<W>) -> Self {
        debug_assert!(exponent != 0); // exponent should not be zero as should be 1 for subnormal numbers
        if mantissa.bit(Self::MB) {
            mantissa = mantissa.bitxor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK); // remove the implicit bit from the mantissa
        } else {
            debug_assert!(exponent == 1); // number is subnormal so exponent should be 1
            exponent = 0;
        }
        Self::from_raw_parts(sign, exponent, mantissa)
    }

    #[inline]
    pub(crate) const fn into_signed_biased_parts(self) -> (bool, i128, BUintD8<W>) {
        let (sign, exp, mant) = self.into_biased_parts();
        (sign, exp as i128, mant)
    }

    #[inline]
    pub(crate) const fn from_signed_biased_parts(sign: bool, exponent: FloatExponent, mantissa: BUintD8<W>) -> Self {
        debug_assert!(!exponent.is_negative());
        let exponent = exponent as UnsignedFloatExponent;
        Self::from_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    pub(crate) const fn into_signed_parts(self) -> (bool, FloatExponent, BUintD8<W>) {
        let (sign, exp, mant) = self.into_signed_biased_parts();
        (sign, exp - Self::EXP_BIAS, mant)
    }

    #[inline]
    pub(crate) const fn from_signed_parts(sign: bool, exponent: FloatExponent, mantissa: BUintD8<W>) -> Self {
        let exponent = exponent + Self::EXP_BIAS;
        Self::from_signed_biased_parts(sign, exponent, mantissa)
    }

    /// mantissa is normalised so that it is always of the form 1.*...*
    #[inline]
    pub(crate) const fn into_normalised_signed_parts(self) -> (bool, FloatExponent, BUintD8<W>) {
        let (sign, exp, mant) = self.into_signed_parts();
        let shift = Self::MB + 1 - mant.bits();
        if shift == 0 {
            (sign, exp, mant)
        } else {
            let normalised_mant = unsafe {
                mant.unchecked_shl_internal(shift)
            }; // SAFETY: we can use unchecked variant since shift is <= Self::MB + 1 < number of bits of float
            debug_assert!(normalised_mant.is_zero() || normalised_mant.bits() == Self::MB + 1);
            let normalised_exp = exp - (shift as FloatExponent);
            
            (sign, normalised_exp, normalised_mant)
        }
    }
    
    #[inline]
    pub(crate) const fn round_exponent_mantissa<const TIES_EVEN: bool>(mut exponent: FloatExponent, mantissa: BUintD8<W>, shift: ExpType) -> (FloatExponent, BUintD8<W>) {
        // we allow current_width to be specified so that we don't have to recompute mantissa.bits() if already known
        let mut shifted_mantissa = unsafe {
            mantissa.unchecked_shr_pad_internal::<false>(shift)
        };
        if !TIES_EVEN {
            return (exponent, shifted_mantissa); // if not TIES_EVEN, then we truncate
        }
        let discarded_shifted_bits = mantissa.bitand(BUintD8::MAX.shr(Self::BITS - shift));
        if discarded_shifted_bits.bit(shift - 1) { // in this case, the discarded portion is at least a half
            if shifted_mantissa.is_odd() || !discarded_shifted_bits.is_power_of_two() { // in this case, ties to even says we round up. checking if not a power of two tells us that there is at least one bit set to 1 (after the most significant bit set to 1). we check in this order as is_odd is O(1) whereas is_power_of_two is O(N)
                shifted_mantissa = shifted_mantissa.add(BUintD8::ONE);
                if shifted_mantissa.bit(shift) { // check for overflow (with respect to the mantissa bit width)
                    exponent += 1;
                    shifted_mantissa = unsafe {
                        shifted_mantissa.unchecked_shr_pad_internal::<false>(1)
                    };
                }
            }
        }
        (exponent, shifted_mantissa)
    }

    #[inline]
    pub(crate) const fn from_normalised_signed_parts(sign: bool, exponent: FloatExponent, mantissa: BUintD8<W>) -> Self {
        debug_assert!(mantissa.is_zero() || mantissa.bits() == Self::MB + 1);

        if exponent < Self::MIN_EXP - 1 {
            let shift = (Self::MIN_EXP - 1 - exponent) as ExpType;
            let (out_exponent, out_mantissa) = Self::round_exponent_mantissa::<true>(Self::MIN_EXP - 1, mantissa, shift);
            
            Self::from_signed_parts(sign, out_exponent, out_mantissa)
        } else {
            Self::from_signed_parts(sign, exponent, mantissa)
        }
    }

    #[inline]
    pub(crate) const fn signed_biased_exponent(self) -> FloatExponent {
        self.into_signed_biased_parts().1
    }
}

#[cfg(test)]
macro_rules! test_reversible_conversion {
    ($to: ident, $from: ident ($($param: ident), *) -> $dest_type: ident $(, $prop: path)?) => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_reversible_conversion_ $dest_type:lower _to_ $to _from_ $from >](v: $dest_type) -> quickcheck::TestResult {
                    if !v.is_finite() {
                        return quickcheck::TestResult::discard();
                    }
                    let ($($param), *) = <$dest_type>::$to(v);
                    let c_from = <$dest_type>::$from($($param), *);
                    // assert_eq!($($prop)?(c_from), $($prop)?(v));
                    quickcheck::TestResult::from_bool($($prop)?(c_from) == $($prop)?(v))
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::{F32, F64};
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::to_bits(a: ftest)
    }
    test_bignum! {
        function: <f64>::from_bits(a: u64)
    }
    test_bignum! {
        function: <f32>::from_bits(a: u32)
    }

    test_reversible_conversion!(into_raw_parts, from_raw_parts(a, b, c) -> FTEST, FTEST::to_bits);
    test_reversible_conversion!(into_biased_parts, from_biased_parts(a, b, c) -> FTEST, FTEST::to_bits);
    test_reversible_conversion!(into_signed_biased_parts, from_signed_biased_parts(a, b, c) -> FTEST, FTEST::to_bits);
    test_reversible_conversion!(into_signed_parts, from_signed_parts(a, b, c) -> FTEST, FTEST::to_bits);
    test_reversible_conversion!(into_normalised_signed_parts, from_normalised_signed_parts(a, b, c) -> FTEST, FTEST::to_bits);
}