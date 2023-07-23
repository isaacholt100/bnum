use super::Float;
use crate::cast::As;
use crate::{BIntD8, BUintD8, ExpType};
use core::iter::{Iterator, Product, Sum};
use core::num::FpCategory;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

type Digit = u8;

// TODO: fix this function
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn add_internal(mut self, mut rhs: Self, negative: bool) -> Self {
        //println!("{:?} {:?}", self, rhs);
        //debug_assert_eq!(self.is_sign_negative(), rhs.is_sign_negative());
        if rhs.abs() > self.abs() {
            // If b has a larger exponent than a, swap a and b so that a has the larger exponent
            core::mem::swap(&mut self, &mut rhs);
        }
        let (_, mut a_exp, mut a_mant) = self.to_parts_biased();
        let (_, b_exp, mut b_mant) = rhs.to_parts_biased();

        let exp_diff = a_exp - b_exp;

        let sticky_bit = BUintD8::from(b_mant.trailing_zeros() + 1) < exp_diff;

        // Append extra bits to the mantissas to ensure correct rounding
        a_mant <<= 2 as ExpType;
        b_mant <<= 2 as ExpType;

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUintD8::ZERO),
            None => BUintD8::ZERO,
        };

        if sticky_bit {
            b_mant |= BUintD8::ONE; // round up
        }

        let mut mant = a_mant + b_mant;

        let overflow = !(mant >> (MB + 3)).is_zero();
        if !overflow {
            if mant & BUintD8::from_digit(0b11) == BUintD8::from_digit(0b11)
                || mant & BUintD8::from_digit(0b110) == BUintD8::from_digit(0b110)
            {
                mant += BUintD8::FOUR;
                if !(mant >> (MB + 3)).is_zero() {
                    mant >>= 1 as ExpType;
                    a_exp += BUintD8::ONE;
                }
            }
        } else {
            match (mant & BUintD8::from_digit(0b111)).digits()[0] {
                0b111 | 0b110 | 0b101 => {
                    mant += BUintD8::EIGHT;
                }
                0b100 => {
                    if mant & BUintD8::from_digit(0b1000) == BUintD8::from_digit(0b1000) {
                        mant += BUintD8::EIGHT; // 0b1000
                    }
                }
                _ => {}
            }

            mant >>= 1 as ExpType;
            a_exp += BUintD8::ONE;
        }
        if a_exp > Self::MAX_UNBIASED_EXP {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        mant >>= 2 as ExpType;

        if (mant >> Self::MB).is_zero() {
            a_exp = BUintD8::ZERO;
        } else {
            mant ^= BUintD8::ONE << Self::MB;
        }
        let a = Self::from_raw_parts(negative, a_exp, mant);
        a
    }
}

impl<const W: usize, const MB: usize> Add for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let self_negative = self.is_sign_negative();
        let rhs_negative = rhs.is_sign_negative();
        let a = match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                if self_negative ^ rhs_negative {
                    Self::NAN
                } else {
                    self
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs,
            (FpCategory::Zero, FpCategory::Zero) => {
                if self_negative && rhs_negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => {
                if self_negative ^ rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    let r = self.add_internal(rhs, self_negative);
                    r
                }
            }
        };
        //assert_eq!(a.to_bits(), (self.to_f64() + rhs.to_f64()).to_bits().into());
        a
    }
}

//crate::errors::op_ref_impl!(Add<Float<N, MB>> for Float<N, MB>, add);

