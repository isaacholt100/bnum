use crate::float::UnsignedFloatExponent;
use crate::{BIntD8, BUintD8};
use crate::doc;
use crate::ExpType;
use super::{Float, FloatExponent};

type Digit = u8;

impl<const W: usize> BUintD8<W> {
    #[inline]
    pub(crate) const fn to_exp_type(&self) -> Option<ExpType> {
        let mut out = 0;
        let mut i = 0;
        if Digit::BITS > ExpType::BITS {
            let small = self.digits[i] as ExpType;
            let trunc = small as Digit;
            if self.digits[i] != trunc {
                return None;
            }
            out = small;
            i = 1;
        } else {
            loop {
                let shift = i << crate::digit::u8::BIT_SHIFT; // TODO: make sure to generalise when using general digits
                if i >= W || shift >= ExpType::BITS as usize {
                    break;
                }
                out |= (self.digits[i] as ExpType) << shift;
                i += 1;
            }
        }

        while i < W {
            if self.digits[i] != 0 {
                return None;
            }
            i += 1;
        }

        Some(out)
    }

    #[inline]
    pub(crate) const fn from_exp_type(int: ExpType) -> Option<Self> {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i << crate::digit::u8::BIT_SHIFT < ExpType::BITS as usize { // TODO: make sure to generalise when using general digits
            let d = (int >> (i << crate::digit::u8::BIT_SHIFT)) as Digit; // TODO: make sure to generalise when using general digits
            if d != 0 {
                if i < W {
                    out.digits[i] = d;
                } else {
                    return None;
                }
            }
            i += 1;
        }
        Some(out)
    }
}

