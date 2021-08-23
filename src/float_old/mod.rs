use crate::uint::BUint;
use crate::iint::BIint;
use crate::digit::{SignedDigit, Digit, self};
use crate::ExpType;
use core::num::FpCategory;

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
    const EXPONENT_BITS: usize = Self::BITS - MANTISSA_BITS;
    const MANTISSA_WORDS: (usize, usize) = (MANTISSA_BITS / digit::BITS as usize, MANTISSA_BITS % digit::BITS as usize);
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
            let mut digits = *self.words();
            digits[0] |= 1 << (Digit::BITS - 1);
            Self::from_words(digits)
        } else {
            self
        }
    }
    const fn exponent(self) -> BIint<W> {
        let u: BUint<W> = self.to_bits().wrapping_shl(1).wrapping_shr(MANTISSA_BITS + 1);
        BIint::from_bits(u)
    }
    const fn mantissa(self) -> BUint<W> {
        self.to_bits().wrapping_shl(Self::EXPONENT_BITS + 1).wrapping_shr(Self::EXPONENT_BITS + 1)
    }
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<W> {
        self.uint
    }
    #[inline(always)]
    const fn to_int(self) -> BIint<W> {
        BIint::from_bits(self.to_bits())
    }
    #[inline(always)]
    pub const fn from_bits(v: BUint<W>) -> Self {
        Self {
            uint: v,
        }
    }
    pub const fn is_finite(self) -> bool {
        self.exponent().trailing_ones() as usize != Self::EXPONENT_BITS
    }
    pub const fn is_nan(self) -> bool {
        !(self.mantissa().is_zero() || self.is_finite())
    }
    pub const fn is_subnormal(self) -> bool {
        self.exponent().is_zero() && !self.is_zero()
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
        Self::from_bits(BUint::from_digits(self_words).bitor(BUint::from_digits(sign_words)))
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
            BIint::cmp(&left, &right).reverse()
        } else {
            BIint::cmp(&left, &right)
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

use core::ops::{Add, Neg};

impl<const W: usize, const MANTISSA_BITS: usize> Add for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (a, b) = if self.exponent() > rhs.exponent() {
            (self.exponent(), rhs.exponent())
        } else {
            (rhs.exponent(), self.exponent())
        };
        todo!()
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Neg for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    pub const RADIX: u32 = 2;
    pub const MANTISSA_DIGITS: u32 = MANTISSA_BITS as u32;
    pub const DIGITS: u32 = BUint::<W>::ONE.wrapping_shl(MANTISSA_BITS as ExpType).log10() as u32;
    pub const EPSILON: Self = todo!();
    pub const EXP_BIAS: BIint<W> = BIint::MAX.wrapping_shr(MANTISSA_BITS + 1);

    pub const MIN: Self = {
        let mut e = BUint::MAX;
        e = e.wrapping_shr(MANTISSA_BITS as ExpType + 1);
        e = e.wrapping_shl(MANTISSA_BITS as ExpType + 1);
        let mut m = BUint::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS as ExpType + 1);
        Self {
            uint: e.bitor(m),
        }
    };
    pub const MIN_POSITIVE: Self = {
        Self {
            uint: BUint::ONE.wrapping_shl(MANTISSA_BITS as ExpType),
        }
    };
    pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();
    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: BIint<W> = Self::EXP_BIAS.neg().wrapping_add(BIint::ONE.wrapping_shl(1));
    pub const MAX_EXP: BIint<W> = Self::EXP_BIAS.wrapping_add(BIint::ONE);
    pub const MIN_10_EXP: Self = todo!();
    pub const MAX_10_EXP: Self = todo!();

    pub const NAN: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType - 1);
        Self {
            uint: u,
        }
    };
    pub const INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self {
            uint: u,
        }
    };
    pub const NEG_INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shr(MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self {
            uint: u,
        }
    };
    pub const ZERO: Self = Self::from_bits(BUint::ZERO);
    pub const NEG_ZERO: Self = Self::from_words(*BIint::<W>::MIN.digits());
    pub const ONE: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(2);
        u = u.wrapping_shr(2 + MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self::from_bits(u)
    };
    pub const NEG_ONE: Self = Self::from_bits(Self::ONE.uint.bitor(Self::NEG_ZERO.uint));
}