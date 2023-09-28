use super::Float;
use crate::bint::BIntD8;
use crate::buint::BUintD8;

const fn buint_from_usize<const N: usize>(u: usize) -> BUintD8<N> {
    const UINT_BITS: usize = <usize>::BITS as usize;
    let mut out = BUintD8::ZERO;
    let mut i = 0;
    while i << crate::digit::u8::BIT_SHIFT < UINT_BITS {
        let d = (u >> (i << crate::digit::u8::BIT_SHIFT)) as u8;
        if d != 0 {
            out.digits[i] = d;
        }
        i += 1;
    }
    out
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = MB as u32 + 1;

    pub const DIGITS: u32 = BUintD8::<W>::ONE.wrapping_shl(Self::MB).ilog10() as u32;

    pub const EPSILON: Self = {
        let u = Self::EXP_BIAS.to_bits().sub(buint_from_usize::<W>(MB));
        Self::from_bits(u.shl(Self::MB))
    };

    pub const EXP_BIAS: BIntD8<W> = BIntD8::MAX.wrapping_shr(Self::MB + 1);

    pub const MIN: Self = {
        let mut e = BUintD8::MAX;
        e = e.wrapping_shr(Self::MB + 1);
        e = e.wrapping_shl(Self::MB + 1);
        let mut m = BUintD8::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS + 1);
        Self::from_bits(e.bitor(m))
    };

    pub const MIN_POSITIVE: Self = { Self::from_bits(BUintD8::ONE.wrapping_shl(Self::MB)) };
    pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();
    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: BIntD8<W> = (Self::EXP_BIAS.neg()).wrapping_add(BIntD8::ONE.wrapping_shl(1));
    pub const MAX_EXP: BIntD8<W> = Self::EXP_BIAS.wrapping_add(BIntD8::ONE);
    pub const MAX_UNBIASED_EXP: BUintD8<W> = Self::EXP_BIAS.to_bits().shl(1); // mul by 2
    pub const MIN_10_EXP: Self = todo!();
    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self =
        Self::from_bits(BUintD8::MAX.wrapping_shr(Self::EXPONENT_BITS + 1));
    pub const MIN_SUBNORMAL: Self = Self::MAX_SUBNORMAL.neg();
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(BUintD8::ONE);
    pub const MAX_NEGATIVE_SUBNORMAL: Self = Self::MIN_POSITIVE_SUBNORMAL.neg();

    pub const NAN: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB - 1);
        Self::from_bits(u)
    };

    pub const QNAN: Self = {
        let bits = Self::NAN.to_bits();
        Self::from_bits(bits.bitor(BUintD8::ONE.shl(Self::MB - 1)))
    };

    pub const NEG_NAN: Self = Self::NAN.neg();

    pub const NEG_QNAN: Self = Self::QNAN.neg();

    pub const INFINITY: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const NEG_INFINITY: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const ZERO: Self = Self::from_bits(BUintD8::ZERO);

    pub const NEG_ZERO: Self = Self::from_words(BIntD8::<W>::MIN.bits.digits);

    pub const ONE: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(2);
        u = u.wrapping_shr(2 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const TWO: Self = {
        let (_, exp, _) = Self::ONE.to_raw_parts();
        Self::from_exp_mant(false, exp.add(BUintD8::ONE), BUintD8::ZERO)
    };

    pub const HALF: Self = {
        let (_, exp, _) = Self::ONE.to_raw_parts();
        Self::from_exp_mant(false, exp.sub(BUintD8::ONE), BUintD8::ZERO)
    };

    pub const QUARTER: Self = {
        let (_, exp, _) = Self::ONE.to_raw_parts();
        Self::from_exp_mant(false, exp.sub(BUintD8::TWO), BUintD8::ZERO)
    };

    pub const NEG_ONE: Self = Self::from_bits(Self::ONE.bits.bitor(Self::NEG_ZERO.bits));
}

#[cfg(test)]
mod tests {
    use super::super::{F32, F64};
    use crate::test::types::{ftest, FTEST};
    use crate::test::TestConvert;
    use crate::ExpType;

    macro_rules! test_constant {
        ($big: ident :: $constant: ident == $primitive: expr) => {
            paste::paste! {
                #[test]
                fn [<test_ $big:lower _constant_ $constant:lower>]() {
                    assert_eq!(TestConvert::into($big::$constant), TestConvert::into($primitive), "constant `{}` not equal to the primitive equivalent", stringify!($constant));
                }
            }
        }
    }

    macro_rules! test_constants {
        {$($constant: ident), *} => {
            $(
                test_constant!(FTEST::$constant == ftest::$constant);
            )*
        };
    }

    macro_rules! test_numeric_constants {
        [$(($constant: ident, $value: expr)), *] => {
            $(
                paste::paste! {
                    test_constant!(FTEST::$constant == $value as ftest);
                }
            )*
        };
    }

    test_constants! {
        /*NAN, */INFINITY, NEG_INFINITY, MAX, MIN, MIN_POSITIVE, EPSILON, /*MIN_EXP, MAX_EXP,*/ RADIX, MANTISSA_DIGITS, DIGITS
    }
    // don't test NAN as Rust f64/f32 NAN bit pattern not guaranteed to be stable across version

    #[test]
    fn nan_consts_is_nan() {
        assert!(FTEST::NAN.is_nan());
        assert!(FTEST::QNAN.is_nan());
    }

    test_numeric_constants![
        (ZERO, 0.0),
        (NEG_ZERO, -0.0),
        (ONE, 1.0),
        (QUARTER, 0.25),
        (HALF, 0.5),
        (NEG_ONE, -1)
    ];

    test_constant!(F64::BITS == 64 as ExpType);
    test_constant!(F32::BITS == 32 as ExpType);
    test_constant!(F64::EXPONENT_BITS == 11 as ExpType);
    test_constant!(F32::EXPONENT_BITS == 8 as ExpType);
    test_constant!(F64::EXP_BIAS == 1023i64);
    test_constant!(F32::EXP_BIAS == 127i32);
    test_constant!(F64::MAX_UNBIASED_EXP == 2046u64);
    test_constant!(F32::MAX_UNBIASED_EXP == 254u32);
}
