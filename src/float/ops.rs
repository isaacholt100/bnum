use super::Float;
use core::num::FpCategory;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::convert::TryInto;
use crate::{BUint, Bint, ExpType, digit};
use core::iter::{Product, Sum, Iterator};
use crate::cast::As;

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
        let (mut a_exp, mut a_mant) = self.exp_mant();
        let (b_exp, mut b_mant) = rhs.exp_mant();
    
        let exp_diff = a_exp - b_exp;
    
        let sticky_bit = BUint::from(b_mant.trailing_zeros() + 1) < exp_diff;
    
        // Append extra bits to the mantissas to ensure correct rounding
        a_mant <<= 2 as ExpType;
        b_mant <<= 2 as ExpType;
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
        b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
            None => BUint::ZERO,
        };
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
    
        if sticky_bit {
            b_mant |= BUint::ONE;
        }
    
        let mut mant = a_mant + b_mant;
    
        let overflow = !(mant >> (MB + 3)).is_zero();
        if !overflow {
            if mant & BUint::from_digit(0b11) == BUint::from_digit(0b11) || mant & BUint::from_digit(0b110) == BUint::from_digit(0b110) {
                mant += BUint::FOUR;
                if !(mant >> (MB + 3)).is_zero() {
                    mant >>= 1 as ExpType;
                    a_exp += BUint::ONE;
                }
            }
        } else {
            match (mant & BUint::from_digit(0b111)).digits()[0] {
                0b111 | 0b110 | 0b101 => {
                    mant += BUint::EIGHT;
                },
                0b100 => {
                    if mant & BUint::from_digit(0b1000) == BUint::from_digit(0b1000) {
                        mant += BUint::EIGHT;
                    }
                },
                _ => {},
            }
            
            mant >>= 1 as ExpType;
            a_exp += BUint::ONE;
        }
        if a_exp > Self::MAX_UNBIASED_EXP {
            return Self::INFINITY;
        }
    
        mant >>= 2 as ExpType;
    
        if (mant >> Self::MB).is_zero() {
            a_exp = BUint::ZERO;
        } else {
            mant ^= BUint::ONE << Self::MB;
        }
        //println!("{}", negative);
        //assert!(negative);
            let a = Self::from_exp_mant(false, a_exp, mant).neg();
            assert!(a.is_sign_negative());
            a
    }
}

impl<const W: usize, const MB: usize> Add for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        let self_negative = self.is_sign_negative();
        let rhs_negative = rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => return self,
            (_, FpCategory::Nan) => return rhs,
            (FpCategory::Infinite, _) => return self,
            (_, FpCategory::Infinite) => return rhs,
            (FpCategory::Zero, FpCategory::Zero) => return if self_negative && rhs_negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            },
            (_, _) => {
                if self_negative ^ rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    let r = self.add_internal(rhs, self_negative);
                    assert!(r.is_sign_negative());
                    r
                }
            }
        }
    }
}

