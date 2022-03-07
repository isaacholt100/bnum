use crate::uint::BUint;
use crate::int::Bint;
use crate::digit::{SignedDigit, Digit, self};
use crate::ExpType;
use core::num::FpCategory;

mod consts;
mod ops;

macro_rules! handle_nan {
    ($ret: expr; $($n: expr), +) => {
        if $($n.is_nan()) || + {
            return $ret;
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Float<const W: usize, const MANTISSA_BITS: usize> {
    uint: BUint<W>,
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    const BITS: usize = W * digit::BITS as usize;
    
    const EXPONENT_BITS: usize = Self::BITS - MANTISSA_BITS - 1;

    const MANTISSA_WORDS: (usize, usize) = (MANTISSA_BITS / digit::BITS as usize, MANTISSA_BITS % digit::BITS as usize);

    const EXPONENT_MASK: BUint<W> = BUint::MAX.wrapping_shl(MANTISSA_BITS as ExpType) ^ Bint::MIN.to_bits();
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    pub const fn normalize(self) -> Self {
        if self.is_subnormal() {
            todo!()
        } else {
            self
        }
    }
    pub const fn from_parts(negative: bool, exponent: BUint<W>, mantissa: BUint<W>) -> Self {
        let mut words = *(exponent | mantissa).digits();
        if negative {
            words[W - 1] |= 1 << (digit::BITS - 1);
        }
        Self::from_words(words)
    }
    #[inline(always)]
    const fn from_words(digits: [Digit; W]) -> Self {
        Self {
            uint: BUint::from_digits(digits),
        }
    }
    #[inline(always)]
    const fn words(&self) -> &[Digit; W] {
        self.uint.digits()
    }
    pub const fn is_sign_positive(&self) -> bool {
        !self.is_sign_negative()
    }
    pub const fn is_sign_negative(&self) -> bool {
        self.to_int().is_negative()
    }
    pub const fn abs(self) -> Self {
        if self.is_sign_negative() {
            self.neg()
        } else {
            self
        }
    }
    const fn exponent(self) -> Bint<W> {
        let u: BUint<W> = (self.to_bits() & Bint::MAX.to_bits()).wrapping_shr(MANTISSA_BITS);
        Bint::from_bits(u)
    }
    const fn unshifted_exponent(self) -> Bint<W> {
        Bint::from_bits(self.to_bits() & Self::EXPONENT_MASK)
    }
    const MANTISSA_MASK: BUint<W> = BUint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);
    const fn mantissa(self) -> BUint<W> {
        self.to_bits() & Self::MANTISSA_MASK
    }
    const fn actual_mantissa(self) -> BUint<W> {
        if self.is_subnormal() {
            self.mantissa()
        } else {
            self.mantissa() | (BUint::ONE.wrapping_shl(MANTISSA_BITS))
        }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<W> {
        self.uint
    }
    #[inline(always)]
    const fn to_int(self) -> Bint<W> {
        Bint::from_bits(self.to_bits())
    }
    #[inline(always)]
    pub const fn from_bits(v: BUint<W>) -> Self {
        Self {
            uint: v,
        }
    }
    pub const fn is_finite(self) -> bool {
        (self.to_bits().wrapping_shl(1).leading_ones() as usize) < Self::EXPONENT_BITS
    }
    pub const fn is_nan(self) -> bool {
        !(self.mantissa().is_zero() || self.is_finite())
    }
    pub const fn is_subnormal(self) -> bool {
        !self.is_zero() && self.exponent().is_zero()
    }
    pub const fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < W - 1 {
            if self.words()[i] != 0 {
                return false;
            }
            i += 1;
        }
        let last = self.words()[W - 1];
        last.trailing_zeros() >= Digit::BITS - 1
    }
    pub const fn classify(self) -> FpCategory {
        // TODO: optimise this method better
        if self.is_finite() {
            if self.exponent().is_zero() {
                if self.is_zero() {
                    FpCategory::Zero
                } else {
                    FpCategory::Subnormal
                }
            } else {
                FpCategory::Normal
            }
        } else {
            if self.is_nan() {
                FpCategory::Nan
            } else {
                FpCategory::Infinite
            }
        }
    }
    pub const fn copysign(self, sign: Self) -> Self {
        let mut self_words = *self.words();
        self_words[W - 1] &= (!0) >> 1;
        let mut sign_words = *sign.words();
        sign_words[W - 1] &= 1 << (digit::BITS - 1);
        Self::from_bits(BUint::from_digits(self_words) | (BUint::from_digits(sign_words)))
    }
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }
    pub const fn neg(self) -> Self {
        let mut words = *self.words();
        words[W - 1] ^= 1 << (digit::BITS - 1);
        Self::from_words(words)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Default for Float<W, MANTISSA_BITS> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    pub const fn to_be_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_be_bytes()
    }
    pub const fn to_le_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_le_bytes()
    }
    pub const fn to_ne_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_ne_bytes()
    }
    pub const fn from_be_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_be_bytes(bytes))
    }
    pub const fn from_le_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_le_bytes(bytes))
    }
    pub const fn from_ne_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_ne_bytes(bytes))
    }
}
use num_traits::ToPrimitive;

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    pub const fn max(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        match self.total_cmp(&other) {
            Ordering::Less => other,
            _ => self,
        }
    }
    pub const fn min(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        match self.total_cmp(&other) {
            Ordering::Greater => other,
            _ => self,
        }
    }
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        match Self::partial_cmp(&min, &max) {
            None | Some(Ordering::Greater) => panic!("assertion failed: min <= max"),
            _ => {
                handle_nan!(self; self);
                let is_zero = self.is_zero();
                if is_zero && min.is_zero() {
                    return self;
                }
                if let Ordering::Less = Self::total_cmp(&self, &min) {
                    return min;
                }
                if is_zero && max.is_zero() {
                    return self;
                }
                if let Ordering::Greater = Self::total_cmp(&self, &max) {
                    return max;
                }
                self
            }
        }
    }
    pub const fn eq(&self, other: &Self) -> bool {
        handle_nan!(false; self, other);
        (self.is_zero() && other.is_zero()) || BUint::eq(&self.to_bits(), &other.to_bits())
    }
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_int();
        let right = other.to_int();
        if left.is_negative() && right.is_negative() {
            Bint::cmp(&left, &right).reverse()
        } else {
            Bint::cmp(&left, &right)
        }
    }
    pub const fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        handle_nan!(None; self, other);
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        Some(self.total_cmp(other))
        /*let (self_neg, other_neg) = (self.is_sign_negative(), other.is_sign_negative());
        if self_neg && !other_neg {
            Some(Ordering::Less)
        } else if !self_neg && other_neg {
            Some(Ordering::Greater)
        } else {
            match BUint::cmp(&self.exponent(), &other.exponent()) {
                Ordering::Greater => {
                    if self_neg {
                        Some(Ordering::Greater)
                    } else {
                        Some(Ordering::Less)
                    }
                },
                Ordering::Less => {
                    if self_neg {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                },
                Ordering::Equal => {
                    match self.mantissa().cmp(&other.mantissa()) {
                        Ordering::Greater => {
                            if self_neg {
                                Some(Ordering::Greater)
                            } else {
                                Some(Ordering::Less)
                            }
                        },
                        Ordering::Less => {
                            if self_neg {
                                Some(Ordering::Less)
                            } else {
                                Some(Ordering::Greater)
                            }
                        },
                        Ordering::Equal => Some(Ordering::Equal),
                    }
                },
            }
        }*/
    }
    fn exp_mant(&self) -> (BUint<W>, BUint<W>) {
        let bits = self.uint;
        let exp = (bits << 1u8) >> (MANTISSA_BITS + 1);
        let mant = bits & Self::MANTISSA_MASK;
        
        if exp.is_zero() {
            (BUint::ONE, mant)
        } else {
            (exp, mant | (BUint::ONE << MANTISSA_BITS))
        }
    }
    #[inline]
    fn from_exp_mant(negative: bool, exp: BUint<W>, mant: BUint<W>) -> Self {
        let mut bits = (exp << MANTISSA_BITS) | mant;
        if negative {
            bits |= BUint::ONE.rotate_right(1);
        }
        Self::from_bits(bits)
    }
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
            return Self::INFINITY;
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

