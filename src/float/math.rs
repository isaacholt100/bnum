use super::Float;
use crate::cast::As;
use crate::{BIntD8, BUintD8};

/*/// Returns tuple of division and whether u is less than v
pub const fn div_float<const N: usize>(u: BUintD8<N>, v: BUintD8<N>) -> (BUintD8<N>, bool) {
    let gt = if let core::cmp::Ordering::Less = u.cmp(&v) {
        0
    } else {
        1
    };
    // `self` is padded with N trailing zeros (less significant digits).
    // `v` is padded with N leading zeros (more significant digits).
    let shift = v.digits[N - 1].leading_zeros();
    // `shift` is between 0 and 64 inclusive.
    let v = super::unchecked_shl(v, shift);
    // `v` is still padded with N leading zeros.

    struct Remainder<const M: usize> {
        first: Digit,
        second: Digit,
        rest: [Digit; M],
    }
    impl<const M: usize> Remainder<M> {
        const fn new(uint: BUintD8<M>, shift: ExpType) -> Self {
            // This shift can be anything from 0 to 64 inclusive.
            // Scenarios:
            // * shift by 0 -> nothing happens, still N trailing zeros.
            // * shift by 64 -> all digits shift by one to the right, there are now (N - 1) trailing zeros and 1 leading zero.
            // * shift by amount between 0 and 64 -> there may be 0 or 1 leading zeros and (N - 1) or N trailing zeros.
            // So indexing between 2N - 1 and N - 1 will get any non-zero digits.
            // Instead of a logical right shift, we will perform a rotate right on the uint - this is the same except the part of the number which may have been removed from the right shift is instead brought to the most significant digit of the number.
            // Then do fancy bit shifts and logic to separate the first and last non zero digits.
            let shift = Digit::BITS - shift;
            let mut rest = uint.rotate_right(shift);
            let last_digit = rest.digits[M - 1];
            let last = (last_digit << shift) >> shift;
            let second = last_digit ^ last;
            rest.digits[M - 1] = last;
            Self {
                first: 0,
                second,
                rest: rest.digits,
            }
        }
        const fn index(&self, index: usize) -> Digit {
            if index == M - 1 {
                self.first
            } else if index == M {
                self.second
            } else if index > M {
                self.rest[index - M - 1]
            } else {
                // There are M - 1 trailing zeros so we can return zero here.
                0
            }
        }
        const fn set_digit(&mut self, index: usize, digit: Digit) {
            if index == M - 1 {
                self.first = digit;
            } else if index == M {
                self.second = digit;
            } else if index > M {
                self.rest[index - M - 1] = digit;
            }
        }
        /*const fn to_uint(self, shift: ExpType) -> BUintD8<M> {
            let mut out = BUintD8::ZERO;
            let mut i = 0;
            while i < M {
                out.digits[i] = self.index(i) >> shift;
                i += 1;
            }
            if shift > 0 {
                let mut i = 0;
                while i < M {
                    out.digits[i] |= self.rest[i] << (Digit::BITS - shift);
                    i += 1;
                }
            }
            out
        }*/
        const fn sub(&mut self, start: usize, rhs: Mul<M>, end: usize) -> bool {
            let mut carry = false;
            let mut i = 0;
            while i < end {
                let (sum, overflow1) = rhs.index(i).overflowing_add(carry as Digit);
                let (sub, overflow2) = self.index(i + start).overflowing_sub(sum);
                self.set_digit(i + start, sub);
                carry = overflow1 || overflow2;
                i += 1;
            }
            carry
        }
        const fn add(&mut self, start: usize, rhs: [Digit; M], end: usize) -> bool {
            let mut carry = false;
            let mut i = 0;
            while i < end {
                let (sum, overflow1) = rhs[i].overflowing_add(carry as Digit);
                let (sum, overflow2) = self.index(i + start).overflowing_sub(sum);
                self.set_digit(i + start, sum);
                carry = overflow1 || overflow2;
                i += 1;
            }
            carry
        }
    }

    // The whole implementation of `Mul` doesn't need to change as it is already padded with N leading zeros.
    struct Mul<const M: usize> {
        last: Digit,
        rest: [Digit; M],
    }
    impl<const M: usize> Mul<M> {
        const fn new(uint: BUintD8<M>, rhs: Digit) -> Self {
            let mut rest = [0; M];
            let mut carry: Digit = 0;
            let mut i = 0;
            while i < M {
                let (prod, c) = crate::arithmetic::mul_carry_unsigned(carry, 0, uint.digits[i], rhs);
                carry = c;
                rest[i] = prod;
                i += 1;
            }
            Self {
                last: carry,
                rest,
            }
        }
        const fn index(&self, index: usize) -> Digit {
            if index == M {
                self.last
            } else {
                self.rest[index]
            }
        }
    }

    let mut u = Remainder::new(u, shift);
    let mut q = BUintD8::ZERO;
    let v_n_1 = v.digits[N - 1];
    let v_n_2 = v.digits[N - 2];
    let gt_half = v_n_1 > digit::HALF;

    let mut j = N + gt;
    while j > gt {
        j -= 1;
        let u_jn = u.index(j + N);
        let mut q_hat = if u_jn < v_n_1 {
            let (mut q_hat, mut r_hat) = if gt_half {
                BUintD8::<N>::div_wide(u_jn, u.index(j + N - 1), v_n_1)
            } else {
                BUintD8::<N>::div_half(u_jn, u.index(j + N - 1), v_n_1)
            };
            loop {
                let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + N - 2) as DoubleDigit;
                let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                if b <= a {
                    break;
                }
                /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                if hi < r_hat {
                    break;
                } else if hi == r_hat && lo <= u.index(j + n - 2) {
                    break;
                }*/
                q_hat -= 1;
                let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                r_hat = new_r_hat;
                if overflow {
                    break;
                }
            }
            q_hat
        } else {
            Digit::MAX
        };

        let q_hat_v = Mul::new(v, q_hat);
        let carry = u.sub(j, q_hat_v, N + 1);
        if carry {
            q_hat -= 1;
            let carry = u.add(j, v.digits, N);
            u.set_digit(j + N, u.index(j + N).wrapping_add(carry as Digit));
        }
        // if self is less than other, q_hat is 0
        q.digits[j - gt] = q_hat;
    }

    (q, gt == 0)
    //super::unchecked_shl(self.as_buint::<{N * 2}>(), N as u16 * 64).div_rem(v.as_buint::<{N * 2}>()).0
}*/