crate::macros::op_ref_impl!(Add<Float<N, MB>> for Float<N, MB>, add);

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
    
        let (a_exp, mut a_mant) = self.exp_mant();
        let (b_exp, mut b_mant) = rhs.exp_mant();
        let exp_diff = a_exp - b_exp;
    
        let mut a_exp = Bint::from_bits(a_exp);
    
        let sticky_bit2 = !exp_diff.is_zero() && exp_diff < BUint::<W>::BITS.into() && b_mant.bit(exp_diff.as_::<ExpType>() - 1);
        let all_zeros = !exp_diff.is_zero() && b_mant.trailing_zeros() + 1 == exp_diff.as_();
    
    
        // Append extra bits to the mantissas to ensure correct rounding
        a_mant = a_mant << 1 as ExpType;
        b_mant = b_mant << 1 as ExpType;
    
        let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let shifted_b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
            None => BUint::ZERO,
        };
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
    
        if sticky_bit {
            //b_mant |= 1;
        }
    
        let mut mant = a_mant - shifted_b_mant;
    
        if mant.bits() == Self::MB + 2 {
            if mant & BUint::from(0b10u8) == BUint::from(0b10u8) && !sticky_bit {
                mant += BUint::ONE;
            }
    
            mant >>= 1 as ExpType;
        } else {
            a_exp -= Bint::ONE;
            a_mant <<= 1 as ExpType;
            b_mant <<= 1 as ExpType;
    
            let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();
    
            // If the shift causes an overflow, the b_mant is too small so is set to 0
            let shifted_b_mant = match exp_diff.try_into().ok() {
                Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
                None => BUint::ZERO,
            };
    
            // If the shift causes an overflow, the b_mant is too small so is set to 0
    
            if sticky_bit {
                //b_mant |= 1;
            }
    
            mant = a_mant - shifted_b_mant;

            if mant.bits() == Self::MB + 2 {
                if mant & BUint::from(0b10u8) == BUint::from(0b10u8) && !sticky_bit {
                    mant += BUint::ONE;
                }
    
                mant >>= 1 as ExpType;
            } else {
                
                let _half_way = (); // TODO
                //println!("sticky: {}", sticky_bit);
                if sticky_bit2 && !all_zeros || (sticky_bit2 && all_zeros && b_mant & BUint::from(0b1u8) == BUint::from(0b1u8)) {
                    //println!("sub");
                    mant -= BUint::ONE;
                }
                let bits = mant.bits();
                mant <<= Self::MB + 1 - bits;
                a_exp -= Bint::from(MB as i64 + 2 - bits as i64);
                if !a_exp.is_positive() {
                    a_exp = Bint::ONE;
                    mant >>= Bint::ONE - a_exp;
                }
            }
        }
    
        if (mant >> Self::MB).is_zero() {
            a_exp = Bint::ZERO;
        } else {
            mant ^= BUint::ONE << Self::MB;
        }
        
        Self::from_exp_mant(negative, a_exp.to_bits(), mant)
    }
}

impl<const W: usize, const MB: usize> Sub for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => return self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => return Self::NEG_NAN,
            (FpCategory::Infinite, _) => return self,
            (_, FpCategory::Infinite) => return rhs.neg(),
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

crate::macros::op_ref_impl!(Sub<Float<N, MB>> for Float<N, MB>, sub);

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn mul_internal(self, rhs: Self, negative: bool) -> Self where [(); W * 2]:, {
        let (a, b) = (self, rhs);
        let (exp_a, mant_a) = a.exp_mant();
        let (exp_b, mant_b) = b.exp_mant();

        // TODO: make so as_ can infer type so can switch trait definition if needed
        let mant_prod = mant_a.as_::<BUint<{W * 2}>>() * mant_b.as_::<BUint<{W * 2}>>();

        let prod_bits = mant_prod.bits();

        if prod_bits == 0 {
            return if negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }

        let extra_bits = if prod_bits > (Self::MB + 1) {
            prod_bits - (Self::MB + 1)
        } else {
            0
        };

        let mut exp = Bint::from_bits(exp_a) + Bint::from_bits(exp_b) + Bint::from(extra_bits) - Self::EXP_BIAS - Bint::from(MB);

        if exp > Self::MAX_EXP + Self::EXP_BIAS - Bint::ONE {
            //println!("rhs: {}", rhs.to_bits());
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let mut extra_shift = BUint::ZERO;
        if !exp.is_positive() {
            extra_shift = (Bint::ONE - exp).to_bits();
            exp = Bint::ONE;
        }
        let total_shift = BUint::from(extra_bits) + extra_shift;

        let sticky_bit = BUint::from(mant_prod.trailing_zeros() + 1) < total_shift;
        let mut mant = match (total_shift - BUint::ONE).to_exp_type() {
            Some(sub) => mant_prod.checked_shr(sub).unwrap_or(BUint::ZERO),
            None => BUint::ZERO,
        };
        if mant & BUint::ONE == BUint::ONE {
            if sticky_bit || mant & BUint::from(0b11u8) == BUint::from(0b11u8) {
                // Round up
                mant += BUint::ONE;
            }
        }
        mant >>= 1 as ExpType;

        if exp == Bint::ONE && mant.bits() < Self::MB + 1 {
            return Self::from_exp_mant(negative, BUint::ZERO, mant.as_());
        }
        if mant >> Self::MB != BUint::ZERO {
            mant ^= BUint::ONE << Self::MB as u32;
        }
        Self::from_exp_mant(negative, exp.to_bits(), mant.as_())
    }
}

