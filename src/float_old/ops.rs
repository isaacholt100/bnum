use super::Float;
use core::num::FpCategory;
use core::ops::{Add, Sub, Mul, Div, Neg};
use crate::{BUint, Bint, ExpType, digit};
use num_traits::ToPrimitive;
use core::iter::{Product, Sum, Iterator};

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    #[inline]    
    fn add_internal(mut self, mut rhs: Self, negative: bool) -> Self {
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
        a_mant <<= 2u32;
        b_mant <<= 2u32;
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
        b_mant = match exp_diff.to_usize() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
            None => BUint::ZERO,
        };
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
    
        if sticky_bit {
            b_mant |= BUint::ONE;
        }
    
        let mut mant = a_mant + b_mant;
    
        let overflow = !(mant >> (MANTISSA_BITS + 3)).is_zero();
        if !overflow {
            if mant & BUint::from_digit(0b11) == BUint::from_digit(0b11) || mant & BUint::from_digit(0b110) == BUint::from_digit(0b110) {
                mant += 0b100;
                if !(mant >> (MANTISSA_BITS + 3)).is_zero() {
                    mant >>= 1u32;
                    a_exp += 1;
                }
            }
        } else {
            match (mant & BUint::from_digit(0b111)).digits()[0] {
                0b111 | 0b110 | 0b101 => {
                    mant += 0b1000;
                },
                0b100 => {
                    if mant & BUint::from_digit(0b1000) == BUint::from_digit(0b1000) {
                        mant += 0b1000;
                    }
                },
                _ => {},
            }
            
            mant >>= 1u32;
            a_exp += 1;
        }
        if a_exp > Self::MAX_UNBIASED_EXP {
            return Self::INFINITY;
        }
    
        mant >>= 2u32;
    
        if (mant >> MANTISSA_BITS).is_zero() {
            a_exp = BUint::ZERO;
        } else {
            mant ^= BUint::ONE << MANTISSA_BITS;
        }
        Self::from_exp_mant(negative, a_exp, mant)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Add for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let self_negative = self.is_sign_negative();
        let rhs_negative = rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => return self,
            (_, FpCategory::Nan) => return rhs,
            (FpCategory::Infinite, _) => return self,
            (_, FpCategory::Infinite) => return rhs,
            (_, _) => {
                if self_negative ^ rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    self.add_internal(rhs, self_negative)
                }
            }
        }
    }
}

crate::macros::op_ref_impl!(Add<Float<N, MANTISSA_BITS>> for Float<N, MANTISSA_BITS>, add);

impl<const W: usize, const MANTISSA_BITS: usize> Sum for Float<W, MANTISSA_BITS> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const W: usize, const MANTISSA_BITS: usize> Sum<&'a Self> for Float<W, MANTISSA_BITS> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a + *b)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    #[inline]
    fn sub_internal(mut self, mut rhs: Self, negative: bool) -> Self {
        let mut negative = negative;
        if rhs.abs() > self.abs() {
            // If b has a larger exponent than a, swap a and b so that a has the larger exponent
            negative = !negative;
            core::mem::swap(&mut self, &mut rhs);
        }
    
        let (a_exp, mut a_mant) = self.exp_mant();
        let (b_exp, mut b_mant) = rhs.exp_mant();
    
        let exp_diff = a_exp - b_exp;
    
        let mut a_exp = Bint::from_bits(a_exp);
    
        let sticky_bit2 = !exp_diff.is_zero() && exp_diff < BUint::<W>::BITS.into() && b_mant.bit(exp_diff.as_usize() - 1);
        let all_zeros = !exp_diff.is_zero() && b_mant.trailing_zeros() + 1 == exp_diff.as_usize();
    
    
        // Append extra bits to the mantissas to ensure correct rounding
        a_mant <<= 1u8;
        b_mant <<= 1u8;
    
        let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_usize();
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let shifted_b_mant = match exp_diff.to_usize() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
            None => BUint::ZERO,
        };
    
        // If the shift causes an overflow, the b_mant is too small so is set to 0
    
        if sticky_bit {
            //b_mant |= 1;
        }
    
        let mut mant = a_mant - shifted_b_mant;
    
        if mant.bits() == MANTISSA_BITS as ExpType + 2 {
            if mant & BUint::from(0b10u8) == BUint::from(0b10u8) && !sticky_bit {
                mant += 0b1;
            }
    
            mant >>= 1u8;
        } else {
            a_exp -= Bint::ONE;
            a_mant <<= 1u8;
            b_mant <<= 1u8;
    
            let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_usize();
    
            // If the shift causes an overflow, the b_mant is too small so is set to 0
            let shifted_b_mant = match exp_diff.to_usize() {
                Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUint::ZERO),
                None => BUint::ZERO,
            };
    
            // If the shift causes an overflow, the b_mant is too small so is set to 0
    
            if sticky_bit {
                //b_mant |= 1;
            }
    
            mant = a_mant - shifted_b_mant;

            if mant.bits() == MANTISSA_BITS as ExpType + 2 {
                if mant & BUint::from(0b10u8) == BUint::from(0b10u8) && !sticky_bit {
                    mant += 0b1;
                }
    
                mant >>= 1u8;
            } else {
                
                let _half_way = (); // TODO
                //println!("sticky: {}", sticky_bit);
                if sticky_bit2 && !all_zeros || (sticky_bit2 && all_zeros && b_mant & BUint::from(0b1u8) == BUint::from(0b1u8)) {
                    //println!("sub");
                    mant -= BUint::ONE;
                }
                let bits = mant.bits();
                mant <<= MANTISSA_BITS as ExpType + 1 - bits;
                a_exp -= Bint::from(MANTISSA_BITS as i64 + 2 - bits as i64);
                if !a_exp.is_positive() {
                    a_exp = Bint::ONE;
                    mant >>= Bint::ONE - a_exp;
                }
            }
        }
    
        if (mant >> MANTISSA_BITS).is_zero() {
            a_exp = Bint::ZERO;
        } else {
            mant ^= BUint::ONE << MANTISSA_BITS;
        }
        
        Self::from_exp_mant(negative, a_exp.to_bits(), mant)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Sub for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => return self,
            (_, FpCategory::Nan) => rhs.neg(),
            (FpCategory::Infinite, FpCategory::Infinite) => return Self::NAN,
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