impl<const W: usize, const MB: usize> Sum for Float<W, MB> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const W: usize, const MB: usize> Sum<&'a Self> for Float<W, MB> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a + *b)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn sub_internal(mut self, mut rhs: Self, mut negative: bool) -> Self {
        if rhs.abs() > self.abs() {
            // If b has a larger exponent than a, swap a and b so that a has the larger exponent
            negative = !negative;
            core::mem::swap(&mut self, &mut rhs);
        }
        if self.abs() == rhs.abs() {
            return Self::ZERO;
        }

        let (_, a_exp, mut a_mant) = self.to_parts_biased();
        let (_, b_exp, mut b_mant) = rhs.to_parts_biased();
        let exp_diff = a_exp - b_exp;

        let mut a_exp = BIntD8::from_bits(a_exp);

        let sticky_bit2 = !exp_diff.is_zero()
            && exp_diff < BUintD8::<W>::BITS.into()
            && b_mant.bit(exp_diff.as_::<ExpType>() - 1);
        let all_zeros =
            !exp_diff.is_zero() && b_mant.trailing_zeros() + 1 == exp_diff.as_::<ExpType>();

        // Append extra bits to the mantissas to ensure correct rounding
        a_mant = a_mant << 1 as ExpType;
        b_mant = b_mant << 1 as ExpType;

        let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let shifted_b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUintD8::ZERO),
            None => BUintD8::ZERO,
        };

        // If the shift causes an overflow, the b_mant is too small so is set to 0

        if sticky_bit {
            //b_mant |= 1;
        }

        let mut mant = a_mant - shifted_b_mant;

        if mant.bits() == Self::MB + 2 {
            if mant & BUintD8::from(0b10u8) == BUintD8::from(0b10u8) && !sticky_bit {
                mant += BUintD8::ONE;
            }

            mant >>= 1 as ExpType;
        } else {
            a_exp -= BIntD8::ONE;
            a_mant <<= 1 as ExpType;
            b_mant <<= 1 as ExpType;

            let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

            // If the shift causes an overflow, the b_mant is too small so is set to 0
            let shifted_b_mant = match exp_diff.try_into().ok() {
                Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUintD8::ZERO),
                None => BUintD8::ZERO,
            };

            // If the shift causes an overflow, the b_mant is too small so is set to 0

            if sticky_bit {
                //b_mant |= 1;
            }

            mant = a_mant - shifted_b_mant;

            if mant.bits() == Self::MB + 2 {
                if mant & BUintD8::from(0b10u8) == BUintD8::from(0b10u8) && !sticky_bit {
                    mant += BUintD8::ONE;
                }

                mant >>= 1 as ExpType;
            } else {
                let _half_way = (); // TODO
                                    //println!("sticky: {}", sticky_bit);
                if sticky_bit2 && !all_zeros
                    || (sticky_bit2
                        && all_zeros
                        && b_mant & BUintD8::from(0b1u8) == BUintD8::from(0b1u8))
                {
                    //println!("sub");
                    mant -= BUintD8::ONE;
                }
                let bits = mant.bits();
                mant <<= Self::MB + 1 - bits;
                a_exp -= (MB as i64 + 2 - bits as i64).as_::<BIntD8<W>>();
                if !a_exp.is_positive() {
                    a_exp = BIntD8::ONE;
                    mant >>= BIntD8::ONE - a_exp;
                }
            }
        }

        if (mant >> Self::MB).is_zero() {
            a_exp = BIntD8::ZERO;
        } else {
            mant ^= BUintD8::ONE << Self::MB;
        }

        Self::from_raw_parts(negative, a_exp.to_bits(), mant)
    }
}

impl<const W: usize, const MB: usize> Sub for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        //println!("{:064b} {:064b}", self.to_bits(), rhs.to_bits());
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                match (self.is_sign_negative(), rhs.is_sign_negative()) {
                    (true, false) => Self::NEG_INFINITY,
                    (false, true) => Self::INFINITY,
                    _ => Self::NAN,
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs.neg(),
            (_, _) => {
                let self_negative = self.is_sign_negative();
                let rhs_negative = rhs.is_sign_negative();
                if self_negative ^ rhs_negative {
                    self.add_internal(rhs, self_negative)
                } else {
                    self.sub_internal(rhs, self_negative)
                }
            }
        }
    }
}