impl<const W: usize, const MB: usize> Mul for Float<W, MB> where [(); W * 2]:, {
    type Output = Self;
    
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => return Self::NAN,
            (FpCategory::Infinite, FpCategory::Zero) | (FpCategory::Zero, FpCategory::Infinite) => Self::NEG_NAN,
            (FpCategory::Infinite, _) | (_, FpCategory::Infinite) => if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            },
            (_, _) => {
                self.mul_internal(rhs, negative)
            },
        }
    }
}

impl<const W: usize, const MB: usize> Product for Float<W, MB>  where [(); W * 2]:, {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MB: usize> Product<&'a Self> for Float<W, MB> where [(); W * 2]:, {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    fn div_internal(self, rhs: Self, negative: bool) -> Self where [(); W * 2]:, {
        let (a, b) = (self, rhs);
        let (e1, s1) = a.exp_mant();
        let (e2, s2) = b.exp_mant();
    
        let b1 = s1.bits();
        let b2 = s2.bits();
    
        let mut e = Bint::from_bits(e1) - Bint::from_bits(e2) + Self::EXP_BIAS + Bint::from(b1) - Bint::from(b2) - Bint::ONE;
    
        let mut extra_shift = BUint::ZERO;
        if !e.is_positive() {
            extra_shift = (Bint::ONE - e).to_bits();
            e = Bint::ONE;
        }
    
        let total_shift = Bint::from(MB as i32 + 1 + b2 as i32 - b1 as i32) - Bint::from_bits(extra_shift);
    
        let large = if !total_shift.is_negative() {
            (s1.as_::<BUint<{W * 2}>>()) << total_shift
        } else {
            (s1.as_::<BUint<{W * 2}>>()) >> (-total_shift)
        };
        let mut division = (large / (s2.as_::<BUint<{W * 2}>>())).as_::<BUint<W>>();
    
        let rem = if division.bits() != Self::MB + 2 {
            let rem = (large % (s2.as_::<BUint<{W * 2}>>())).as_::<BUint<W>>();
            rem
        } else {
            e += Bint::ONE;
            division = ((large >> 1 as ExpType) / (s2.as_::<BUint<{W * 2}>>())).as_::<BUint<W>>();
            //println!("div {} {}", large >> 1u8, s2);
            let rem = ((large >> 1 as ExpType) % (s2.as_::<BUint<{W * 2}>>())).as_::<BUint<W>>();
            rem
        };
        //println!("{}", rem);
        if rem * BUint::TWO > s2 {
            division += BUint::ONE;
        } else if rem * BUint::TWO == s2 {
            if (division & BUint::ONE) == BUint::ONE {
                division += BUint::ONE;
            }
        }
        if division.bits() == Self::MB + 2 {
            e += Bint::ONE;
            division >>= 1 as ExpType;
        }
    
        if e > Self::MAX_EXP + Self::EXP_BIAS - Bint::ONE {
            return Self::INFINITY;
        }

        //println!("{:032b}", division);
    
        if e == Bint::ONE && division.bits() < Self::MB + 1 {
            return Self::from_exp_mant(negative, BUint::ZERO, division);
        }
    
