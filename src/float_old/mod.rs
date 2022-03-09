use crate::uint::BUint;
use crate::int::Bint;
use crate::digit::{Digit, self};
use crate::ExpType;
use core::num::FpCategory;

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

mod classify;
mod cmp;
mod consts;
mod convert;
mod math;
mod ops;

#[derive(Clone, Copy, Debug)]
pub struct Float<const W: usize, const MB: usize> {
    uint: BUint<W>,
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    const BITS: usize = W * digit::BITS as usize;
    
    const EXPONENT_BITS: usize = Self::BITS - MB - 1;

    const MANTISSA_WORDS: (usize, usize) = (MB / digit::BITS as usize, MB % digit::BITS as usize);

    const EXPONENT_MASK: BUint<W> = BUint::MAX.wrapping_shl(MB as ExpType) ^ Bint::MIN.to_bits();
}

impl<const W: usize, const MB: usize> Float<W, MB> {
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
    const fn exponent(self) -> Bint<W> {
        let u: BUint<W> = (self.to_bits() & Bint::MAX.to_bits()).wrapping_shr(MB);
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
            self.mantissa() | (BUint::ONE.wrapping_shl(MB))
        }
    }
    #[inline(always)]
    const fn to_int(self) -> Bint<W> {
        Bint::from_bits(self.to_bits())
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

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    fn exp_mant(&self) -> (BUint<W>, BUint<W>) {
        let bits = self.uint;
        let exp = (bits << 1u8) >> (MB + 1);
        let mant = bits & Self::MANTISSA_MASK;
        
        if exp.is_zero() {
            (BUint::ONE, mant)
        } else {
            (exp, mant | (BUint::ONE << MB))
        }
    }
    #[inline]
    fn from_exp_mant(negative: bool, exp: BUint<W>, mant: BUint<W>) -> Self {
        let mut bits = (exp << MB) | mant;
        if negative {
            bits |= BUint::ONE.rotate_right(1);
        }
        Self::from_bits(bits)
    }
}

impl From<f64> for crate::F64 {
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}