//crate::errors::op_ref_impl!(Sub<Float<N, MB>> for Float<N, MB>, sub);

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn _mul_internal_old(self, rhs: Self, negative: bool) -> Self
    where
        [(); W * 2]:,
    {
        let (a, b) = (self, rhs);
        let (exp_a, mant_a) = a.exp_mant();
        let (exp_b, mant_b) = b.exp_mant();

        // TODO: make so as_ can infer type so can switch trait definition if needed
        let mant_prod = mant_a.as_::<BUintD8<{ W * 2 }>>() * mant_b.as_::<BUintD8<{ W * 2 }>>();

        let prod_bits = mant_prod.bits();

        if prod_bits == 0 {
            return if negative { Self::NEG_ZERO } else { Self::ZERO };
        }

        let extra_bits = if prod_bits > (Self::MB + 1) {
            prod_bits - (Self::MB + 1)
        } else {
            0
        };

        let mut exp =
            BIntD8::from_bits(exp_a) + BIntD8::from_bits(exp_b) + BIntD8::from(extra_bits)
                - Self::EXP_BIAS
                - BIntD8::from(MB);

        if exp > Self::MAX_EXP + Self::EXP_BIAS - BIntD8::ONE {
            //println!("rhs: {}", rhs.to_bits());
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let mut extra_shift = BUintD8::ZERO;
        if !exp.is_positive() {
            extra_shift = (BIntD8::ONE - exp).to_bits();
            exp = BIntD8::ONE;
        }
        let total_shift = BUintD8::from(extra_bits) + extra_shift;

        let sticky_bit = BUintD8::from(mant_prod.trailing_zeros() + 1) < total_shift;
        let mut mant = match (total_shift - BUintD8::ONE).to_exp_type() {
            Some(sub) => mant_prod.checked_shr(sub).unwrap_or(BUintD8::ZERO),
            None => BUintD8::ZERO,
        };
        if mant & BUintD8::ONE == BUintD8::ONE {
            if sticky_bit || mant & BUintD8::from(0b11u8) == BUintD8::from(0b11u8) {
                // Round up
                mant += BUintD8::ONE;
            }
        }
        mant >>= 1 as ExpType;

        if exp == BIntD8::ONE && mant.bits() < Self::MB + 1 {
            return Self::from_exp_mant(negative, BUintD8::ZERO, mant.as_());
        }
        if mant >> Self::MB != BUintD8::ZERO {
            mant ^= BUintD8::ONE << Self::MB;
        }
        Self::from_exp_mant(negative, exp.to_bits(), mant.as_())
    }

    #[inline]
    fn mul_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = (self, rhs);
        let (_, exp_a, mant_a) = a.to_parts_biased();
        let (_, exp_b, mant_b) = b.to_parts_biased();

        // TODO: make so as_ can infer type so can switch trait definition if needed
        let mut mant_prod = mant_a.widening_mul(mant_b);

        let prod_bits = if mant_prod.1.bits() == 0 {
            mant_prod.0.bits()
        } else {
            mant_prod.1.bits() + Self::BITS
        };

        if prod_bits == 0 {
            return if negative { Self::NEG_ZERO } else { Self::ZERO };
        }

        let extra_bits = if prod_bits > (Self::MB + 1) {
            prod_bits - (Self::MB + 1)
        } else {
            0
        };

        let mut exp =
            BIntD8::from_bits(exp_a) + BIntD8::from_bits(exp_b) + BIntD8::from(extra_bits)
                - Self::EXP_BIAS
                - BIntD8::from(MB);

        if exp > Self::MAX_EXP + Self::EXP_BIAS - BIntD8::ONE {
            //println!("rhs: {}", rhs.to_bits());
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let mut extra_shift = BUintD8::ZERO;
        if !exp.is_positive() {
            extra_shift = (BIntD8::ONE - exp).to_bits();
            exp = BIntD8::ONE;
        }
        let total_shift = BUintD8::from(extra_bits) + extra_shift;

        let mp0tz = mant_prod.0.trailing_zeros();
        let tz = if mp0tz == Self::BITS {
            mant_prod.1.trailing_zeros() + Self::BITS
        } else {
            mp0tz
        };

        let sticky_bit = BUintD8::from(tz + 1) < total_shift;
        let mut mant = match (total_shift - BUintD8::ONE).to_exp_type() {
            Some(sub) => {
                if sub > Self::BITS * 2 {
                    (BUintD8::ZERO, BUintD8::ZERO)
                } else if sub >= Self::BITS {
                    (mant_prod.1 >> (sub - Self::BITS), BUintD8::ZERO)
                } else {
                    let mask = BUintD8::MAX >> (Self::BITS - sub);
                    let carry = mant_prod.1 & mask;
                    mant_prod.1 >>= sub;
                    mant_prod.0 = (mant_prod.0 >> sub) | (carry << (Self::BITS - sub));
                    mant_prod
                }
            }
            None => (BUintD8::ZERO, BUintD8::ZERO),
        };
        if mant.0.bit(0) {
            if sticky_bit || mant.0.bit(1) {
                // Round up
                let (sum, carry) = mant.0.overflowing_add(BUintD8::ONE);
                mant.0 = sum;
                if carry {
                    mant.1 += BUintD8::ONE;
                }
            }
        }
        {
            let carry = mant.1.bit(0);
            //mant.1 >>= 1 as ExpType;
            mant.0 >>= 1 as ExpType;
            if carry {
                mant.0 |= BIntD8::MIN.to_bits();
            }
        }

        let mut m1b = mant.1.bits();
        if m1b != 0 {
            m1b -= 1;
        }
        /*let bits = if m1b == 0 {
            mant.0.bits()
        } else {
            m1b + Self::BITS
        };*/
        let m0b = mant.0.bits();
        if m0b > Self::MB + 1 {
            // it's possible that the mantissa has too many bits, so shift it right and increase the exponent until it has the correct number of bits
            let inc = m0b - (Self::MB + 1);
            mant.0 = mant.0 >> inc;
            exp += BIntD8::from(inc);
        }

        if exp > Self::MAX_EXP + Self::EXP_BIAS - BIntD8::ONE {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        if exp == BIntD8::ONE && m1b < Self::MB + 1 {
            return Self::from_raw_parts(negative, BUintD8::ZERO, mant.0);
        }
        //if mant >> Self::MB != BUintD8::ZERO {
        mant.0 ^= BUintD8::ONE << Self::MB;
        //}
        Self::from_raw_parts(negative, exp.to_bits(), mant.0)
    }
}

impl<const W: usize, const MB: usize> Mul for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => return Self::NAN,
            (FpCategory::Infinite, FpCategory::Zero) | (FpCategory::Zero, FpCategory::Infinite) => {
                Self::NAN
            }
            (FpCategory::Infinite, _) | (_, FpCategory::Infinite) => {
                if negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
            (_, _) => self.mul_internal(rhs, negative),
        }
    }
}

