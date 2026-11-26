use super::{Float, FloatExponent, UnsignedFloatExponent};
use crate::{Exponent, Int, Uint};

impl<const W: usize> Uint<W> {
    #[inline]
    pub(crate) const fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    #[inline(always)]
    pub const fn to_bits(self) -> Uint<W> {
        self.bits
    }

    #[inline(always)]
    pub const fn from_bits(v: Uint<W>) -> Self {
        Self { bits: v }
    }

    #[inline(always)]
    pub const fn as_bits(&self) -> &Uint<W> {
        &self.bits
    }

    #[inline(always)]
    pub const fn as_bits_mut(&mut self) -> &mut Uint<W> {
        &mut self.bits
    }

    #[inline(always)]
    pub(crate) const fn to_signed_bits(self) -> Int<W> {
        self.to_bits().cast_signed()
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    const _ASSERT_1: () = assert!(Self::EXPONENT_BITS <= 127);
    const _ASSERT_2: () = assert!(Self::MIN_EXP.checked_sub(MB as FloatExponent + 1).is_some()); // ensures into_normalised_signed_parts won't panic

    // split into sign, exponent and mantissa
    #[inline]
    pub(crate) const fn into_raw_parts(self) -> (bool, UnsignedFloatExponent, Uint<W>) {
        let sign = self.is_sign_negative();
        let exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let mant = self.bits.bitand(Self::MANTISSA_MASK);

        (sign, exp.cast_to_unsigned_float_exponent(), mant)
    }

    /// construct float from sign, exponent and mantissa
    #[inline]
    pub(crate) const fn from_raw_parts(
        sign: bool,
        exponent: UnsignedFloatExponent,
        mantissa: Uint<W>,
    ) -> Self {
        debug_assert!(mantissa.bit_width() <= Self::MB);
        let mut bits = Uint::cast_from_unsigned_float_exponent(exponent)
            .shl(Self::MB)
            .bitor(mantissa);
        if sign {
            bits.set_bit(Self::BITS - 1, true);
        }
        Self::from_bits(bits)
    }

    /// split into sign, exponent and mantissa and adjust to reflect actual numerical represenation, but without taking exponent bias into account
    #[inline]
    pub(crate) const fn into_biased_parts(self) -> (bool, UnsignedFloatExponent, Uint<W>) {
        let (sign, exp, mant) = self.into_raw_parts();
        if exp == 0 {
            (sign, 1, mant)
        } else {
            (
                sign,
                exp,
                mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK),
            )
        }
    }

    #[inline]
    pub(crate) const fn from_biased_parts(
        sign: bool,
        mut exponent: UnsignedFloatExponent,
        mut mantissa: Uint<W>,
    ) -> Self {
        debug_assert!(exponent != 0); // exponent should not be zero as should be 1 for subnormal numbers
        if mantissa.bit(Self::MB) {
            mantissa = mantissa.bitxor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
        // remove the implicit bit from the mantissa
        } else {
            debug_assert!(exponent == 1); // number is subnormal so exponent should be 1
            exponent = 0;
        }
        Self::from_raw_parts(sign, exponent, mantissa)
    }

    #[inline]
    pub(crate) const fn into_signed_biased_parts(self) -> (bool, FloatExponent, Uint<W>) {
        let (sign, exp, mant) = self.into_biased_parts();
        (sign, exp as _, mant)
    }

