use super::Float;
use crate::{BUint, Bint};

/*
All functions:
mul_add, div_euclid, rem_euclid, powi, powf, exp, exp2, ln, log, log2, log10, cbrt, hypot, sin, cos, tan, asin, acos, atan, atan2, sin_cos, exp_m1, ln_1p, sinh, cosh, tanh, asinh, acosh, atanh, to_degrees, to_radians
*/

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub fn scalbn(mut self, mut n: Bint<W>) -> Self where [(); W * 2]:, {
        let x1p127 = Self::from_bits(BUint::MAX >> 1u8 << (Self::MB + 2)); // 0x1p127f === 2 ^ 127
        let x1p24 = Self::from_exp_mant(false, BUint::from(MB), BUint::ZERO); // 0x1p24f === 2 ^ 24

        if n > Self::EXP_BIAS {
            self = self * x1p127;
            n -= Self::EXP_BIAS;
            if n > Self::EXP_BIAS {
                self = self * x1p127;
                n -= Self::EXP_BIAS;
                if n > Self::EXP_BIAS {
                    n = Self::EXP_BIAS;
                }
            }
        } else if n < Self::MIN_EXP {
            self = self * (Self::MIN_POSITIVE * x1p24);
            n += Self::EXP_BIAS - Bint::from(MB) - Bint::TWO;
            if n < Self::MIN_EXP {
                self = self * (Self::MIN_POSITIVE * x1p24);
                n += Self::EXP_BIAS - Bint::from(MB) - Bint::TWO;
                if n < Self::MIN_EXP {
                    n = Self::MIN_EXP;
                }
            }
        }
        self * Self::from_bits(((Self::EXP_BIAS + n).to_bits()) << Self::MB)
    }

    #[inline]
    pub const fn abs(self) -> Self {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
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
            let u = BUint::MAX << (Self::MB - 1);
            return Self::from_bits(u);
        }

        let tiny = Self::from_bits(BUint::from(0b11011u8) << Self::MB); // TODO: may not work for exponents stored with very few bits
        
        let mut ix = Bint::from_bits(bits);
        let mut i: Bint<W>;
        let mut m = ix >> Self::MB;
        if m.is_zero() {
            /* subnormal x */
            i = Bint::ZERO;
            while (ix & (Bint::ONE << Self::MB)).is_zero() {
                ix <<= 1;
                i = i + Bint::ONE;
            }
            m -= i - Bint::ONE;
        }
        m -= Self::EXP_BIAS; /* unbias exponent */
        ix = (ix & Bint::from_bits(BUint::MAX >> (Self::BITS - Self::MB))) | (Bint::ONE << Self::MB);
        if m & Bint::ONE == Bint::ONE {
            /* odd m, double x to make it even */
            ix += ix;
        }
        m >>= 1; /* m = [m/2] */

        /* generate sqrt(x) bit by bit */
        ix += ix;
        let mut q = Bint::ZERO;
        let mut s = Bint::ZERO;
        let mut r = BUint::ONE << (Self::MB + 1); /* r = moving bit from right to left */

        let mut t: Bint<W>;
        while !r.is_zero() {
            t = s + Bint::from_bits(r);
            if t <= ix {
                s = t + Bint::from_bits(r);
                ix -= t;
                q += Bint::from_bits(r);
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
                    q += Bint::TWO;
                } else {
                    q += q & Bint::ONE;
                }
            }
        }

        ix = (q >> 1u8) + Bint::from_bits((BUint::MAX << (Self::MB + 1 + 2)) >> 2u8);
        ix += m << Self::MB;
        Self::from_bits(ix.to_bits())
    }

    #[inline]
    pub fn round(self) -> Self where [(); W * 2]:, {
        let a = Self::HALF - Self::QUARTER * Self::EPSILON; // TODO: can precalculate quarter * eps so no need for where bound
        (self + a.copysign(self)).trunc()
    }

    #[inline]
    pub fn ceil(self) -> Self {
        let mut u = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if e >= Bint::from(MB) {
            return self;
        }
        if !e.is_negative() {
            let m = (BUint::MAX >> (Self::BITS - Self::MB)) >> e;
            if (u & m).is_zero() {
                return self;
            }
            if self.is_sign_positive() {
                u += m;
            }
            u &= !m;
        } else {
            if self.is_sign_negative() {
                return Self::NEG_ZERO;
            } else if !(u << 1u8).is_zero() {
                return Self::ONE;
            }
        }
        Self::from_bits(u)
    }

    #[inline]
    pub fn floor(self) -> Self {
        let mut bits = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if e >= Bint::from(MB) {
            return self;
        }
        if !e.is_negative() {
            let m = (BUint::MAX >> (Self::BITS - Self::MB)) >> e;
            if (bits & m).is_zero() {
                return self;
            }
            if self.is_sign_negative() {
                bits += m;
            }
            bits &= !m;
        } else {
            if self.is_sign_positive() {
                return Self::ZERO;
            } else if !(bits << 1u8).is_zero() {
                return Self::NEG_ONE;
            }
        }
        Self::from_bits(bits)
    }

    #[inline]
    pub fn trunc(self) -> Self {
        //return self.fract_trunc().1;
        let mut i = self.to_bits();
        let exp_bits = Bint::from(Self::BITS - Self::MB);
        let mut e = self.exponent() - Self::EXP_BIAS + exp_bits;

        if e >= Bint::from(Self::BITS) {
            return self;
        }
        if e < exp_bits {
            e = Bint::ONE;
        }
        let m = Bint::NEG_ONE.to_bits() >> e;
        if (i & m).is_zero() {
            return self;
        }
        i &= !m;
        Self::from_bits(i)
    }

    #[inline]
    pub fn fract(self) -> Self {
        self.fract_trunc().0
    }

    #[inline]
    pub fn fract_trunc(self) -> (Self, Self) {
        handle_nan!((self, self); self);

        let mut u = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if self.is_infinite() {
            return (Self::NEG_NAN, self);
        }

        if e >= Bint::from(MB) {
            return (Self::ZERO, self);
        }
        if e.is_negative() {
            let trunc = if self.is_sign_negative() {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
            if self.is_zero() {
                return (Self::ZERO, self);
            }
            return (self, trunc);
        }

        let mask = BUint::<W>::MAX >> (e + Bint::from(Self::BITS - Self::MB));
        if (u & mask).is_zero() {
            return (Self::ZERO, self);
        }
        u &= !mask;
        let trunc = Self::from_bits(u);
        (self - trunc, trunc)
    }

    #[inline]
    pub fn recip(self) -> Self where [(); W * 2]:, {
        Self::ONE / self
    }

    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self where [(); W * 2]:, {
        let div = (self / rhs).trunc();
        if self % rhs < Self::ZERO {
            return if rhs > Self::ZERO {
                div - Self::ONE
            } else {
                div + Self::ONE
            }
        }
        div
    }

    #[inline]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let rem = self % rhs;
        if rem < Self::NEG_ZERO {
            rem + rhs.abs()
        } else {
            rem
        }
    }

    /*pub fn remquof(mut self, mut y: Self) -> /*(Self, Bint<W>)*/Self where [(); {(W * 2).saturating_sub(W)
    }]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
        handle_nan!(self; self);
        handle_nan!(y; y);
        if self.is_infinite() || y.is_infinite() {
            return Self::NAN;
        }

        if y.is_zero() {
            return Self::QNAN;
        }
        if self.is_zero() {
            return self;
        }
        let ux = self.to_bits();
        let mut uy = y.to_bits();
        let mut ex = self.exponent();
        let mut ey = y.exponent();
        let sx = self.is_sign_negative();
        let sy = y.is_sign_negative();
        let mut uxi = ux;
    
        /* normalize x and y */
        let mut i;
        if ex.is_zero() {
            i = uxi << (Self::BITS - Self::MB);
            while !Bint::from_bits(i).is_negative() {
                ex -= Bint::ONE;
                i <<= 1u8;
            }
            uxi <<= -ex + Bint::ONE;
        } else {
            uxi &= BUint::MAX >> (Self::BITS - Self::MB);
            uxi |= BUint::ONE << Self::MB;
        }
        if ey.is_zero() {
            i = uy << (Self::BITS - Self::MB);
            while !Bint::from_bits(i).is_negative() {
                ey -= Bint::ONE;
                i <<= 1u8;
            }
            uy <<= -ey + Bint::ONE;
        } else {
            uy &= BUint::MAX >> (Self::BITS - Self::MB);
            uy |= BUint::ONE << Self::MB;
        }
    
        let mut q = BUint::<W>::ZERO;
        if ex + Bint::ONE != ey {
            if ex < ey {
                return /*(self, 0);*/self;
            }
            /* x mod y */
            while ex > ey {
                i = uxi.wrapping_sub(uy);
                if !Bint::from_bits(i).is_negative() {
                    uxi = i;
                    q += BUint::ONE;
                }
                uxi <<= 1u8;
                q <<= 1u8;
                ex -= Bint::ONE;
            }
            i = uxi.wrapping_sub(uy);
            if !Bint::from_bits(i).is_negative() {
                uxi = i;
                q += BUint::ONE;
            }
            if uxi.is_zero() {
                //ex = Bint::TWO - Bint::from(Self::BITS);
                ex = Bint::from(-60i8);
            } else {
                while (uxi >> Self::MB).is_zero() {
                    uxi <<= 1u8;
                    ex -= Bint::ONE;
                }
            }
        }
    
        /* scale result and decide between |x| and |x|-|y| */
        if ex.is_positive() {
            uxi -= BUint::ONE << Self::MB;
            uxi |= ex.to_bits() << Self::MB;
        } else {
            uxi >>= -ex + Bint::ONE;
        }
        self = Self::from_bits(uxi);
        if sy {
            y = -y;
        }
        if ex == ey || (ex + Bint::ONE == ey && (Self::TWO * self > y || (Self::TWO * self == y && !(q % BUint::TWO).is_zero()))) {
            self = self - y;
            q += BUint::ONE;
        }
        q &= BUint::MAX >> 1u8;
        let quo = if sx ^ sy { -Bint::from_bits(q) } else { Bint::from_bits(q) };
        if sx {
            //(-self, quo)
            -self
        } else {
            //(self, quo)
            self
        }
    }*/
}

#[cfg(test)]
mod tests {
    test_float! {
        function: abs(f: f64)
    }

    test_float! {
        function: sqrt(f: f64)
    }

    test_float! {
        function: ceil(f: f64)
    }

    test_float! {
        function: floor(f: f64)
    }

    test_float! {
        function: round(f: f64)
    }

    test_float! {
        function: trunc(f: f64)
    }

    test_float! {
        function: fract(f: f64)
    }

    test_float! {
        function: div_euclid(f1: f64, f2: f64)
    }

    test_float! {
        function: rem_euclid(f1: f64, f2: f64)
    }

    #[test]
    fn fmod() {
        let f1 = 0.0;
        let f2 = f64::INFINITY;
        //println!("{:064b}", ((-0.0f64).div_euclid(f2)).to_bits());
        let a = (crate::F64::from(f1) * (crate::F64::from(f2))).to_bits();
        let b = (f1 * (f2)).to_bits();
        /*println!("{:064b}", a);
        println!("{:064b}", b);*/
        assert!(a == b.into());
    }
}