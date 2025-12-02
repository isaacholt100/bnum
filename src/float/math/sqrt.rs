use super::Float;
use crate::cast::CastFrom;
use crate::{
    Int, Uint,
    float::{FloatExponent, UnsignedFloatExponent},
};

impl<const W: usize, const MB: usize> Float<W, MB> {
    // TODO: could also use the sqrt algorithm on Uint with twice as many bits, compare performance with this implementation from libm
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
            /*let u = Uint::MAX << (Self::MB - 1);
            return Self::from_bits(u);*/
        }

        let tiny = Self::from_bits(Uint::cast_from(0b11011u8) << Self::MB); // TODO: may not work for exponents stored with very few bits

        let mut ix = bits.cast_signed();
        let mut i: FloatExponent;
        let mut m = (bits >> Self::MB).cast_to_unsigned_float_exponent() as FloatExponent;
        if m == 0 {
            /* subnormal x */
            i = 0;
            while (ix & (Int::ONE << Self::MB)).is_zero() {
                ix <<= 1;
                i = i + 1;
            }
            m -= i - 1;
        }
        m -= Self::EXP_BIAS; /* unbias exponent */
        ix = (ix & (Uint::MAX >> (Self::BITS - Self::MB)).cast_signed()) | (Int::ONE << Self::MB);
        if m & 1 == 1 {
            /* odd m, double x to make it even */
            ix += ix;
        }
        m >>= 1; /* m = [m/2] */

        /* generate sqrt(x) bit by bit */
        ix += ix;
        let mut q = Int::ZERO;
        let mut s = Int::ZERO;
        let mut r = Uint::ONE << (Self::MB + 1); /* r = moving bit from right to left */

        let mut t: Int<W>;
        while !r.is_zero() {
            t = s + r.cast_signed();
            if t <= ix {
                s = t + r.cast_signed();
                ix -= t;
                q += r.cast_signed();
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
                    q += crate::n!(0b10);
                } else {
                    q += q & crate::n!(0b1); // kenobi
                }
            }
        }

        ix = (q >> 1u8) + ((Uint::MAX << (Self::MB + 1 + 2)) >> 2u8).cast_signed();
        ix += (Uint::cast_from_unsigned_float_exponent(m as UnsignedFloatExponent) << Self::MB)
            .cast_signed();
        Self::from_bits(ix.cast_unsigned())
    }
}