    #[inline]
    pub(crate) const fn from_signed_biased_parts(
        sign: bool,
        exponent: FloatExponent,
        mantissa: Uint<W>,
    ) -> Self {
        debug_assert!(!exponent.is_negative());
        let exponent = exponent as UnsignedFloatExponent;
        Self::from_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    pub(crate) const fn into_signed_parts(self) -> (bool, FloatExponent, Uint<W>) {
        let (sign, exp, mant) = self.into_signed_biased_parts();
        (sign, exp - Self::EXP_BIAS, mant)
    }

    #[inline]
    pub(crate) const fn from_signed_parts(
        sign: bool,
        exponent: FloatExponent,
        mantissa: Uint<W>,
    ) -> Self {
        let exponent = exponent + Self::EXP_BIAS;
        Self::from_signed_biased_parts(sign, exponent, mantissa)
    }

    /// mantissa is normalised so that it is always of the form 1.*...*
    #[inline]
    pub(crate) const fn into_normalised_signed_parts(self) -> (bool, FloatExponent, Uint<W>) {
        let (sign, exp, mant) = self.into_signed_parts();
        let shift = Self::MB + 1 - mant.bit_width();
        if shift == 0 {
            (sign, exp, mant)
        } else {
            let normalised_mant = unsafe { mant.unchecked_shl_internal(shift) }; // SAFETY: we can use unchecked variant since shift is <= Self::MB + 1 < number of bits of float
            debug_assert!(normalised_mant.is_zero() || normalised_mant.bit_width() == Self::MB + 1);
            let normalised_exp = exp - (shift as FloatExponent);

            (sign, normalised_exp, normalised_mant)
        }
    }

    // return a sign s, an exponent e and an integer mantissa m such that self = s * m * 2^e, where m is an integer with at most MB + 1 bits. so treat mantissa as an integer, not as a fixed point number.
    #[inline]
    pub(crate) const fn into_integral_signed_parts(self) -> (bool, FloatExponent, Uint<W>) {
        let (sign, exp, mant) = self.into_signed_parts();
        let diff = if mant.is_zero() {
            0
        } else {
            mant.bit_width() as FloatExponent - 1
        };
        (sign, exp - diff, mant)
    }

    #[inline]
    pub(crate) const fn round_exponent_mantissa<const TIES_EVEN: bool>(
        mut exponent: FloatExponent,
        mantissa: Uint<W>,
        shift: Exponent,
    ) -> (FloatExponent, Uint<W>) {
        // we allow current_width to be specified so that we don't have to recompute mantissa.bit_width() if already known
        let mut shifted_mantissa = unsafe { mantissa.unchecked_shr_internal(shift) };
        if !TIES_EVEN {
            return (exponent, shifted_mantissa); // if not TIES_EVEN, then we truncate
        }
        let discarded_shifted_bits = mantissa.bitand(Uint::MAX.shr(Self::BITS - shift));
        if discarded_shifted_bits.bit(shift - 1) {
            // in this case, the discarded portion is at least a half
            if shifted_mantissa.is_odd() || !discarded_shifted_bits.is_power_of_two() {
                // in this case, ties to even says we round up. checking if not a power of two tells us that there is at least one bit set to 1 (after the most significant bit set to 1). we check in this order as is_odd is O(1) whereas is_power_of_two is O(N)
                shifted_mantissa = shifted_mantissa.add(Uint::ONE);
                if shifted_mantissa.bit(shift) {
                    // check for overflow (with respect to the mantissa bit width)
                    exponent += 1;
                    shifted_mantissa = unsafe { shifted_mantissa.unchecked_shr_internal(1) };
                }
            }
        }
        (exponent, shifted_mantissa)
    }

    #[inline]
    pub(crate) const fn from_normalised_signed_parts(
        sign: bool,
        exponent: FloatExponent,
        mantissa: Uint<W>,
    ) -> Self {
        debug_assert!(mantissa.is_zero() || mantissa.bit_width() == Self::MB + 1);

        if exponent < Self::MIN_EXP - 1 {
            let shift = (Self::MIN_EXP - 1 - exponent) as Exponent;
            let (out_exponent, out_mantissa) =
                Self::round_exponent_mantissa::<true>(Self::MIN_EXP - 1, mantissa, shift);

            Self::from_signed_parts(sign, out_exponent, out_mantissa)
        } else {
            Self::from_signed_parts(sign, exponent, mantissa)
        }
    }

    #[inline]
    pub(crate) const fn signed_biased_exponent(self) -> FloatExponent {
        self.into_signed_biased_parts().1
    }

    /// Returns the exponent bias `2^(EXPONENT_BITS - 1) - 1` as a `Uint<W>`.
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2` (including `EXPONENT_BITS >= 32`).
    #[inline]
    pub(crate) const fn exp_bias_uint() -> Uint<W> {
        Uint::<W>::ONE.shl(Self::EXPONENT_BITS - 1).sub(Uint::ONE)
    }

    /// Returns the all-ones raw exponent `2^EXPONENT_BITS - 1` as a `Uint<W>`.
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2` (including `EXPONENT_BITS >= 32`).
    #[inline]
    pub(crate) const fn exp_max_uint() -> Uint<W> {
        Uint::<W>::MAX.shr(Self::MB + 1)
    }

    /// Returns `1.0` without relying on `i32` `EXP_BIAS`.
    #[inline]
    pub(crate) const fn one_val() -> Self {
        Self::from_bits(Self::exp_bias_uint().shl(Self::MB))
    }

    /// Returns `-1.0` without relying on `i32` `EXP_BIAS`.
    #[inline]
    pub(crate) const fn neg_one_val() -> Self {
        Self::one_val().neg()
    }

    /// Returns `2.0` without relying on `i32` `EXP_BIAS`.
    #[inline]
    pub(crate) const fn two_val() -> Self {
        Self::from_bits(Self::exp_bias_uint().add(Uint::ONE).shl(Self::MB))
    }

    /// Returns `0.5` without relying on `i32` `EXP_BIAS` (also handles `EXPONENT_BITS == 2`,
    /// where `0.5` is subnormal).
    #[inline]
    pub(crate) const fn half_val() -> Self {
        if Self::EXPONENT_BITS == 2 {
            Self::from_bits(Uint::<W>::ONE.shl(Self::MB - 1))
        } else {
            Self::from_bits(Self::exp_bias_uint().sub(Uint::ONE).shl(Self::MB))
        }
    }

    /// Splits a finite `Float<W, MB>` into `(sign, biased_exp, significand)` where
    /// `value = (-1)^sign * significand * 2^(biased_exp - bias - MB)`, `biased_exp >= 1`,
    /// and `significand < 2^(MB + 1)`. Works for all `EXPONENT_BITS >= 2`.
    #[inline]
    pub(crate) const fn into_biased_uint_parts(self) -> (bool, Uint<W>, Uint<W>) {
        let sign = self.is_sign_negative();
        let raw_exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let raw_mant = self.bits.bitand(Self::MANTISSA_MASK);
        if raw_exp.is_zero() {
            (sign, Uint::ONE, raw_mant)
        } else {
            (sign, raw_exp, raw_mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK))
        }
    }