/*
All functions:
mul_add, div_euclid, rem_euclid, powi, powf, exp, exp2, ln, log, log2, log10, cbrt, hypot, sin, cos, tan, asin, acos, atan, atan2, sin_cos, exp_m1, ln_1p, sinh, cosh, tanh, asinh, acosh, atanh, to_degrees, to_radians
*/

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn abs(self) -> Self {
        if self.is_sign_negative() {
            self.neg()
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
            return Self::NAN;
            /*let u = BUintD8::MAX << (Self::MB - 1);
            return Self::from_bits(u);*/
        }

        let tiny = Self::from_bits(BUintD8::from(0b11011u8) << Self::MB); // TODO: may not work for exponents stored with very few bits

        let mut ix = BIntD8::from_bits(bits);
        let mut i: BIntD8<W>;
        let mut m = ix >> Self::MB;
        if m.is_zero() {
            /* subnormal x */
            i = BIntD8::ZERO;
            while (ix & (BIntD8::ONE << Self::MB)).is_zero() {
                ix <<= 1;
                i = i + BIntD8::ONE;
            }
            m -= i - BIntD8::ONE;
        }
        m -= Self::EXP_BIAS; /* unbias exponent */
        ix = (ix & BIntD8::from_bits(BUintD8::MAX >> (Self::BITS - Self::MB)))
            | (BIntD8::ONE << Self::MB);
        if m & BIntD8::ONE == BIntD8::ONE {
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
        ix += m << Self::MB;
        Self::from_bits(ix.to_bits())
    }

    #[inline]
    pub fn round(self) -> Self {
        let a = Self::HALF - Self::QUARTER * Self::EPSILON;
        (self + a.copysign(self)).trunc()
    }

    #[inline]
    pub fn ceil(self) -> Self {
        let mut u = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if e >= BIntD8::from(MB) {
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

    #[inline]
    pub fn floor(self) -> Self {
        let mut bits = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if e >= BIntD8::from(MB) {
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

    #[inline]
    pub fn trunc(self) -> Self {
        //return self.fract_trunc().1;
        let mut i = self.to_bits();
        let exp_bits = BIntD8::from(Self::BITS - Self::MB);
        let mut e = self.exponent() - Self::EXP_BIAS + exp_bits;

        if e >= BIntD8::from(Self::BITS) {
            return self;
        }
        if e < exp_bits {
            e = BIntD8::ONE;
        }
        let m = BIntD8::NEG_ONE.to_bits() >> e;
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
        if self.is_infinite() {
            return (Self::NAN, self);
        }

        let mut u = self.to_bits();
        let e = self.exponent() - Self::EXP_BIAS;

        if self.is_infinite() {
            return (Self::NAN, self);
        }

        if e >= BIntD8::from(MB) {
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

        let mask = BUintD8::<W>::MAX >> (e + (Self::BITS - Self::MB).as_::<BIntD8<W>>());
        if (u & mask).is_zero() {
            return (Self::ZERO, self);
        }
        u &= !mask;
        let trunc = Self::from_bits(u);
        (self - trunc, trunc)
    }

    #[inline]
    pub fn recip2(self) -> Self
    where
        [(); W * 2]:,
    {
        Self::ONE / self
    }

    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self
    where
        [(); W * 2]:,
    {
        let div = (self / rhs).trunc();
        if self % rhs < Self::ZERO {
            return if rhs > Self::ZERO {
                div - Self::ONE
            } else {
                div + Self::ONE
            };
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

    #[inline]
    pub fn powi(mut self, n: i32) -> Self
    where
        [(); W * 2]:,
    {
        // println!("{:032b}, {}", self.to_bits(), n);
        if n == 0 {
            return self;
        }
        let mut n_abs = n.unsigned_abs(); // unsigned abs since otherwise overflow could occur (if n == i32::MIN)
        let mut y = Self::ONE;
        while n_abs > 1 {
            if n_abs & 1 == 1 {
                // out = out * self;
                y = y * self;
            }
            self = self * self;
            n_abs >>= 1;
        }
        if n.is_negative() {
            Self::ONE / (self * y)
        } else {
            self * y
        }
    }

    /*pub fn remquof(mut self, mut y: Self) -> /*(Self, BIntD8<W>)*/(Self, Self) {
        handle_nan!(self; self);
        handle_nan!(y; y);
        if self.is_infinite() || y.is_infinite() {
            return (Self::NAN, Self::NAN);
        }

        if y.is_zero() {
            return (Self::QNAN, Self::QNAN);
        }
        if self.is_zero() {
            return (self, self);
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
            while !BIntD8::from_bits(i).is_negative() {
                ex -= BIntD8::ONE;
                i <<= 1u8;
            }
            uxi <<= -ex + BIntD8::ONE;
        } else {
            uxi &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uxi |= BUintD8::ONE << Self::MB;
        }
        if ey.is_zero() {
            i = uy << (Self::BITS - Self::MB);
            while !BIntD8::from_bits(i).is_negative() {
                ey -= BIntD8::ONE;
                i <<= 1u8;
            }
            uy <<= -ey + BIntD8::ONE;
        } else {
            uy &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uy |= BUintD8::ONE << Self::MB;
        }

        let mut q = BUintD8::<W>::ZERO;
        if ex + BIntD8::ONE != ey {
            if ex < ey {
                return (self, 0);
            }
            /* x mod y */
            while ex > ey {
                i = uxi.wrapping_sub(uy);
                if !BIntD8::from_bits(i).is_negative() {
                    uxi = i;
                    q += BUintD8::ONE;
                }
                uxi <<= 1u8;
                q <<= 1u8;
                ex -= BIntD8::ONE;
            }
            i = uxi.wrapping_sub(uy);
            if !BIntD8::from_bits(i).is_negative() {
                uxi = i;
                q += BUintD8::ONE;
            }
            if uxi.is_zero() {
                //ex = BIntD8::TWO - BIntD8::from(Self::BITS);
                ex = BIntD8::from(-60i8);
            } else {
                while (uxi >> Self::MB).is_zero() {
                    uxi <<= 1u8;
                    ex -= BIntD8::ONE;
                }
            }
        }

        /* scale result and decide between |x| and |x|-|y| */
        if ex.is_positive() {
            uxi -= BUintD8::ONE << Self::MB;
            uxi |= ex.to_bits() << Self::MB;
        } else {
            uxi >>= -ex + BIntD8::ONE;
        }
        self = Self::from_bits(uxi);
        if sy {
            y = -y;
        }
        if ex == ey || (ex + BIntD8::ONE == ey && (Self::TWO * self > y || (Self::TWO * self == y && !(q % BUintD8::TWO).is_zero()))) {
            self = self - y;
            q += BUintD8::ONE;
        }
        q &= BUintD8::MAX >> 1u8;
        let quo = if sx ^ sy { -BIntD8::from_bits(q) } else { BIntD8::from_bits(q) };
        if sx {
            (-self, quo)
        } else {
            (self, quo)
        }
    }*/

    #[cfg(test)]
    pub(crate) fn to_f64(self) -> f64 {
        f64::from_bits(self.to_bits().as_())
    }

    #[cfg(test)]
    pub(crate) fn to_f32(self) -> f32 {
        f32::from_bits(self.to_bits().as_())
    }
}

#[cfg(test)]
impl super::F32 {
    #[inline]
    pub fn recip(self) -> Self {
        /*let (e, m) = self.exp_mant();
        let normalised = Self::from_exp_mant(self.is_sign_negative(), Self::EXP_BIAS.to_bits() - BUintD8::ONE, m);
        println!("norm: {}", normalised.to_f64());
        let r = normalised.recip_internal();
        let e = self.exponent() - Self::EXP_BIAS;
        //println!("{}", e);
        let e = (-e) + Self::EXP_BIAS;
        Self::from_exp_mant(self.is_sign_negative(), e.to_bits() - BUintD8::ONE, r.exp_mant().1)*/
        self.recip_internal()
    }

    #[inline]
    pub fn recip_internal(self) -> Self {
        if self.is_zero() {
            return Self::NAN;
        }
        // solve 1/b - x = 0 so 1/x - b = 0 =: f(x)
        // x_{n + 1} = x_n - f(x_n) / f'(x_n)
        // = x_n - (1/x_n - b) / (-1/x_n^2)
        // = x_n + (x_n - b x_n^2)
        // = x_n (2 - b x_n)
        let (e, m) = self.exp_mant();
        let e = BIntD8::from_bits(e) - Self::EXP_BIAS;
        let inv_e = (-e + Self::EXP_BIAS).to_bits() - BUintD8::ONE;
        //println!("{}", e);
        let normalised = Self::from_exp_mant(false, Self::EXP_BIAS.to_bits() - BUintD8::ONE, m);
        if normalised == Self::ONE {
            return Self::from_exp_mant(self.is_sign_negative(), inv_e + 1, BUintD8::ZERO);
        }
        //println!("norm init: {:064b}", normalised.to_bits());
        let mut x_n = Self::from_bits(
            (normalised * Self::HALF).to_bits() ^ (BUintD8::MAX >> (Self::BITS - Self::MB)),
        );

        let mut m_n = x_n.exp_mant().1 << 1;

        /*
        0.5 <= x_n < 1
        1 <= normalised < 2
        so 0.5 <= x_n * normalised < 2
        1 <= x_n * 2 < 2
        */

        //println!("x_n: {}", x_n.to_f32());
        let mut iters = 0;
        loop {
            let a1 = x_n * Self::TWO;
            let a2 = x_n * normalised * x_n;
            let x_n_1 = a1 - a2;
            assert!(a1.to_f32() >= 1.0 && a1.to_f32() <= 2.0);
            assert!(a2.to_f32() >= 0.5 && a2.to_f32() <= 1.0);

            let ma1 = m_n << 1;
            let xnf = x_n_1.to_f32();
            assert!(
                0.5 <= xnf && xnf < 1.0,
                "{}, norm: {}",
                xnf,
                normalised.to_f32()
            );
            // x_n * (2 - norm * x_n)
            if x_n_1 == x_n || iters == 100 {
                //println!("done: new: {}, old: {}", x_n_1.to_f64(), x_n.to_f64());
                //println!("norm: {:064b}", x_n_1.to_bits());
                let mut m = x_n_1.exp_mant().1;
                if m.bit(Self::MB) {
                    m ^= BUintD8::ONE << Self::MB;
                }
                let unnormalised = Self::from_exp_mant(self.is_sign_negative(), inv_e, m);
                println!("iters: {}", iters);
                return unnormalised;
            }
            x_n = x_n_1;
            iters += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::abs(f: ftest)
    }

    test_bignum! {
        function: <ftest>::sqrt(f: ftest)
    }

    test_bignum! {
        function: <ftest>::ceil(f: ftest)
    }

    test_bignum! {
        function: <ftest>::floor(f: ftest)
    }

    /*test_bignum! {
        function: <ftest>::round(f: ftest)
    }*/

    test_bignum! {
        function: <ftest>::trunc(f: ftest)
    }

    test_bignum! {
        function: <ftest>::fract(f: ftest)
    }

    test_bignum! {
        function: <ftest>::div_euclid(f1: ftest, f2: ftest)
    }

    test_bignum! {
        function: <ftest>::rem_euclid(f1: ftest, f2: ftest)
    }

    test_bignum! {
        function: <ftest>::powi(f: ftest, n: i32)
    }

    /*#[test]
    fn fmod() {
        use super::super::F64;
        let f1 = 0.0;
        let f2 = f64::INFINITY;
        //println!("{:064b}", ((-0.0f64).div_euclid(f2)).to_bits());
        let a = (F64::from(f1) * (F64::from(f2))).to_bits();
        let b = (f1 * (f2)).to_bits();
        /*println!("{:064b}", a);
        println!("{:064b}", b);*/
        assert!(a == b.into());
    }

    quickcheck::quickcheck! {
        fn qc_recip(u: u32) -> quickcheck::TestResult {
            let f = f32::from_bits(u);
            if !f.is_finite() || f >= 2.0 || f <= 1.0 {
                return quickcheck::TestResult::discard();
            }

            let b1 = f.recip().to_bits();
            let b2 = super::super::F32::from(f).recip().to_f32().to_bits();
            return quickcheck::TestResult::from_bool(b1 == b2 || b1 + 1 == b2);
        }
    }

    #[test]
    fn recip2() {
        assert!((0.0 as ftest).to_bits().count_zeros() == 32);
        use super::super::F32;

        let f1 = 1.7517333f32; //f32::from_bits(0b0_01111110_01001000000000000000000u32);
        println!("{}", f1);

        println!("{:032b}", f1.recip().to_bits());
        println!("{:032b}", F32::from(f1).recip_internal().to_f32().to_bits());
        panic!("")
    }

    test_bignum! {
        function: <ftest>::recip(f: ftest),
        skip: !f.is_finite() || f == 0.0 || f >= 2.0 || f <= 1.0
    }

    #[test]
    fn recip_u8() {
        let mut g = true;
        let mut close = true;
        for i in 1..=u8::MAX {
            let u = 0b0_01111110_00000000000000000000000u32 | ((i as u32) << 15);
            let f = f32::from_bits(u);

            let b1 = f.recip().to_bits();
            let b2 = super::super::F32::from(f)
                .recip_internal()
                .to_f32()
                .to_bits();
            let eq = b1 == b2;
            if !eq {
                println!("{:08b}", i);
                if b2 - b1 != 1 {
                    close = false;
                }
            }
            if b1 > b2 {
                if b1 > b2 {
                    g = false;
                }
            }
        }
        println!("all greater: {}", g);
        println!("all close: {}", close);
        panic!("")
    }*/
}
//0011111111101001100110011001100110011001100111110001100011111001
//0011111111100000000000000000000000000000000000110110111110011100

//0011111111110011111111111111111111111111111110111011010001111101
// 0011111111111111111111111111111111111111111110010010000011001000