        if division >> Self::MB != BUint::ZERO {
            division ^= BUint::ONE << Self::MB;
        }
        Self::from_exp_mant(negative, e.to_bits(), division)
    }
}

impl<const W: usize, const MB: usize> Div for Float<W, MB> where [(); W * 2]:, {
    type Output = Self;
    
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Infinite) => Self::NEG_NAN,
            (FpCategory::Zero, FpCategory::Zero) => {
                Self::NEG_NAN
            },
            (FpCategory::Infinite, _) | (_, FpCategory::Zero) => if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            },
            (FpCategory::Zero, _) | (_, FpCategory::Infinite) => if negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            },
            (_, _) => {
                self.div_internal(rhs, negative)
            },
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
            while !Bint::from_bits(i).is_negative() {
                ex -= Bint::ONE;
                i <<= 1 as ExpType;
            }

            uxi <<= -ex + Bint::ONE;
        } else {
            uxi &= BUint::MAX >> (Self::BITS - Self::MB);
            uxi |= BUint::ONE << Self::MB;
        }
        //println!("{}", i);

        if ey.is_zero() {
            i = uyi << (Self::BITS - Self::MB);
            while !Bint::from_bits(i).is_negative() {
                ey -= Bint::ONE;
                i <<= 1 as ExpType;
            }

            uyi <<= -ey + Bint::ONE;
        } else {
            uyi &= BUint::MAX >> (Self::BITS - Self::MB);
            uyi |= BUint::ONE << Self::MB;
        }
        /* x mod y */
        while ex > ey {
            i = uxi.wrapping_sub(uyi);
            if !Bint::from_bits(i).is_negative() {
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

            ex -= Bint::ONE;
        }

        i = uxi.wrapping_sub(uyi);
        if !Bint::from_bits(i).is_negative() {
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
            ex -= Bint::ONE;
        }

        /* scale result up */
        if ex.is_positive() {
            uxi -= BUint::ONE << Self::MB;
            uxi |= (ex.to_bits()) << Self::MB;
        } else {
            uxi >>= -ex + Bint::ONE;
        }

        let f = Self::from_bits(uxi);
        if self.is_sign_negative() {
            -f
        } else {
            f
        }
    }
}

impl<const W: usize, const MB: usize> const Neg for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let mut words = *self.words();
        words[W - 1] ^= 1 << (digit::BITS - 1);
        Self::from_words(words)
    }
}

impl<const W: usize, const MB: usize> const Neg for &Float<W, MB> {
    type Output = Float<W, MB>;

    #[inline]
    fn neg(self) -> Float<W, MB> {
        (*self).neg()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::F64;

    trait ToBits {
        fn to_bits(self) -> u64;
    }
    impl ToBits for F64 {
        fn to_bits(self) -> u64 {
            self.to_bits().as_()
        }
    }
    impl ToBits for f64 {
        fn to_bits(self) -> u64 {
            self.to_bits()
        }
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Add>::add(a: f64, b: f64),
        quickcheck_skip: a.is_sign_positive() || b.is_sign_positive()
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Sub>::sub(a: f64, b: f64),
        quickcheck_skip: a.is_sign_negative() != b.is_sign_negative()
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Mul>::mul(a: f64, b: f64)
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Div>::div(a: f64, b: f64)
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Rem>::rem(a: f64, b: f64)
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Neg>::neg(f: f64)
    }

    #[test]
    fn sub() {
        let f1 = f64::from_bits(0b1100001111100000000000000000000000000000000000000000000000000000);
        let f2 = f64::from_bits(0b1111111110001111001000000100000100100001110101001010110010110011);
        //println!("{:064b}", ((-0.0f64).div_euclid(f2)).to_bits());
        let a = (crate::F64::from(f1) + (crate::F64::from(f2))).to_bits();
        let b = (f1 + (f2)).to_bits();
        println!("{:064b}", a);
        println!("{:064b}", b);
        assert!(a == b.into());
    }
}