    /// Packs `(sign, biased_exp, significand)` into a finite `Float<W, MB>`.
    /// Caller must ensure `biased_exp < Self::exp_max_uint()` and `significand < 2^(MB + 1)`.
    #[inline]
    pub(crate) const fn from_biased_uint_parts(
        sign: bool,
        exp: Uint<W>,
        mut mant: Uint<W>,
    ) -> Self {
        if mant.is_zero() {
            return if sign { Self::NEG_ZERO } else { Self::ZERO };
        }
        let raw_exp = if mant.bit(Self::MB) {
            mant.set_bit(Self::MB, false);
            exp
        } else {
            debug_assert!(exp.eq(&Uint::ONE));
            Uint::ZERO
        };
        let mut bits = raw_exp.shl(Self::MB).bitor(mant);
        if sign {
            bits.set_bit(Self::BITS - 1, true);
        }
        Self::from_bits(bits)
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
    use crate::test::test_bignum;
    use crate::types::{F32, F64};

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest>::to_bits(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::from_bits(a: UTest)
        }

        test_reversible_conversion!(into_raw_parts, from_raw_parts(a, b, c) -> FTest, FTest::to_bits);
        test_reversible_conversion!(into_biased_parts, from_biased_parts(a, b, c) -> FTest, FTest::to_bits);
        test_reversible_conversion!(into_signed_biased_parts, from_signed_biased_parts(a, b, c) -> FTest, FTest::to_bits);
        test_reversible_conversion!(into_signed_parts, from_signed_parts(a, b, c) -> FTest, FTest::to_bits);
        test_reversible_conversion!(into_normalised_signed_parts, from_normalised_signed_parts(a, b, c) -> FTest, FTest::to_bits);
    }
}
