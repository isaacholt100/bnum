use super::Float;
use crate::buint::BUintD8;
use crate::bint::BIntD8;
use crate::cast::As;

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = MB as u32 + 1;

    pub const DIGITS: u32 = BUintD8::<W>::ONE.wrapping_shl(Self::MB).ilog10() as u32;

    pub const EPSILON: Self = {
        let u = Self::EXP_BIAS.to_bits() - MB.as_::<BUintD8<W>>();
        Self::from_bits(u << Self::MB)
    };

    pub const EXP_BIAS: BIntD8<W> = BIntD8::MAX.wrapping_shr(Self::MB + 1);

    pub const MIN: Self = {
        let mut e = BUintD8::MAX;
        e = e.wrapping_shr(Self::MB + 1);
        e = e.wrapping_shl(Self::MB + 1);
        let mut m = BUintD8::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS + 1);
        Self::from_bits(e | m)
    };

    pub const MIN_POSITIVE: Self = {
        Self::from_bits(BUintD8::ONE.wrapping_shl(Self::MB))
    };
    pub const MAX_NEGATIVE: Self = -Self::MIN_POSITIVE;
    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: BIntD8<W> = (-Self::EXP_BIAS).wrapping_add(BIntD8::ONE.wrapping_shl(1));
    pub const MAX_EXP: BIntD8<W> = Self::EXP_BIAS.wrapping_add(BIntD8::ONE);
    pub const MAX_UNBIASED_EXP: BUintD8<W> = Self::EXP_BIAS.to_bits() * BUintD8::TWO;
    pub const MIN_10_EXP: Self = todo!();
    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self = Self::from_bits(BUintD8::MAX.wrapping_shr(Self::EXPONENT_BITS + 1));
    pub const MIN_SUBNORMAL: Self = -Self::MAX_SUBNORMAL;
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(BUintD8::ONE);
    pub const MAX_NEGATIVE_SUBNORMAL: Self = -Self::MIN_POSITIVE_SUBNORMAL;

    pub const NAN: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB - 1);
        Self::from_bits(u)
    };

    pub const QNAN: Self = {
        let bits = Self::NAN.to_bits();
        Self::from_bits(bits | (BUintD8::ONE << (Self::MB - 1)))
    };

    pub const NEG_NAN: Self = -Self::NAN;

    pub const NEG_QNAN: Self = -Self::QNAN;

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
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp + BUintD8::ONE, BUintD8::ZERO)
    };

    pub const HALF: Self = {
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp - BUintD8::ONE, BUintD8::ZERO)
    };

    pub const QUARTER: Self = {
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp - BUintD8::TWO, BUintD8::ZERO)
    };
    
    pub const NEG_ONE: Self = Self::from_bits(Self::ONE.bits | Self::NEG_ZERO.bits);
}

#[cfg(test)]
mod tests {
    use super::super::{F64, F32};
    use crate::test::TestConvert;
    use crate::test::types::{ftest, FTEST};
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
        /*NAN, */INFINITY, NEG_INFINITY, MAX, MIN, MIN_POSITIVE, EPSILON, MIN_EXP, MAX_EXP, RADIX, MANTISSA_DIGITS, DIGITS
    }
    // don't test NAN as Rust f64/f32 NAN bit pattern not guaranteed to be stable across version

    #[test]
    fn nan_consts_is_nan() {
        assert!(FTEST::NAN.is_nan());
        assert!(FTEST::QNAN.is_nan());
    }    

    test_numeric_constants![
        (ZERO, 0.0), (NEG_ZERO, -0.0), (ONE, 1.0), (QUARTER, 0.25), (HALF, 0.5), (NEG_ONE, -1)
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