crate::macros::op_ref_impl!(Sub<Float<N, MANTISSA_BITS>> for Float<N, MANTISSA_BITS>, sub);

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    #[inline]
    fn mul_internal(self, rhs: Self, negative: bool) -> Self where [(); {(W * 2).saturating_sub(W)
    }]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
        let (a, b) = (self, rhs);
        let (exp_a, mant_a) = a.exp_mant();
        let (exp_b, mant_b) = b.exp_mant();

        let mant_prod = mant_a.as_buint::<{W * 2}>() * mant_b.as_buint::<{W * 2}>();

        let prod_bits = mant_prod.bits();

        if prod_bits == 0 {
            return if negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }

        let extra_bits = if prod_bits > (MANTISSA_BITS + 1) {
            prod_bits - (MANTISSA_BITS + 1)
        } else {
            0
        };

        let mut exp = Bint::from_bits(exp_a) + Bint::from_bits(exp_b) + Bint::from(extra_bits) - Self::EXP_BIAS - Bint::from(MANTISSA_BITS);

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
                mant += 1;
            }
        }
        mant >>= 1u8;

        if exp == Bint::ONE && mant.bits() < MANTISSA_BITS as ExpType + 1 {
            return Self::from_exp_mant(negative, BUint::ZERO, mant.as_buint::<W>());
        }
        if mant >> MANTISSA_BITS != BUint::ZERO {
            mant ^= BUint::ONE << MANTISSA_BITS as u32;
        }
        Self::from_exp_mant(negative, exp.to_bits(), mant.as_buint::<W>())
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Mul for Float<W, MANTISSA_BITS> where [(); (W * 2).saturating_sub(W)]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => return if negative {
                Self::NEG_NAN
            } else {
                Self::NAN
            },
            (FpCategory::Infinite, FpCategory::Zero) | (FpCategory::Zero, FpCategory::Infinite) => if negative {
                Self::NEG_NAN
            } else {
                Self::NAN
            },
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

impl<const W: usize, const MANTISSA_BITS: usize> Product for Float<W, MANTISSA_BITS> where [(); (W * 2).saturating_sub(W)]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MANTISSA_BITS: usize> Product<&'a Self> for Float<W, MANTISSA_BITS> where [(); (W * 2).saturating_sub(W)]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    #[inline]
    fn div_internal(self, rhs: Self, negative: bool) -> Self where [(); {(W * 2).saturating_sub(W)
    }]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
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
    
        let total_shift = Bint::from(MANTISSA_BITS as i32 + 1 + b2 as i32 - b1 as i32) - Bint::from_bits(extra_shift);
    
        let large = if !total_shift.is_negative() {
            (s1.as_buint::<{W * 2}>()) << total_shift
        } else {
            (s1.as_buint::<{W * 2}>()) >> (-total_shift)
        };
        let mut division = (large / (s2.as_buint::<{W * 2}>())).as_buint::<W>();
    
        let rem = if division.bits() != MANTISSA_BITS as ExpType + 2 {
            let rem = (large % (s2.as_buint::<{W * 2}>())).as_buint::<W>();
            rem
        } else {
            e += Bint::ONE;
            division = ((large >> 1u8) / (s2.as_buint::<{W * 2}>())).as_buint::<W>();
            //println!("div {} {}", large >> 1u8, s2);
            let rem = ((large >> 1u8) % (s2.as_buint::<{W * 2}>())).as_buint::<W>();
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
        if division.bits() == MANTISSA_BITS as ExpType + 2 {
            e += Bint::ONE;
            division >>= 1u8;
        }
    
        if e > Self::MAX_EXP + Self::EXP_BIAS - Bint::ONE {
            return Self::INFINITY;
        }

        //println!("{:032b}", division);
    
        if e == Bint::ONE && division.bits() < MANTISSA_BITS as ExpType + 1 {
            return Self::from_exp_mant(negative, BUint::ZERO, division);
        }
    
        if division >> MANTISSA_BITS != BUint::ZERO {
            division ^= BUint::ONE << MANTISSA_BITS;
        }
        Self::from_exp_mant(negative, e.to_bits(), division)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Div for Float<W, MANTISSA_BITS> where [(); (W * 2).saturating_sub(W)]: Sized, [(); W.saturating_sub(W * 2)]: Sized {
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Infinite) => if negative {
                Self::NEG_NAN
            } else {
                Self::NAN
            },
            (FpCategory::Zero, FpCategory::Zero) => {
                Self::NAN
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

impl<const W: usize, const MANTISSA_BITS: usize> const Neg for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut words = *self.words();
        words[W - 1] ^= 1 << (digit::BITS - 1);
        Self::from_words(words)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> const Neg for &Float<W, MANTISSA_BITS> {
    type Output = Float<W, MANTISSA_BITS>;

    fn neg(self) -> Float<W, MANTISSA_BITS> {
        (*self).neg()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Add>::add(a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan() // TODO: need to handle this case
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Sub>::sub(a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan()
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Mul>::mul(a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan()
    }
    
    crate::test::test_op! {
        big: F64,
        primitive: f64,
        function: <Div>::div(a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan()
    }
}