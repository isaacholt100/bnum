use crate::ExpType;
use crate::uint::BUint;
use crate::int::Bint;
use crate::digit::{Digit, self};
//use crate::ExpType;
use crate::As;

#[allow(unused)]
macro_rules! test_float {
    {
        function: $name: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
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
            ])?
            ,quickcheck: ($($param : $(ref $re)? $ty), *)
            $(,quickcheck_skip: $skip)?
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
    const MB: ExpType = MB as _;
    const BITS: ExpType = BUint::<W>::BITS;
    
    const EXPONENT_BITS: ExpType = Self::BITS - Self::MB - 1;

    /*const MANTISSA_WORDS: (usize, usize) = (MB / digit::BITS as usize, MB % digit::BITS as usize);

    const EXPONENT_MASK: BUint<W> = BUint::MAX.wrapping_shl(Self::MB) ^ Bint::MIN.to_bits();*/
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
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
        let u: BUint<W> = (self.to_bits() & Bint::MAX.to_bits()).wrapping_shr(Self::MB);
        Bint::from_bits(u)
    }
    
    /*const fn actual_exponent(self) -> Bint<W> {
        self.exponent() - Self::EXP_BIAS
    }
    const fn unshifted_exponent(self) -> Bint<W> {
        Bint::from_bits(self.to_bits() & Self::EXPONENT_MASK)
    }*/
    const MANTISSA_MASK: BUint<W> = BUint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);
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

    #[inline]
    pub const fn copysign(self, sign: Self) -> Self {
        let mut self_words = *self.words();
        if sign.is_sign_negative() {
            self_words[W - 1] |= 1 << (digit::BITS - 1);
        } else {
            self_words[W - 1] &= (!0) >> 1;
        }
        Self::from_bits(BUint::from_digits(self_words))
    }

    #[inline]
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }
}

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    const fn exp_mant(&self) -> (BUint<W>, BUint<W>) {
        let bits = self.bits;
        let exp = (bits << 1u8) >> (Self::MB + 1);
        let mant = bits & Self::MANTISSA_MASK;
        
        if exp.is_zero() {
            (BUint::ONE, mant)
        } else {
            (exp, mant | (BUint::ONE << Self::MB))
        }
    }

    #[inline]
    pub(super) const fn decode(self) -> (BUint<W>, Bint<W>) {
        let bits = self.bits;
        let exp = (bits << 1u8) >> (Self::MB + 1);
        let mant = if exp.is_zero() {
            (bits & Self::MANTISSA_MASK) << 1 as ExpType
        } else {
            (bits & Self::MANTISSA_MASK) | (BUint::power_of_two(MB as ExpType))
        };
        let exp = Bint::from_bits(exp) - Self::EXP_BIAS + MB.as_::<Bint<W>>();
        (mant, exp)
    }

    #[inline]
    const fn from_exp_mant(negative: bool, exp: BUint<W>, mant: BUint<W>) -> Self {
        let mut bits = (exp << Self::MB) | mant;
        if negative {
            bits = bits | Bint::MIN.to_bits();
        }
        let f = Self::from_bits(bits);
        if negative {
            assert!(f.is_sign_negative());
        }
        f
    }
}

#[cfg(test)]
impl From<f64> for crate::F64 {
    #[inline]
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

#[cfg(test)]
mod tests {
    test_float! {
        function: copysign(f1: f64, f2: f64)
    }

    test_float! {
        function: signum(f: f64)
    }

    #[test]
    fn test_from_exp_mant() {
        let f = crate::F64::from_exp_mant(true, crate::BUint::ZERO, crate::BUint::ZERO);
        assert!(f.is_sign_negative());
    }
}