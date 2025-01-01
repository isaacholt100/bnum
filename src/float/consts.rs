use super::{Float, FloatExponent, UnsignedFloatExponent};
use crate::buint::BUintD8;
use crate::doc;

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

#[doc = doc::consts::impl_desc!()]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::consts::RADIX!(F)]
    pub const RADIX: u32 = 2;

    #[doc = doc::consts::MANTISSA_DIGITS!(F)]
    pub const MANTISSA_DIGITS: u32 = MB as u32 + 1;

    #[doc = doc::consts::DIGITS!(F)]
    pub const DIGITS: u32 = BUintD8::<W>::ONE.wrapping_shl(Self::MB).ilog10() as u32;

    pub(crate) const MB_AS_FLOAT_EXP: FloatExponent = Self::MB as FloatExponent;

    #[doc = doc::consts::EPSILON!(F)]
    pub const EPSILON: Self = Self::normal_power_of_two(-Self::MB_AS_FLOAT_EXP);

    pub(crate) const HALF_EPSILON: Self = Self::normal_power_of_two(-(Self::MB as FloatExponent + 1));

    pub const EXP_BIAS: FloatExponent = (1 << (Self::EXPONENT_BITS - 1)) - 1;// UnsignedFloatExponent::MAX.wrapping_shr(Self::MB + 1) as _;

    #[doc = doc::consts::MIN!(F)]
    pub const MIN: Self = {
        let mut e = BUintD8::MAX;
        e = e.wrapping_shr(Self::MB + 1);
        e = e.wrapping_shl(Self::MB + 1);
        let mut m = BUintD8::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS + 1);
        Self::from_bits(e.bitor(m))
    };

    #[doc = doc::consts::MIN_POSITIVE!(F)]
    pub const MIN_POSITIVE: Self = Self::from_bits(BUintD8::ONE.wrapping_shl(Self::MB));

    pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();

    #[doc = doc::consts::MAX!(F)]
    pub const MAX: Self = Self::MIN.abs();


    #[doc = doc::consts::MIN_EXP!(F)]
    pub const MIN_EXP: FloatExponent = -Self::EXP_BIAS + 2;

    pub(crate) const MIN_SUBNORMAL_EXP: FloatExponent = -Self::EXP_BIAS + 1 - Self::MB as FloatExponent; // TODO: need to check that this fits into FloatExponent

    #[doc = doc::consts::MAX_EXP!(F)]
    pub const MAX_EXP: FloatExponent = Self::EXP_BIAS + 1;

    pub const MAX_UNBIASED_EXP: UnsignedFloatExponent = (Self::EXP_BIAS as UnsignedFloatExponent) * 2;

    pub const MIN_10_EXP: Self = todo!();

    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self = Self::from_bits(BUintD8::MAX.wrapping_shr(Self::EXPONENT_BITS + 1));

    pub const MIN_SUBNORMAL: Self = Self::MAX_SUBNORMAL.neg();

    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(BUintD8::ONE);

    pub const MAX_NEGATIVE_SUBNORMAL: Self = Self::MIN_POSITIVE_SUBNORMAL.neg();

    #[doc = doc::consts::NAN!(F)]
    pub const NAN: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB - 1);
        Self::from_bits(u)
    };

    // pub const QNAN: Self = {
    //     let bits = Self::NAN.to_bits();
    //     Self::from_bits(bits.bitor(BUintD8::ONE.shl(Self::MB - 1)))
    // };

    pub const NEG_NAN: Self = Self::NAN.neg();

    // pub const NEG_QNAN: Self = Self::QNAN.neg();

    #[doc = doc::consts::INFINITY!(F)]
    pub const INFINITY: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    #[doc = doc::consts::NEG_INFINITY!(F)]
    pub const NEG_INFINITY: Self = {
        let mut u = BUintD8::MAX;
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const ZERO: Self = Self::from_bits(BUintD8::ZERO);

    pub const NEG_ZERO: Self = Self::ZERO.neg();

    pub const ONE: Self = Self::normal_power_of_two(0);

    pub const TWO: Self = Self::normal_power_of_two(1);

    pub const HALF: Self = Self::normal_power_of_two(-1);

    pub const QUARTER: Self = Self::normal_power_of_two(-2);

    pub const NEG_ONE: Self = Self::ONE.neg();
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

    // #[test]
    // fn nan_consts_is_nan() {
    //     assert!(FTEST::NAN.is_nan());
    //     assert!(FTEST::QNAN.is_nan());
    // }

    test_numeric_constants![
        (ZERO, 0.0),
        (NEG_ZERO, -0.0),
        (ONE, 1.0),
        (QUARTER, 0.25),
        (HALF, 0.5),
        (NEG_ONE, -1),
        (TWO, 2)
    ];

    
    test_constant!(F64::BITS == 64 as ExpType);
    test_constant!(F32::BITS == 32 as ExpType);
    test_constant!(F64::EXPONENT_BITS == 11 as ExpType);
    test_constant!(F32::EXPONENT_BITS == 8 as ExpType);
    test_constant!(F64::EXP_BIAS == 1023i128);
    test_constant!(F32::EXP_BIAS == 127i128);
    test_constant!(F64::MAX_UNBIASED_EXP == 2046u128);
    test_constant!(F32::MAX_UNBIASED_EXP == 254u128);
}
