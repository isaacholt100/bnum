use super::Float;
use crate::{BUint, Bint};
use crate::digit;

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const fn abs(self) -> Self {
        let mut words = *self.words();
        words[W - 1] |= 1 << (digit::BITS - 1);
        Self::from_words(words)
    }
    pub fn sqrt(self) -> Self {
        handle_nan!(self; self);
        if self.is_zero() {
            return self;
        }
        let bits = self.to_bits();
        if bits == Self::INFINITY.to_bits() {
            return Self::INFINITY;
        }
        if self.is_sign_negative() {
            let u = BUint::MAX << (MB - 1);
            return Self::from_bits(u);
        }

        let tiny = Self::from_bits(BUint::from(0b11011u8) << MB);

        //let sign = Bint::<W>::from_bits(BUint::ONE << (Self::BITS - 1));
        let mut ix = Bint::from_bits(bits);

        /* take care of Inf and NaN 
        if (ix.to_bits() & ((BUint::MAX << (MB + 1)) >> 1u8)) == ((BUint::MAX << (MB + 1)) >> 1) {
            return self * self + self; /* sqrt(NaN)=NaN, sqrt(+inf)=+inf, sqrt(-inf)=sNaN */
        }

        /* take care of zero */
        if !ix.is_positive() {
            if (ix & !sign).is_zero() {
                return x; /* sqrt(+-0) = +-0 */
            }
            if ix < 0 {
                return (x - x) / (x - x); /* sqrt(-ve) = sNaN */
            }
        }*/

        /* normalize x */
        let mut i: Bint<W>;
        let mut m = ix >> MB;
        if m.is_zero() {
            /* subnormal x */
            i = Bint::ZERO;
            while (ix & (Bint::ONE << MB)).is_zero() {
                ix <<= 1;
                i = i + Bint::ONE;
            }
            m -= i - Bint::ONE;
        }
        m -= Self::EXP_BIAS; /* unbias exponent */
        ix = (ix & Bint::from_bits(BUint::MAX >> (Self::BITS - MB))) | (Bint::ONE << MB);
        if m & Bint::ONE == Bint::ONE {
            /* odd m, double x to make it even */
            ix += ix;
        }
        m >>= 1; /* m = [m/2] */

        /* generate sqrt(x) bit by bit */
        ix += ix;
        let mut q = Bint::ZERO;
        let mut s = Bint::ZERO;
        let mut r = BUint::ONE << (MB + 1); /* r = moving bit from right to left */

        let mut t: Bint<W>;
        while !r.is_zero() {
            t = s + Bint::from_bits(r);
            if t <= ix {
                s = t + Bint::from_bits(r);
                ix -= t;
                q += Bint::from_bits(r);
            }
            ix += ix;
            r >>= 1u8;
        }

        /* use floating add to find out rounding direction */
        let mut z: Self;
        if !ix.is_zero() {
            z = Self::ONE - tiny; /* raise inexact flag */
            if z >= Self::ONE {
                z = Self::ONE + tiny;
                if z > Self::ONE {
                    q += Bint::TWO;
                } else {
                    q += q & Bint::ONE;
                }
            }
        }

        ix = (q >> 1u8) + Bint::from_bits((BUint::MAX << (MB + 1 + 2)) >> 2u8);
        ix += m << MB;
        Self::from_bits(ix.to_bits())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sqrt() {
        println!("{:064}")
        panic!("{}", crate::F64::EXP_BIAS);
    }
    test_float! {
        function: sqrt(f: f64)
    }
}