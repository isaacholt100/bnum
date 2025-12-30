use crate::Byte;
#[cfg(test)]
use crate::cast::As;
use crate::doc;
use crate::{Exponent, Int, Uint};

#[cfg(test)]
use crate::types::{F32, F64};

#[cfg(test)]
impl From<f64> for F64 {
    #[inline]
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().as_())
    }
}

#[cfg(test)]
impl From<f32> for F32 {
    #[inline]
    fn from(f: f32) -> Self {
        Self::from_bits(f.to_bits().as_())
    }
}

macro_rules! handle_nan {
    ($ret: expr; $($n: expr), +) => {
        if $($n.is_nan()) || + {
            return $ret;
        }
    };
}

mod bytes;
mod cast;
mod classify;
mod cmp;
mod const_trait_fillers;
mod consts;
mod convert;
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

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Float<const W: usize, const MB: usize> {
    bits: Uint<W>,
}

pub(crate) type FloatExponent = i32; // TODO: decide whether this should be i128 or i32 (or i64). benefit of i128: more exponents possible. benefit of i32: this is what f32, f64 use, aligns better with u32 exponents used for ints
pub(crate) type UnsignedFloatExponent = u32; // TODO: change these to just Exponent and SignedExponent

// TODO: implement rand traits

impl<const W: usize, const MB: usize> Float<W, MB> {
    const MB: Exponent = MB as _;
    const BITS: Exponent = Uint::<W>::BITS;

    const EXPONENT_BITS: Exponent = Self::BITS - Self::MB - 1;

    const MANTISSA_MASK: Uint<W> = Uint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);

    const SIGN_MASK: Uint<W> = Int::MAX.cast_unsigned();

    const MANTISSA_IMPLICIT_LEADING_ONE_MASK: Uint<W> = Uint::ONE.shl(Self::MB);
}

impl<const W: usize> Uint<W> {
    #[inline]
    pub(crate) const fn cast_from_unsigned_float_exponent(mut exp: UnsignedFloatExponent) -> Self {
        let mut out = Self::MIN;
        let mut i = 0;
        while exp != 0 && i < W {
            let masked = exp as Byte & Byte::MAX;
            out.bytes[i] = masked;
            if UnsignedFloatExponent::BITS <= Byte::BITS {
                exp = 0;
            } else {
                exp = exp.wrapping_shr(Byte::BITS);
            }
            i += 1;
        }
        out
    }

    #[inline]
    pub(crate) const fn cast_to_unsigned_float_exponent(self) -> UnsignedFloatExponent {
        let mut out = 0;
        let mut i = 0;
        while i * (Byte::BITS as usize) < UnsignedFloatExponent::BITS as usize && i < W {
            out |= (self.bytes[i] as UnsignedFloatExponent) << (i * (Byte::BITS as usize));
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
        let exponent_bits =
            Uint::cast_from_unsigned_float_exponent(biased_exponent as UnsignedFloatExponent);
        let float_bits = exponent_bits.shl(Self::MB);
        Self::from_bits(float_bits)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }

    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn copysign(mut self, sign: Self) -> Self {
        self.as_bits_mut()
            .set_bit(Self::BITS - 1, sign.is_sign_negative());
        self
    }

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
                    Self::from_bits(self.to_bits().sub(Uint::ONE))
                } else {
                    Self::from_bits(self.to_bits().add(Uint::ONE))
                }
            }
        }
    }

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
                    Self::from_bits(self.to_bits().add(Uint::ONE))
                } else {
                    Self::from_bits(self.to_bits().sub(Uint::ONE))
                }
            }
        }
    }
}

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    #[doc = doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<const W: usize, const MB: usize> quickcheck::Arbitrary for crate::Float<W, MB> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::from_bits(Uint::arbitrary(g))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing floats;

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
}
