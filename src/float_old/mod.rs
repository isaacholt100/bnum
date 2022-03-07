use crate::uint::BUint;
use crate::int::Bint;
use crate::digit::{Digit, self};
use crate::ExpType;
use core::num::FpCategory;

mod consts;
mod ops;

#[allow(unused)]
macro_rules! test_float {
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?
    } => {
        crate::test::test_big_num! {
            big: crate::F64,
            primitive: f64,
            function: $name,
            $(cases: [
                $(($($arg), *) ), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: Into::into
        }
    };
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?,
        converter: $converter: expr
    } => {
        crate::test::test_big_num! {
            big: crate::F64,
            primitive: f64,
            function: $name,
            $(cases: [
                $(($($arg), *)), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: $converter
        }
    };
}

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
            -self
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

impl From<f64> for crate::F64 {
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
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

    /*#[test]
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
    }*/

    /*quickcheck! {
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
    }*/


    /*crate::test::test_trait! {
        big: F64,
        primitive: f64,
        test_name: add,
        function: <Add>::add,
        quickcheck: (a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan(),
        converter: converter
    }*/

    /*crate::test::test_trait! {
        big: F64,
        primitive: f64,
        test_name: add,
        function: <Add>::add,
        quickcheck: (a: f64, b: f64),
        quickcheck_skip: b.is_nan() || a.is_nan(),
        converter: converter
    }*/
    
    /*#[test]
    fn test_mul() {
        let a = <f64>::add(1.0f64.into(), 2.0f64);
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
    }*/
}