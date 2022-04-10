use crate::ExpType;
use crate::uint::BUint;
use crate::int::Bint;
use crate::digit::{Digit, self};
//use crate::ExpType;

#[allow(unused)]
macro_rules! test_float {
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?
        $(,big_converter: $big_converter: expr)?
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
            $(big_converter: $big_converter,)?
            converter: Into::into
        }
    };
    {
        function: $name: ident ($($param: ident : $ty: ty), *),
        $(cases: [
            $(($($arg: expr), *)), *
        ],)?
        $(quickcheck_skip: $skip: expr,)?
        $(big_converter: $big_converter: expr,)?
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
            $(big_converter: $big_converter,)?
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

mod cast;
mod classify;
mod cmp;
mod consts;
mod convert;
mod math;
mod ops;

#[cfg(feature = "serde_all")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_all", derive(Serialize, Deserialize))]
pub struct Float<const W: usize, const MB: usize> {
    bits: BUint<W>,
}

// TODO: implement rand traits

impl<const W: usize, const MB: usize> Float<W, MB> {
    const BITS: usize = W * digit::BITS as usize;
    
    const EXPONENT_BITS: usize = Self::BITS - MB - 1;

    /*const MANTISSA_WORDS: (usize, usize) = (MB / digit::BITS as usize, MB % digit::BITS as usize);

    const EXPONENT_MASK: BUint<W> = BUint::MAX.wrapping_shl(MB as ExpType) ^ Bint::MIN.to_bits();*/
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const fn from_parts(negative: bool, exponent: BUint<W>, mantissa: BUint<W>) -> Self {
        let mut words = *(exponent | mantissa).digits();
        if negative {
            words[W - 1] |= 1 << (digit::BITS - 1);
        }
        Self::from_words(words)
    }
    #[inline(always)]
    const fn from_words(words: [Digit; W]) -> Self {
        Self::from_bits(BUint::from_digits(words))
    }
    #[inline(always)]
    const fn words(&self) -> &[Digit; W] {
        self.bits.digits()
    }

    #[inline]
    const fn exponent(self) -> Bint<W> {
        let u: BUint<W> = (self.to_bits() & Bint::MAX.to_bits()).wrapping_shr(MB as ExpType);
        Bint::from_bits(u)
    }
    
    /*const fn actual_exponent(self) -> Bint<W> {
        self.exponent() - Self::EXP_BIAS
    }
    const fn unshifted_exponent(self) -> Bint<W> {
        Bint::from_bits(self.to_bits() & Self::EXPONENT_MASK)
    }*/
    const MANTISSA_MASK: BUint<W> = BUint::MAX.wrapping_shr(Self::EXPONENT_BITS as ExpType + 1);
    /*const fn mantissa(self) -> BUint<W> {
        self.to_bits() & Self::MANTISSA_MASK
    }
    const fn actual_mantissa(self) -> BUint<W> {
        if self.is_subnormal() {
            self.mantissa()
        } else {
            self.mantissa() | (BUint::ONE.wrapping_shl(MB))
        }
    }*/
    #[inline(always)]
    const fn to_int(self) -> Bint<W> {
        Bint::from_bits(self.to_bits())
    }
    pub const fn copysign(self, sign: Self) -> Self {
        let mut self_words = *self.words();
        if sign.is_sign_negative() {
            self_words[W - 1] |= 1 << (digit::BITS - 1);
        } else {
            self_words[W - 1] &= (!0) >> 1;
        }
        Self::from_bits(BUint::from_digits(self_words))
    }
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }
}

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    const fn exp_mant(&self) -> (BUint<W>, BUint<W>) {
        let bits = self.bits;
        let exp = (bits << 1u8) >> (MB + 1);
        let mant = bits & Self::MANTISSA_MASK;
        
        if exp.is_zero() {
            (BUint::ONE, mant)
        } else {
            (exp, mant | (BUint::ONE << MB))
        }
    }
    #[inline]
    const fn from_exp_mant(negative: bool, exp: BUint<W>, mant: BUint<W>) -> Self {
        let mut bits = (exp << MB) | mant;
        if negative {
            bits |= Bint::MIN.to_bits();
        }
        let f = Self::from_bits(bits);
        if negative {
            assert!(f.is_sign_negative());
        }
        f
    }
}

impl From<f64> for crate::F64 {
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

#[cfg(test)]
mod tests {
    fn to_u64_bits(f: crate::F64) -> u64 {
        f.to_bits().as_u64()
    }

    test_float! {
        function: copysign(f1: f64, f2: f64),
        big_converter: to_u64_bits,
        converter: f64::to_bits
    }

    test_float! {
        function: signum(f: f64),
        big_converter: to_u64_bits,
        converter: f64::to_bits
    }

    #[test]
    fn test_from_exp_mant() {
        let f = crate::F64::from_exp_mant(true, crate::BUint::ZERO, crate::BUint::ZERO);
        assert!(f.is_sign_negative());
    }
}