use core::cmp::{PartialEq, PartialOrd, Ordering};

impl<const W: usize, const MANTISSA_BITS: usize> PartialEq for Float<W, MANTISSA_BITS> {
    fn eq(&self, other: &Self) -> bool {
        Self::eq(&self, &other)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> PartialOrd for Float<W, MANTISSA_BITS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Self::partial_cmp(&self, &other)
    }
}

use core::ops::{Add, Sub, Mul, Div, Neg};

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

impl<const W: usize, const MANTISSA_BITS: usize> Neg for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::F64;
    type F32 = crate::Float::<4, 23>;

    #[test]
    fn test_constants() {
        macro_rules! test_constant {
            ($($constant: ident), *) => {
                $(
                    assert_eq!(F64::$constant.to_bits(), f64::$constant.to_bits().into(), "constant `{}` not equal to the primitive equivalent", stringify!($constant));
                )*
            }
        }
        test_constant!(NAN, INFINITY, NEG_INFINITY, MAX, MIN, MIN_POSITIVE);

        assert_eq!(F64::ZERO.to_bits(), 0.0f64.to_bits().into());
        assert_eq!(F64::NEG_ZERO.to_bits(), (-0.0f64).to_bits().into());
        assert_eq!(F64::ONE.to_bits(), 1.0f64.to_bits().into());
        assert_eq!(F64::NEG_ONE.to_bits(), (-1.0f64).to_bits().into());

        assert_eq!(F64::MAX_NEGATIVE.to_bits(), (-f64::MIN_POSITIVE).to_bits().into());

        assert_eq!(F64::MIN_EXP, f64::MIN_EXP.into());
        assert_eq!(F64::MAX_EXP, f64::MAX_EXP.into());

        assert_eq!(F64::RADIX, f64::RADIX);
        assert_eq!(F64::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_eq!(F64::DIGITS, f64::DIGITS);

        assert_eq!(F64::BITS, 64);
        assert_eq!(F64::EXPONENT_BITS, 11);
        assert_eq!(F64::EXP_BIAS, 1023i32.into());
        assert_eq!(F32::MAX_UNBIASED_EXP, 254u32.into());
    }

    #[test]
    fn test_add() {
        let nan = F32::NAN;
        let nan = F32::from_bits(nan.to_bits() | crate::BUint::ONE);
        assert_eq!(nan, nan + F32::ZERO);
        assert_eq!(F32::from_bits((-1f32).to_bits().into()) - F32::from_bits((-2f32).to_bits().into()), F32::from_bits((1f32).to_bits().into()));
        let (u1, u2) = (0xFFFFFFFFFFFF3, 0xFFFF2F3FFFFF3);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0x37FF013484968490u64, 0x35D0EE71100010FFu64);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0, 0);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFF, 0x10000000000000);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFF);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFF, 0x0000000000001);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());
    }

    #[test]
    fn test_sub() {
        assert_eq!((F32::ZERO - F32::NAN).to_bits(), (-F32::NAN).to_bits());
        //assert_eq!(F32::INFINITY - F32::INFINITY, F32::NAN);
        let (u1, u2) = (16777216, 16777216);
        let (f1, f2) = (f32::from_bits(u1), f32::from_bits(u2));
        let (float1, float2) = (F32::from_bits(u1.into()), F32::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFF3, 0xFFFF2F3FFFFF3);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0x37FF013484968490u64, 0x35D0EE71100010FFu64);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0, 0);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFF);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFF, 0x0000000000001);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFF, 0x10000000000000);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 - float2).to_bits(), (f1 - f2).to_bits().into());
    }

    use quickcheck::{TestResult, quickcheck};

    quickcheck! {
        fn test_quickcheck_add(a: f32, b: f32) -> TestResult {
            //if !a.is_normal() || !b.is_normal() {
                //return TestResult::discard();
            //}
            let c = F32::from_bits(crate::BUint::<4>::from(a.to_bits())) + F32::from_bits(crate::BUint::<4>::from(b.to_bits()));
            let d = a + b;
            //println!("e: {:032b}", d.to_bits());
            //println!("a: {:032b}", c.to_bits());
            TestResult::from_bool(c.to_bits() == d.to_bits().into())
        }
    }

    quickcheck! {
        fn test_quickcheck_sub(a: f32, b: f32) -> TestResult {
            let c = F32::from_bits(crate::BUint::<4>::from(a.to_bits())) - F32::from_bits(crate::BUint::<4>::from(b.to_bits()));
            let d = a - b;
            if a == 0.0 && b.is_nan() {
                return TestResult::discard();
            }
            TestResult::from_bool(c.to_bits() == d.to_bits().into())
        }
    }

    quickcheck! {
        fn test_quickcheck_mul(a: f32, b: f32) -> TestResult {
            let c = F32::from_bits(crate::BUint::<4>::from(a.to_bits())) * F32::from_bits(crate::BUint::<4>::from(b.to_bits()));
            let d = a * b;
            if a == 0.0 && b.is_nan() {
                return TestResult::discard();
            }
            TestResult::from_bool(c.to_bits() == d.to_bits().into())
        }
    }

    quickcheck! {
        fn test_quickcheck_div(a: f32, b: f32) -> TestResult {
            let c = F32::from_bits(crate::BUint::<4>::from(a.to_bits())) / F32::from_bits(crate::BUint::<4>::from(b.to_bits()));
            let d = a / b;
            TestResult::from_bool(c.to_bits() == d.to_bits().into())
        }
    }
    
    #[test]
    fn test_mul() {
        let (f1, f2) = (1.0f32, f32::NAN);
        let (u1, u2) = (f1.to_bits(), f2.to_bits());
        let (float1, float2) = (F32::from_bits(u1.into()), F32::from_bits(u2.into()));
        assert_eq!((float1 * float2).to_bits(), (f1 * f2).to_bits().into());
    }
    
    #[test]
    fn test_div() {
        let (f1, f2) = /*(-9.657683f32, -1146.6165f32);*/(f32::INFINITY, f32::INFINITY);
        let (u1, u2) = (f1.to_bits(), f2.to_bits());
        let (float1, float2) = (F32::from_bits(u1.into()), F32::from_bits(u2.into()));
        assert_eq!((float1 / float2).to_bits(), (f1 / f2).to_bits().into());
    }

    #[test]
    fn test_speed() {
        //01111111011111111111111111111111
        //01111111011111111111110010001010
        //panic!("{:b}", F32::from_bits(0b10000000000000001000000000000000u32.into()).exponent());
        for i in (0..i32::MAX as u32).rev() {
            let (float1, float2) = (F32::from_bits(i.into()), F32::from_bits(i.wrapping_add(885).into()));
            let (f1, f2) = (f32::from_bits(i), f32::from_bits(i.wrapping_add(885)));
            let s1 = (float2 + float1).to_bits();
            let s2 = (f2 + f1).to_bits();
            assert_eq!(s1, s2.into(), "{} (bin = {:032b})", i, i);
            //assert_eq!(i + 1, ((i as i128) + 1) as u128);
        }
    }
}