impl<const W: usize, const MB: usize> Product for Float<W, MB>
where
    [(); W * 2]:,
{
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MB: usize> Product<&'a Self> for Float<W, MB>
where
    [(); W * 2]:,
{
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn div_internal(self, rhs: Self, negative: bool) -> Self
    where
        [(); W * 2]:,
    {
        let (a, b) = (self, rhs);
        let (_, e1, s1) = a.to_parts_biased();
        let (_, e2, s2) = b.to_parts_biased();

        let b1 = s1.bits();
        let b2 = s2.bits();

        let mut e =
            BIntD8::from_bits(e1) - BIntD8::from_bits(e2) + Self::EXP_BIAS + BIntD8::from(b1)
                - BIntD8::from(b2)
                - BIntD8::ONE;

        let mut extra_shift = BUintD8::ZERO;
        if !e.is_positive() {
            extra_shift = (BIntD8::ONE - e).to_bits();
            e = BIntD8::ONE;
        }

        let total_shift =
            BIntD8::from(MB as i32 + 1 + b2 as i32 - b1 as i32) - BIntD8::from_bits(extra_shift);

        let large = if !total_shift.is_negative() {
            (s1.as_::<BUintD8<{ W * 2 }>>()) << total_shift
        } else {
            (s1.as_::<BUintD8<{ W * 2 }>>()) >> (-total_shift)
        };
        let mut division = (large / (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();

        let rem = if division.bits() != Self::MB + 2 {
            let rem = (large % (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            rem
        } else {
            e += BIntD8::ONE;
            division =
                ((large >> 1 as ExpType) / (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            //println!("div {} {}", large >> 1u8, s2);
            let rem =
                ((large >> 1 as ExpType) % (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            rem
        };
        //println!("{}", rem);
        if rem * BUintD8::TWO > s2 {
            division += BUintD8::ONE;
        } else if rem * BUintD8::TWO == s2 {
            if (division & BUintD8::ONE) == BUintD8::ONE {
                division += BUintD8::ONE;
            }
        }
        if division.bits() == Self::MB + 2 {
            e += BIntD8::ONE;
            division >>= 1 as ExpType;
        }

        if e > Self::MAX_EXP + Self::EXP_BIAS - BIntD8::ONE {
            return Self::INFINITY;
        }

        //println!("{:032b}", division);

        if e == BIntD8::ONE && division.bits() < Self::MB + 1 {
            return Self::from_exp_mant(negative, BUintD8::ZERO, division);
        }

        if division >> Self::MB != BUintD8::ZERO {
            division ^= BUintD8::ONE << Self::MB;
        }
        Self::from_exp_mant(negative, e.to_bits(), division)
    }
}

impl<const W: usize, const MB: usize> Div for Float<W, MB>
where
    [(); W * 2]:,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Infinite) => Self::NAN,
            (FpCategory::Zero, FpCategory::Zero) => Self::NAN,
            (FpCategory::Infinite, _) | (_, FpCategory::Zero) => {
                if negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
            (FpCategory::Zero, _) | (_, FpCategory::Infinite) => {
                if negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => self.div_internal(rhs, negative),
        }
    }
}

impl<const W: usize, const MB: usize> Rem for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn rem(self, y: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(y; y);

        if y.is_zero() {
            return Self::NAN;
        }
        if self.is_zero() {
            return self;
        }
        if self.is_infinite() {
            return Self::NAN;
        }
        if y.is_infinite() {
            return self;
        }

        let mut uxi = self.to_bits();
        let mut uyi = y.to_bits();
        let mut ex = self.exponent();
        let mut ey = y.exponent();
        let mut i;

        if uxi << 1 as ExpType <= uyi << 1 as ExpType {
            if uxi << 1 as ExpType == uyi << 1 as ExpType {
                return if self.is_sign_negative() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }

            return self;
        }

        /* normalize x and y */
        if ex.is_zero() {
            i = uxi << (Self::BITS - Self::MB);
            while !BIntD8::from_bits(i).is_negative() {
                ex -= BIntD8::ONE;
                i <<= 1 as ExpType;
            }

            uxi <<= -ex + BIntD8::ONE;
        } else {
            uxi &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uxi |= BUintD8::ONE << Self::MB;
        }
        //println!("{}", i);

        if ey.is_zero() {
            i = uyi << (Self::BITS - Self::MB);
            while !BIntD8::from_bits(i).is_negative() {
                ey -= BIntD8::ONE;
                i <<= 1 as ExpType;
            }

            uyi <<= -ey + BIntD8::ONE;
        } else {
            uyi &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uyi |= BUintD8::ONE << Self::MB;
        }
        /* x mod y */
        while ex > ey {
            i = uxi.wrapping_sub(uyi);
            if !BIntD8::from_bits(i).is_negative() {
                if i.is_zero() {
                    return if self.is_sign_negative() {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    };
                }
                uxi = i;
            }
            uxi <<= 1 as ExpType;

            ex -= BIntD8::ONE;
        }

        i = uxi.wrapping_sub(uyi);
        if !BIntD8::from_bits(i).is_negative() {
            if i.is_zero() {
                return if self.is_sign_negative() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            uxi = i;
        }

        while (uxi >> Self::MB).is_zero() {
            uxi <<= 1 as ExpType;
            ex -= BIntD8::ONE;
        }

        /* scale result up */
        if ex.is_positive() {
            uxi -= BUintD8::ONE << Self::MB;
            uxi |= (ex.to_bits()) << Self::MB;
        } else {
            uxi >>= -ex + BIntD8::ONE;
        }

        let f = Self::from_bits(uxi);
        if self.is_sign_negative() {
            -f
        } else {
            f
        }
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn neg(mut self) -> Self {
        self.bits.digits[W - 1] ^= 1 << (Digit::BITS - 1);
        self
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const Neg for Float<W, MB> {
        type Output = Self;

        #[inline]
        fn neg(self) -> Self {
            Self::neg(self)
        }
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const Neg for &Float<W, MB> {
        type Output = Float<W, MB>;

        #[inline]
        fn neg(self) -> Float<W, MB> {
            (*self).neg()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    /*test_bignum! {
        function: <ftest as Add>::add(a: ftest, b: ftest)
    }*/

    test_bignum! {
        function: <ftest as Sub>::sub(a: ftest, b: ftest)
    }

    test_bignum! {
        function: <ftest as Mul>::mul(a: ftest, b: ftest),
        cases: [
            (5.6143642e23f64 as ftest, 35279.223f64 as ftest)
        ]
    }

    test_bignum! {
        function: <ftest as Div>::div(a: ftest, b: ftest)
    }

    test_bignum! {
        function: <ftest as Rem>::rem(a: ftest, b: ftest)
    }

    test_bignum! {
        function: <ftest as Neg>::neg(f: ftest)
    }
}
