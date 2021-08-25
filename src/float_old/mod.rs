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
    pub const fn normalize(self) -> Self {
        if self.is_subnormal() {
            todo!()
        } else {
            self
        }
    }
    pub const fn from_parts(sign: bool, exponent: BIint<W>, mantissa: BUint<W>) -> Self {
        let sign = if sign {
            Self::NEG_ZERO.to_bits()
        } else {
            Self::ZERO.to_bits()
        };
        let exp = exponent.to_bits().wrapping_shl(MANTISSA_BITS as ExpType);
        let u = sign.bitor(exp).bitor(mantissa);
        Self::from_bits(u)
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
    const fn exponent(self) -> BIint<W> {
        let u: BUint<W> = self.to_bits().bitand(BIint::MAX.to_bits()).wrapping_shr(MANTISSA_BITS);
        BIint::from_bits(u)
    }
    const MANTISSA_MASK: BUint<W> = BUint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);
    const fn mantissa(self) -> BUint<W> {
        self.to_bits().bitand(Self::MANTISSA_MASK)
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
        handle_nan!(Self::NAN; self, rhs);
        if self == Self::INFINITY {
            if rhs == Self::NEG_INFINITY {
                return Self::NAN;
            }
            return self;
        } else if self == Self::NEG_INFINITY {
            if rhs == Self::INFINITY {
                return Self::NAN;
            }
            return self;
        } else if rhs == Self::INFINITY {
            if self == Self::NEG_INFINITY {
                return Self::NAN;
            }
            return rhs;
        } else if rhs == Self::NEG_INFINITY {
            if self == Self::INFINITY {
                return Self::NAN;
            }
            return rhs;
        }
        let self_e = self.exponent();
        let rhs_e = rhs.exponent();
        let exp_diff = self_e - rhs_e;
        let (mut a, mut b, mut exponent) = if exp_diff.is_negative() {
            (rhs, self, rhs_e)
        } else {
            (self, rhs, self_e)
        };
        let am = if a.is_normal() {
            a.mantissa() | (BUint::ONE << MANTISSA_BITS)
        } else {
            a.mantissa()
        };
        let bm = if b.is_normal() {
            (b.mantissa() | (BUint::ONE << MANTISSA_BITS)) >> exp_diff.abs()
        } else {
            b.mantissa().checked_shr(exp_diff.abs().as_usize()).unwrap_or(BUint::ZERO)
        };
        println!("{}", exp_diff);
        let mut mantissa = am + bm;
        println!("{:?}", mantissa);
        //mantissa = mantissa ^ (BUint::ONE << (mantissa.bits() - 1));
        if mantissa.leading_zeros() == (Self::BITS - MANTISSA_BITS - 2) as ExpType {
            exponent = exponent.wrapping_add(BIint::ONE);
            if exponent.trailing_ones() == Self::EXPONENT_BITS as ExpType {
                return Self::INFINITY;
            }
            if mantissa.digits()[0] & 1 == 1 {
                mantissa += BUint::ONE;
            }
            mantissa >>= 1;
        }
        if !exponent.is_zero() {
            mantissa = mantissa & !(BUint::ONE << MANTISSA_BITS);
        }
        Self::from_parts(a.is_sign_negative(), exponent, mantissa)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Neg for Float<W, MANTISSA_BITS> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {
    const BITS: usize = W * digit::BITS as usize;
    const EXPONENT_BITS: usize = Self::BITS - MANTISSA_BITS - 1;
    const MANTISSA_WORDS: (usize, usize) = (MANTISSA_BITS / digit::BITS as usize, MANTISSA_BITS % digit::BITS as usize);

    pub const RADIX: u32 = 2;
    pub const MANTISSA_DIGITS: u32 = MANTISSA_BITS as u32 + 1;
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

    pub const MAX_SUBNORMAL: Self = Self {
        uint: BUint::MAX.wrapping_shr(Self::EXPONENT_BITS as ExpType + 1),
    };
    pub const MIN_SUBNORMAL: Self = Self::MAX_SUBNORMAL.neg();
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self {
        uint: BUint::ONE,
    };
    pub const MAX_NEGATIVE_SUBNORMAL: Self = Self::MIN_POSITIVE_SUBNORMAL.neg();

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
    }

    #[test]
    fn test_add() {
        let (u1, u2) = (0x37FF013484968490u64, 0x35D0EE71100010FFu64);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFFF, 0x10000000000000);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0, 0);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());

        let (u1, u2) = (0xFFFFFFFFFFFF3, 0xFFFF2F3FFFFF3);
        let (f1, f2) = (f64::from_bits(u1), f64::from_bits(u2));
        let (float1, float2) = (F64::from_bits(u1.into()), F64::from_bits(u2.into()));
        assert_eq!((float1 + float2).to_bits(), (f1 + f2).to_bits().into());
    }

    #[test]
    fn test_speed() {
        //panic!("{:b}", F32::from_bits(0b10000000000000001000000000000000u32.into()).exponent());
        for i in 0..i32::MAX as u32 {
            let float = F32::from_bits(i.into());
            let f = f32::from_bits(i);
            let s1 = (float + float).to_bits();
            let s2 = (f + f).to_bits();
            assert_eq!(s1.as_u32(), s2, "expected: {:032b} got: {:032b} at number {} (float = {})", s2, s1, i, f);
            //assert_eq!(i + 1, ((i as i128) + 1) as u128);
        }
    }
}