#[doc = doc::rounding::impl_desc!()]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::rounding::floor!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn floor(self) -> Self {
        let (fract, trunc) = self.fract_trunc();
        if self.is_sign_positive() || fract.is_zero() {
            trunc
        } else {
            trunc - Self::ONE
        }
    }

    #[inline]
    pub fn floor2(self) -> Self {
        let mut bits = self.to_bits();
        let e = self.signed_biased_exponent() - Self::EXP_BIAS;

        if e >= MB as FloatExponent {
            return self;
        }
        if !e.is_negative() {
            let m = (BUintD8::MAX >> (Self::BITS - Self::MB)) >> e;
            if (bits & m).is_zero() {
                return self;
            }
            if self.is_sign_negative() {
                bits += m;
            }
            bits &= !m;
        } else if self.is_sign_positive() {
            return Self::ZERO;
        } else if !(bits << 1u8).is_zero() {
            return Self::NEG_ONE;
        }
        Self::from_bits(bits)
    }

    #[doc = doc::rounding::ceil!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn ceil(self) -> Self {
        let (fract, trunc) = self.fract_trunc();
        if self.is_sign_negative() || fract.is_zero() {
            trunc
        } else {
            trunc + Self::ONE
        }
    }

    #[inline]
    pub fn ceil2(self) -> Self {
        let mut u = self.to_bits();
        let e = self.signed_biased_exponent() - Self::EXP_BIAS;

        if e >= MB as FloatExponent {
            return self;
        }
        if !e.is_negative() {
            let m = (BUintD8::MAX >> (Self::BITS - Self::MB)) >> e;
            if (u & m).is_zero() {
                return self;
            }
            if self.is_sign_positive() {
                u += m;
            }
            u &= !m;
        } else if self.is_sign_negative() {
            return Self::NEG_ZERO;
        } else if !(u << 1u8).is_zero() {
            return Self::ONE;
        }
        Self::from_bits(u)
    }

    #[doc = doc::rounding::round!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn round(self) -> Self {
        let (fract, trunc) = self.fract_trunc();
        if fract.abs() < Self::HALF {
            trunc
        } else if trunc.is_sign_negative() {
            trunc - Self::ONE
        } else {
            trunc + Self::ONE
        }
    }

    #[inline]
    pub fn round2(self) -> Self {
        let a = Self::HALF - Self::QUARTER * Self::EPSILON;
        if self.is_sign_positive() {
            (self + a).trunc()
        } else {
            (self - a).trunc()
        }
    }

    #[inline]
    pub fn round_ties_even(self) -> Self {
        use core::cmp::Ordering;

        let (fract, trunc) = self.fract_trunc();
        match fract.abs().total_cmp(&Self::HALF) {
            Ordering::Less => trunc,
            Ordering::Greater => if trunc.is_sign_negative() {
                trunc - Self::ONE
            } else {
                trunc + Self::ONE
            },
            Ordering::Equal => {
                let (_, exponent, mantissa) = trunc.into_signed_parts();
                let mantissa_length = (Self::MB - mantissa.trailing_zeros()) as FloatExponent;
                debug_assert!(exponent >= mantissa_length);
                let is_even = exponent > mantissa_length;
                if is_even {
                    trunc
                } else if trunc.is_sign_negative() {
                    trunc - Self::ONE
                } else {
                    trunc + Self::ONE
                }
            },
        }
    }

    #[inline]
    pub const fn fract_trunc(self) -> (Self, Self) {
        handle_nan!((self, self); self);
        if self.is_infinite() {
            return (Self::NAN, self);
        }
        if self.is_zero() {
            return (Self::ZERO, self);
        }

        let (sign, exponent, mantissa) = self.into_signed_parts();

        if exponent.is_negative() { // exponent is negative, so absolute value must be < 1, so truncate to 0
            return if sign {
                (self, Self::NEG_ZERO)
            } else {
                (self, Self::ZERO)
            };
        }
        // exponent is >= 0, so can take unsigned_abs without changing its value
        
        debug_assert!(!self.is_subnormal()); // exponent >= 0 so number should be normal, so mantissa has implicit leading one

        let abs_exponent = exponent.unsigned_abs();
        if UnsignedFloatExponent::BITS - abs_exponent.leading_zeros() <= ExpType::BITS { // if number of bits needed to store abs_exponent is less than bit width of ExpType, then can cast
            let small_exponent = abs_exponent as ExpType;
            if small_exponent >= Self::MB { // if the exponent exceeds the number of mantissa bits, then the number is an integer so truncation does nothing and fractional part is zero
                (Self::ZERO, self)
            } else {
                let mask = BUintD8::<W>::MAX.shl(Self::MB - small_exponent);
                let trunc_mantissa = mantissa.bitand(mask); // set the last MB - exponent bits of the mantissa to zero - this is the fractional part

                let trunc = Self::from_signed_parts(sign, exponent, trunc_mantissa);
                let fract = if trunc_mantissa.eq(&mantissa) {
                    Self::ZERO
                } else {
                    let unshifted_mantissa = mantissa.bitand(mask.not());
                    let shift = Self::MB + 1 - unshifted_mantissa.bits(); // amount of zeros before the first 1 in the fractional part 0.0...01... 
                    // debug_assert!(shift > 0);
                    let fract_mantissa = unshifted_mantissa.shl(shift);
                    let abs_fract_exponent = (shift - small_exponent) as UnsignedFloatExponent; // absolute value of exponent of fractional part
                    let fract_exponent = -(abs_fract_exponent as FloatExponent);
                    let fract = Self::from_signed_parts(sign, fract_exponent, fract_mantissa);

                    fract
                };
                (fract, trunc)
            }
        } else {
            (Self::ZERO, self)
        }
    }

    #[doc = doc::rounding::trunc!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn trunc(self) -> Self {
        self.fract_trunc().1
    }

    #[doc = doc::rounding::fract!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn fract(self) -> Self {
        self.fract_trunc().0
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::floor(f: ftest)
    }
    test_bignum! {
        function: <ftest>::ceil(f: ftest)
    }
    test_bignum! {
        function: <ftest>::round(f: ftest)
    }
    test_bignum! {
        function: <ftest>::round_ties_even(f: ftest)
    }
    test_bignum! {
        function: <ftest>::trunc(f: ftest)
    }
    test_bignum! {
        function: <ftest>::fract(f: ftest)
    }
}