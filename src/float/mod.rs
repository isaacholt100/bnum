use crate::{BUintD8, BIntD8, ExpType};
use crate::doc;

type Digit = u8;

#[cfg(test)]
pub type F64 = Float<8, 52>;

#[cfg(test)]
pub type F32 = Float<4, 23>;

#[cfg(test)]
impl From<f64> for F64 {
    #[inline]
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

#[cfg(test)]
impl From<f32> for F32 {
    #[inline]
    fn from(f: f32) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

macro_rules! handle_nan {
    ($ret: expr; $($n: expr), +) => {
        if $($n.is_nan()) || + {
            return $ret;
        }
    };
}

mod cast;
mod classify;
mod cmp;
mod consts;
mod const_trait_fillers;
mod convert;
mod endian;
mod math;
#[cfg(feature = "numtraits")]
mod numtraits;
mod ops;
// mod parse;
#[cfg(feature = "rand")]
mod random;
mod rounding;
mod to_str;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// TODO: THINK ABOUT MAKING FLOAT EXPONENT AT MOST ~128 BITS, THEN COULD USE I128 FOR EXPONENT CALCULATIONS, WOULD BE MUCH FASTER AND USE LESS SPACE

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Float<const W: usize, const MB: usize> {
    bits: BUintD8<W>,
}

pub(crate) type FloatExponent = i128;
pub(crate) type UnsignedFloatExponent = u128;

// TODO: implement rand traits

impl<const W: usize, const MB: usize> Float<W, MB> {
    const MB: ExpType = MB as _;
    const BITS: ExpType = BUintD8::<W>::BITS;

    const EXPONENT_BITS: ExpType = Self::BITS - Self::MB - 1;

    const MANTISSA_MASK: BUintD8<W> = BUintD8::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);

    const SIGN_MASK: BUintD8<W> = BIntD8::MAX.to_bits();

    const MANTISSA_IMPLICIT_LEADING_ONE_MASK: BUintD8<W> = BUintD8::ONE.shl(Self::MB);
}

impl<const W: usize> BUintD8<W> {
    #[inline]
    pub(crate) const fn cast_from_unsigned_float_exponent(mut exp: UnsignedFloatExponent) -> Self {
        let mut out = Self::MIN;
        let mut i = 0;
        while exp != 0 && i < W {
            let masked = exp as Digit & Digit::MAX;
            out.digits[i] = masked;
            if UnsignedFloatExponent::BITS <= Digit::BITS {
                exp = 0;
            } else {
                exp = exp.wrapping_shr(Digit::BITS);
            }
            i += 1;
        }
        out
    }

    #[inline]
    pub(crate) const fn cast_to_unsigned_float_exponent(self) -> UnsignedFloatExponent {
        let mut out = 0;
        let mut i = 0;
        while i << crate::digit::BIT_SHIFT < UnsignedFloatExponent::BITS as usize && i < W {
            out |= (self.digits[i] as UnsignedFloatExponent) << (i << crate::digit::BIT_SHIFT);
            i += 1;
        }
        out
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    const MIN_EXP_MINUS_ONE: FloatExponent = Self::MIN_EXP - 1;
    // TODO: write test for this
    /// generate powers of two that are not subnormal
    const fn normal_power_of_two(exponent: FloatExponent) -> Self {
        debug_assert!(exponent >= Self::MIN_EXP_MINUS_ONE);
        debug_assert!(exponent < Self::MAX_EXP);
        let biased_exponent = exponent + Self::EXP_BIAS;
        let exponent_bits = BUintD8::cast_from_unsigned_float_exponent(biased_exponent as UnsignedFloatExponent);
        let float_bits = exponent_bits.shl(Self::MB);
        Self::from_bits(float_bits)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::signum!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }

    #[doc = doc::copysign!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn copysign(self, sign: Self) -> Self {
        let mut self_words = *self.words();
        if sign.is_sign_negative() {
            self_words[W - 1] |= 1 << (Digit::BITS - 1);
        } else {
            self_words[W - 1] &= (!0) >> 1;
        }
        Self::from_bits(BUintD8::from_digits(self_words))
    }

    #[doc = doc::next_up!(F)]
    #[inline]
    pub const fn next_up(self) -> Self {
        use core::num::FpCategory;

        match self.classify() {
            FpCategory::Nan => self,
            FpCategory::Infinite => {
                if self.is_sign_negative() {
                    Self::MIN
                } else {
                    self
                }
            }
            FpCategory::Zero => Self::MIN_POSITIVE_SUBNORMAL,
            _ => {
                if self.is_sign_negative() {
                    Self::from_bits(self.to_bits().sub(BUintD8::ONE))
                } else {
                    Self::from_bits(self.to_bits().add(BUintD8::ONE))
                }
            }
        }
    }

    #[doc = doc::next_down!(F)]
    #[inline]
    pub const fn next_down(self) -> Self {
        use core::num::FpCategory;

        match self.classify() {
            FpCategory::Nan => self,
            FpCategory::Infinite => {
                if self.is_sign_negative() {
                    self
                } else {
                    Self::MAX
                }
            }
            FpCategory::Zero => Self::MAX_NEGATIVE_SUBNORMAL,
            _ => {
                if self.is_sign_negative() {
                    Self::from_bits(self.to_bits().add(BUintD8::ONE))
                } else {
                    Self::from_bits(self.to_bits().sub(BUintD8::ONE))
                }
            }
        }
    }
}

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<const W: usize, const MB: usize> quickcheck::Arbitrary for crate::Float<W, MB> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::from_bits(BUintD8::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::copysign(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest>::signum(a: ftest)
    }
    test_bignum! {
        function: <ftest>::next_up(a: ftest)
    }
    test_bignum! {
        function: <ftest>::next_down(a: ftest)
    }
}