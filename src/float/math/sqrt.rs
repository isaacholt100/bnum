use crate::{float::{FloatExponent, UnsignedFloatExponent}, BIntD8, BUintD8};
use super::Float;

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub(super) fn sqrt_internal(self) -> Self {
        handle_nan!(self; self);
        if self.is_zero() {
            return self;
        }
        let bits = self.to_bits();
        if bits == Self::INFINITY.to_bits() {
            return Self::INFINITY;
        }
        if self.is_sign_negative() {
            return Self::NAN;
            /*let u = BUintD8::MAX << (Self::MB - 1);
            return Self::from_bits(u);*/
        }

        let tiny = Self::from_bits(BUintD8::from(0b11011u8) << Self::MB); // TODO: may not work for exponents stored with very few bits

        let mut ix = BIntD8::from_bits(bits);
        let mut i: FloatExponent;
        let mut m = (bits >> Self::MB).cast_to_unsigned_float_exponent() as FloatExponent;
        if m == 0 {
            /* subnormal x */
            i = 0;
            while (ix & (BIntD8::ONE << Self::MB)).is_zero() {
                ix <<= 1;
                i = i + 1;
            }
            m -= i - 1;
        }
        m -= Self::EXP_BIAS; /* unbias exponent */
        ix = (ix & BIntD8::from_bits(BUintD8::MAX >> (Self::BITS - Self::MB)))
            | (BIntD8::ONE << Self::MB);
        if m & 1 == 1 {
            /* odd m, double x to make it even */
            ix += ix;
        }
        m >>= 1; /* m = [m/2] */

        /* generate sqrt(x) bit by bit */
        ix += ix;
        let mut q = BIntD8::ZERO;
        let mut s = BIntD8::ZERO;
        let mut r = BUintD8::ONE << (Self::MB + 1); /* r = moving bit from right to left */

        let mut t: BIntD8<W>;
        while !r.is_zero() {
            t = s + BIntD8::from_bits(r);
            if t <= ix {
                s = t + BIntD8::from_bits(r);
                ix -= t;
                q += BIntD8::from_bits(r);
            }
            ix += ix;
            r = r >> 1u8;
        }

        /* use floating add to find out rounding direction */
        let mut z: Self;
        if !ix.is_zero() {
            z = Self::ONE - tiny; /* raise inexact flag */
            if z >= Self::ONE {
                z = Self::ONE + tiny;
                if z > Self::ONE {
                    q += BIntD8::TWO;
                } else {
                    q += q & BIntD8::ONE;
                }
            }
        }

        ix = (q >> 1u8) + BIntD8::from_bits((BUintD8::MAX << (Self::MB + 1 + 2)) >> 2u8);
        ix += (BUintD8::cast_from_unsigned_float_exponent(m as UnsignedFloatExponent) << Self::MB).cast_signed();
        Self::from_bits(ix.to_bits())
    }
}