use super::{Float, FloatExponent, UnsignedFloatExponent};
use crate::integer::Uint;

mod e;
mod frac_1_pi;
mod frac_1_sqrt_2;
mod frac_2_pi;
mod frac_pi_2;
mod frac_pi_3;
mod frac_pi_4;
mod frac_pi_6;
mod frac_pi_8;
mod ln_10;
mod ln_2;
mod log10_2;
mod log10_e;
mod log2_10;
mod log2_e;
mod pi;
mod sqrt_2;
mod tau;

/// Associated constants.
impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = MB as u32 + 1;

    pub const DIGITS: u32 = Uint::<W>::ONE.wrapping_shl(Self::MB).ilog10() as u32;

    pub(crate) const MB_AS_FLOAT_EXP: FloatExponent = Self::MB as FloatExponent;

    pub const EPSILON: Self = Self::normal_power_of_two(-Self::MB_AS_FLOAT_EXP);

    pub(crate) const HALF_EPSILON: Self =
        Self::normal_power_of_two(-(Self::MB as FloatExponent + 1));

    pub const EXP_BIAS: FloatExponent = (1 << (Self::EXPONENT_BITS - 1)) - 1; // UnsignedFloatExponent::MAX.wrapping_shr(Self::MB + 1) as _;

    pub const MIN: Self = {
        let mut e = Uint::MAX;
        e = e.wrapping_shr(Self::MB + 1);
        e = e.wrapping_shl(Self::MB + 1);
        let mut m = Uint::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS + 1);
        Self::from_bits(e.bitor(m))
    };

    pub const MIN_POSITIVE: Self = Self::from_bits(Uint::ONE.wrapping_shl(Self::MB));

    pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();

    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: FloatExponent = -Self::EXP_BIAS + 2;

    pub(crate) const MIN_SUBNORMAL_EXP: FloatExponent =
        -Self::EXP_BIAS + 1 - Self::MB as FloatExponent; // TODO: need to check that this fits into FloatExponent

    pub const MAX_EXP: FloatExponent = Self::EXP_BIAS + 1;

    pub const MAX_UNBIASED_EXP: UnsignedFloatExponent =
        (Self::EXP_BIAS as UnsignedFloatExponent) * 2;

    pub const MIN_10_EXP: Self = todo!();

    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self =
        Self::from_bits(Uint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1));

    pub const MIN_SUBNORMAL: Self = Self::MAX_SUBNORMAL.neg();

    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(Uint::ONE);

    pub const MAX_NEGATIVE_SUBNORMAL: Self = Self::MIN_POSITIVE_SUBNORMAL.neg();

    pub const NAN: Self = {
        let mut u = Uint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB - 1);
        Self::from_bits(u)
    };

    // pub const QNAN: Self = {
    //     let bits = Self::NAN.to_bits();
    //     Self::from_bits(bits.bitor(Uint::ONE.shl(Self::MB - 1)))
    // };

    pub(crate) const NEG_NAN: Self = Self::NAN.neg();

    // pub const NEG_QNAN: Self = Self::QNAN.neg();

    pub const INFINITY: Self = {
        let mut u = Uint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const NEG_INFINITY: Self = {
        let mut u = Uint::MAX;
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub(crate) const ZERO: Self = Self::from_bits(Uint::ZERO);

    pub(crate) const NEG_ZERO: Self = Self::ZERO.neg();

    pub(crate) const ONE: Self = Self::normal_power_of_two(0);

    pub(crate) const TWO: Self = Self::normal_power_of_two(1);

    pub(crate) const HALF: Self = Self::normal_power_of_two(-1);

    pub(crate) const QUARTER: Self = Self::normal_power_of_two(-2);

    pub(crate) const NEG_ONE: Self = Self::ONE.neg();
}

/// Mathematical constants.
///
/// Each constant is computed at compile time to the full precision of `Self` and rounded
/// according to [`Self::ROUNDING_MODE`].
impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Euler's number ($e$), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(F64::E.to_bits().to_le_bytes(), core::f64::consts::E.to_bits().to_le_bytes());
    /// ```
    pub const E: Self = Self::e_internal();

    /// $\frac{1}{\pi}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_1_PI.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_1_PI.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_1_PI: Self = Self::frac_1_pi_internal();

    /// $\frac{1}{\sqrt{2}}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_1_SQRT_2.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_1_SQRT_2.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_1_SQRT_2: Self = Self::frac_1_sqrt_2_internal();

    /// $\frac{2}{\pi}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_2_PI.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_2_PI.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_2_PI: Self = Self::frac_2_pi_internal();

    /// $\frac{\pi}{2}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_PI_2.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_PI_2.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_PI_2: Self = Self::frac_pi_2_internal();

    /// $\frac{\pi}{3}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_PI_3.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_PI_3.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_PI_3: Self = Self::frac_pi_3_internal();

    /// $\frac{\pi}{4}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_PI_4.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_PI_4.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_PI_4: Self = Self::frac_pi_4_internal();

    /// $\frac{\pi}{6}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_PI_6.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_PI_6.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_PI_6: Self = Self::frac_pi_6_internal();

    /// $\frac{\pi}{8}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::FRAC_PI_8.to_bits().to_le_bytes(),
    ///     core::f64::consts::FRAC_PI_8.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const FRAC_PI_8: Self = Self::frac_pi_8_internal();

    /// $\ln(2)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LN_2.to_bits().to_le_bytes(),
    ///     core::f64::consts::LN_2.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LN_2: Self = Self::ln_2_internal();

    /// $\ln(10)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LN_10.to_bits().to_le_bytes(),
    ///     core::f64::consts::LN_10.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LN_10: Self = Self::ln_10_internal();

    /// $\log_2(10)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LOG2_10.to_bits().to_le_bytes(),
    ///     core::f64::consts::LOG2_10.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LOG2_10: Self = Self::log2_10_internal();

    /// $\log_2(e)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LOG2_E.to_bits().to_le_bytes(),
    ///     core::f64::consts::LOG2_E.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LOG2_E: Self = Self::log2_e_internal();

    /// $\log_{10}(2)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LOG10_2.to_bits().to_le_bytes(),
    ///     core::f64::consts::LOG10_2.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LOG10_2: Self = Self::log10_2_internal();

    /// $\log_{10}(e)$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::LOG10_E.to_bits().to_le_bytes(),
    ///     core::f64::consts::LOG10_E.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const LOG10_E: Self = Self::log10_e_internal();

    /// Archimedes' constant ($\pi$), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(F64::PI.to_bits().to_le_bytes(), core::f64::consts::PI.to_bits().to_le_bytes());
    /// ```
    pub const PI: Self = Self::pi_internal();

    /// $\sqrt{2}$, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::SQRT_2.to_bits().to_le_bytes(),
    ///     core::f64::consts::SQRT_2.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const SQRT_2: Self = Self::sqrt_2_internal();

    /// The full circle constant ($\tau = 2\pi$), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::F64;
    ///
    /// assert_eq!(
    ///     F64::TAU.to_bits().to_le_bytes(),
    ///     core::f64::consts::TAU.to_bits().to_le_bytes()
    /// );
    /// ```
    pub const TAU: Self = Self::tau_internal();
}

#[cfg(test)]
mod tests {
    use super::{Float, FloatExponent};
    use crate::test::TestConvert;
    use crate::types::{F16, F32, F64, F80, F128, F256};
    use crate::{Exponent, RoundingMode, Uint};

    macro_rules! test_constant {
        ($BaseType: ident :: $constant: ident == $value: expr) => {
            paste::paste! {
                #[test]
                fn [<test_ $BaseType:lower _constant_ $constant:lower>]() {
                    assert_eq!(TestConvert::into($BaseType::$constant), TestConvert::into($value), "constant `{}` not equal to the primitive equivalent", stringify!($constant));
                }
            }
        }
    }

    macro_rules! test_constants {
        {$($constant: ident), *} => {
            $(
                test_constant!(FTest::$constant == FTestBase::$constant);
            )*
        };
    }

    macro_rules! test_numeric_constants {
        [$(($constant: ident, $value: expr)), *] => {
            $(
                paste::paste! {
                    test_constant!(FTest::$constant == $value as FTestBase);
                }
            )*
        };
    }

    macro_rules! test_math_constants_for {
        ($FloatType:ident, $prim:ident; $($constant:ident),* $(,)?) => {
            paste::paste! {
                $(
                    #[test]
                    fn [<test_ $FloatType:lower _math_constant_ $constant:lower>]() {
                        const VAL: $FloatType = $FloatType::$constant;
                        assert_eq!(
                            TestConvert::into(VAL),
                            TestConvert::into(core::$prim::consts::$constant),
                            "math constant `{}::{}` does not match `core::{}::consts::{}`",
                            stringify!($FloatType),
                            stringify!($constant),
                            stringify!($prim),
                            stringify!($constant),
                        );
                    }
                )*
            }
        };
    }

    test_math_constants_for!(
        F32, f32;
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI,
        FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
        LN_2, LN_10, LOG2_10, LOG2_E, LOG10_2, LOG10_E,
        PI, SQRT_2, TAU,
    );

    test_math_constants_for!(
        F64, f64;
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI,
        FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
        LN_2, LN_10, LOG2_10, LOG2_E, LOG10_2, LOG10_E,
        PI, SQRT_2, TAU,
    );

    crate::test::test_all! {
        testing floats;

        test_constants! {
            /*NAN, */INFINITY, NEG_INFINITY, MAX, MIN, MIN_POSITIVE, EPSILON, /*MIN_EXP, MAX_EXP,*/ RADIX, MANTISSA_DIGITS, DIGITS
        }
        // don't test NAN as Rust f64/f32 NAN bit pattern not guaranteed to be stable across version

        // #[test]
        // fn nan_consts_is_nan() {
        //     assert!(FTest::NAN.is_nan());
        //     assert!(FTest::QNAN.is_nan());
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

        test_constant!(F64::BITS == 64 as Exponent);
        test_constant!(F32::BITS == 32 as Exponent);
        test_constant!(F64::EXPONENT_BITS == 11 as Exponent);
        test_constant!(F32::EXPONENT_BITS == 8 as Exponent);
        test_constant!(F64::EXP_BIAS == 1023 as FloatExponent);
        test_constant!(F32::EXP_BIAS == 127 as FloatExponent);
        test_constant!(F64::MAX_UNBIASED_EXP == 2046 as Exponent);
        test_constant!(F32::MAX_UNBIASED_EXP == 254 as Exponent);
    }

    fn hex_to_be_bytes<const W: usize>(hex: &str) -> [u8; W] {
        assert_eq!(hex.len(), W * 2);
        let raw = hex.as_bytes();
        let mut out = [0u8; W];
        for i in 0..W {
            let hi = match raw[2 * i] {
                b'0'..=b'9' => raw[2 * i] - b'0',
                b'a'..=b'f' => raw[2 * i] - b'a' + 10,
                b'A'..=b'F' => raw[2 * i] - b'A' + 10,
                _ => panic!("invalid hex digit"),
            };
            let lo = match raw[2 * i + 1] {
                b'0'..=b'9' => raw[2 * i + 1] - b'0',
                b'a'..=b'f' => raw[2 * i + 1] - b'a' + 10,
                b'A'..=b'F' => raw[2 * i + 1] - b'A' + 10,
                _ => panic!("invalid hex digit"),
            };
            out[i] = (hi << 4) | lo;
        }
        out
    }

    macro_rules! check_all_18_oracle {
        ($F:ty, $W:expr, $expected:expr) => {{
            const ACTUAL: [(&str, $F); 18] = [
                ("E", <$F>::E),
                ("FRAC_1_PI", <$F>::FRAC_1_PI),
                ("FRAC_1_SQRT_2", <$F>::FRAC_1_SQRT_2),
                ("FRAC_2_PI", <$F>::FRAC_2_PI),
                ("FRAC_PI_2", <$F>::FRAC_PI_2),
                ("FRAC_PI_3", <$F>::FRAC_PI_3),
                ("FRAC_PI_4", <$F>::FRAC_PI_4),
                ("FRAC_PI_6", <$F>::FRAC_PI_6),
                ("FRAC_PI_8", <$F>::FRAC_PI_8),
                ("LN_2", <$F>::LN_2),
                ("LN_10", <$F>::LN_10),
                ("LOG2_10", <$F>::LOG2_10),
                ("LOG2_E", <$F>::LOG2_E),
                ("LOG10_2", <$F>::LOG10_2),
                ("LOG10_E", <$F>::LOG10_E),
                ("PI", <$F>::PI),
                ("SQRT_2", <$F>::SQRT_2),
                ("TAU", <$F>::TAU),
            ];
            let exp: [(&str, &str); 18] = $expected;
            for i in 0..18 {
                assert_eq!(ACTUAL[i].0, exp[i].0);
                assert_eq!(
                    ACTUAL[i].1.to_be_bytes(),
                    hex_to_be_bytes::<$W>(exp[i].1),
                    "{}::{} bit mismatch",
                    stringify!($F),
                    ACTUAL[i].0,
                );
            }
        }};
    }

    #[test]
    fn math_constants_exact_oracle_diverse_formats() {
        type TinyExp = Float<2, 13>;
        type HugeExp = Float<16, 3>;
        type F512 = Float<64, 488>;
        type F192 = Float<192, 1500>;

        check_all_18_oracle!(
            F16,
            2,
            [
                ("E", "4170"),
                ("FRAC_1_PI", "3518"),
                ("FRAC_1_SQRT_2", "39a8"),
                ("FRAC_2_PI", "3918"),
                ("FRAC_PI_2", "3e48"),
                ("FRAC_PI_3", "3c30"),
                ("FRAC_PI_4", "3a48"),
                ("FRAC_PI_6", "3830"),
                ("FRAC_PI_8", "3648"),
                ("LN_2", "398c"),
                ("LN_10", "409b"),
                ("LOG2_10", "42a5"),
                ("LOG2_E", "3dc5"),
                ("LOG10_2", "34d1"),
                ("LOG10_E", "36f3"),
                ("PI", "4248"),
                ("SQRT_2", "3da8"),
                ("TAU", "4648"),
            ]
        );

        // EXPONENT_BITS == 2: constants < 1.0 are subnormal; TAU overflows to +inf.
        check_all_18_oracle!(
            TinyExp,
            2,
            [
                ("E", "4b7e"),
                ("FRAC_1_PI", "0a30"),
                ("FRAC_1_SQRT_2", "16a1"),
                ("FRAC_2_PI", "145f"),
                ("FRAC_PI_2", "3244"),
                ("FRAC_PI_3", "2183"),
                ("FRAC_PI_4", "1922"),
                ("FRAC_PI_6", "10c1"),
                ("FRAC_PI_8", "0c91"),
                ("LN_2", "162e"),
                ("LN_10", "44d7"),
                ("LOG2_10", "5527"),
                ("LOG2_E", "2e2b"),
                ("LOG10_2", "09a2"),
                ("LOG10_E", "0de6"),
                ("PI", "5244"),
                ("SQRT_2", "2d41"),
                ("TAU", "6000"),
            ]
        );
        assert!(TinyExp::FRAC_PI_4.is_subnormal());
        assert!(TinyExp::TAU.is_infinite());

        // EXPONENT_BITS == 124 (>= 32): verifies no dependency on i32 EXP_BIAS.
        check_all_18_oracle!(
            HugeExp,
            16,
            [
                ("E", "40000000000000000000000000000003"),
                ("FRAC_1_PI", "3fffffffffffffffffffffffffffffea"),
                ("FRAC_1_SQRT_2", "3ffffffffffffffffffffffffffffff3"),
                ("FRAC_2_PI", "3ffffffffffffffffffffffffffffff2"),
                ("FRAC_PI_2", "3ffffffffffffffffffffffffffffffd"),
                ("FRAC_PI_3", "3ffffffffffffffffffffffffffffff8"),
                ("FRAC_PI_4", "3ffffffffffffffffffffffffffffff5"),
                ("FRAC_PI_6", "3ffffffffffffffffffffffffffffff0"),
                ("FRAC_PI_8", "3fffffffffffffffffffffffffffffed"),
                ("LN_2", "3ffffffffffffffffffffffffffffff3"),
                ("LN_10", "40000000000000000000000000000001"),
                ("LOG2_10", "40000000000000000000000000000005"),
                ("LOG2_E", "3ffffffffffffffffffffffffffffffc"),
                ("LOG10_2", "3fffffffffffffffffffffffffffffea"),
                ("LOG10_E", "3fffffffffffffffffffffffffffffee"),
                ("PI", "40000000000000000000000000000005"),
                ("SQRT_2", "3ffffffffffffffffffffffffffffffb"),
                ("TAU", "4000000000000000000000000000000d"),
            ]
        );

        check_all_18_oracle!(
            F80,
            10,
            [
                ("E", "40005bf0a8b145769535"),
                ("FRAC_1_PI", "3ffd45f306dc9c882a54"),
                ("FRAC_1_SQRT_2", "3ffe6a09e667f3bcc909"),
                ("FRAC_2_PI", "3ffe45f306dc9c882a54"),
                ("FRAC_PI_2", "3fff921fb54442d1846a"),
                ("FRAC_PI_3", "3fff0c152382d7365846"),
                ("FRAC_PI_4", "3ffe921fb54442d1846a"),
                ("FRAC_PI_6", "3ffe0c152382d7365846"),
                ("FRAC_PI_8", "3ffd921fb54442d1846a"),
                ("LN_2", "3ffe62e42fefa39ef358"),
                ("LN_10", "400026bb1bbb5551582e"),
                ("LOG2_10", "4000a934f0979a3715fd"),
                ("LOG2_E", "3fff71547652b82fe177"),
                ("LOG10_2", "3ffd34413509f79fef31"),
                ("LOG10_E", "3ffdbcb7b1526e50e32a"),
                ("PI", "4000921fb54442d1846a"),
                ("SQRT_2", "3fff6a09e667f3bcc909"),
                ("TAU", "4001921fb54442d1846a"),
            ]
        );

        check_all_18_oracle!(
            F128,
            16,
            [
                ("E", "40005bf0a8b1457695355fb8ac404e7a"),
                ("FRAC_1_PI", "3ffd45f306dc9c882a53f84eafa3ea6a"),
                ("FRAC_1_SQRT_2", "3ffe6a09e667f3bcc908b2fb1366ea95"),
                ("FRAC_2_PI", "3ffe45f306dc9c882a53f84eafa3ea6a"),
                ("FRAC_PI_2", "3fff921fb54442d18469898cc51701b8"),
                ("FRAC_PI_3", "3fff0c152382d73658465bb32e0f567b"),
                ("FRAC_PI_4", "3ffe921fb54442d18469898cc51701b8"),
                ("FRAC_PI_6", "3ffe0c152382d73658465bb32e0f567b"),
                ("FRAC_PI_8", "3ffd921fb54442d18469898cc51701b8"),
                ("LN_2", "3ffe62e42fefa39ef35793c7673007e6"),
                ("LN_10", "400026bb1bbb5551582dd4adac5705a6"),
                ("LOG2_10", "4000a934f0979a3715fc9257edfe9b60"),
                ("LOG2_E", "3fff71547652b82fe1777d0ffda0d23a"),
                ("LOG10_2", "3ffd34413509f79fef311f12b35816f9"),
                ("LOG10_E", "3ffdbcb7b1526e50e32a6ab7555f5a68"),
                ("PI", "4000921fb54442d18469898cc51701b8"),
                ("SQRT_2", "3fff6a09e667f3bcc908b2fb1366ea95"),
                ("TAU", "4001921fb54442d18469898cc51701b8"),
            ]
        );

        check_all_18_oracle!(
            F256,
            32,
            [
                ("E", "400005bf0a8b1457695355fb8ac404e7a79e3b1738b079c5a6d2b53c26c8228d"),
                ("FRAC_1_PI", "3fffd45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd7"),
                ("FRAC_1_SQRT_2", "3fffe6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b066"),
                ("FRAC_2_PI", "3fffe45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd7"),
                ("FRAC_PI_2", "3ffff921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c762"),
                ("FRAC_PI_3", "3ffff0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f97"),
                ("FRAC_PI_4", "3fffe921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c762"),
                ("FRAC_PI_6", "3fffe0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f97"),
                ("FRAC_PI_8", "3fffd921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c762"),
                ("LN_2", "3fffe62e42fefa39ef35793c7673007e5ed5e81e6864ce5316c5b141a2eb7175"),
                ("LN_10", "4000026bb1bbb5551582dd4adac5705a61451c51fd9f3b4bbf21d078c3d0403e"),
                ("LOG2_10", "40000a934f0979a3715fc9257edfe9b5fb699b2d8abfc6f675a9d236d590105d"),
                ("LOG2_E", "3ffff71547652b82fe1777d0ffda0d23a7d11d6aef551bad2b4b1164a2cd9a34"),
                ("LOG10_2", "3fffd34413509f79fef311f12b35816f922f04d5a618a87a3e69314bcde4d6fa"),
                ("LOG10_E", "3fffdbcb7b1526e50e32a6ab7555f5a67b8647dc68c048b934404747e5a89ef2"),
                ("PI", "40000921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c762"),
                ("SQRT_2", "3ffff6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b066"),
                ("TAU", "40001921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c762"),
            ]
        );

        // F512 (MB = 488 -> stage1_limbs = 10 > 8): exercises dynamic arbitrary-precision paths!
        check_all_18_oracle!(
            F512,
            64,
            [
                ("E", "4000005bf0a8b1457695355fb8ac404e7a79e3b1738b079c5a6d2b53c26c8228c867f799273b9c49367df2fa5fc6c6c618ebb1ed0364055d88c2f5a7be3dabac"),
                ("FRAC_1_PI", "3ffffd45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd778ac36e48dc74849ba5c00c925dd413a32439fc3bd63962534e7dd1046bea5d"),
                ("FRAC_1_SQRT_2", "3ffffe6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b0667322a95f90608757145875163fcdfb907b6721ee950bc8738f694f0090e6c7c"),
                ("FRAC_2_PI", "3ffffe45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd778ac36e48dc74849ba5c00c925dd413a32439fc3bd63962534e7dd1046bea5d"),
                ("FRAC_PI_2", "3fffff921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a33679a748636605614dbe4be286e9fc26adadb"),
                ("FRAC_PI_3", "3fffff0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f96f7983170d60a212f0067d3f71779a66f85979958eb892987ec59f152c473c92"),
                ("FRAC_PI_4", "3ffffe921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a33679a748636605614dbe4be286e9fc26adadb"),
                ("FRAC_PI_6", "3ffffe0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f96f7983170d60a212f0067d3f71779a66f85979958eb892987ec59f152c473c92"),
                ("FRAC_PI_8", "3ffffd921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a33679a748636605614dbe4be286e9fc26adadb"),
                ("LN_2", "3ffffe62e42fefa39ef35793c7673007e5ed5e81e6864ce5316c5b141a2eb71755f457cf70ec40dbd75930ab2aa5f695f43621da5d5c6b827042884eae765223"),
                ("LN_10", "40000026bb1bbb5551582dd4adac5705a61451c51fd9f3b4bbf21d078c3d0403e05ae52c5e05af635020b998e1978058be1ad058c448308217c5b5f71ef10806"),
                ("LOG2_10", "400000a934f0979a3715fc9257edfe9b5fb699b2d8abfc6f675a9d236d590105cf0b3a0cca1fbd3baa3e747c497d56c75a17ac286bdc900acb94a99fe2c59a9f"),
                ("LOG2_E", "3fffff71547652b82fe1777d0ffda0d23a7d11d6aef551bad2b4b1164a2cd9a342648fbc3887eeaa2ed9ac49b25eeb82d7c167d52173cc1895213f897f5e06a8"),
                ("LOG10_2", "3ffffd34413509f79fef311f12b35816f922f04d5a618a87a3e69314bcde4d6f98c79650d45b03233f57a13cb978e668e37a25228e692b6317243cb16a4fea4b"),
                ("LOG10_E", "3ffffdbcb7b1526e50e32a6ab7555f5a67b8647dc68c048b934404747e5a89ef1d4a78ea849df42805e7e52446ab258c8c942a319c77b3fa71b978df4571a592"),
                ("PI", "400000921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a33679a748636605614dbe4be286e9fc26adadb"),
                ("SQRT_2", "3fffff6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b0667322a95f90608757145875163fcdfb907b6721ee950bc8738f694f0090e6c7c"),
                ("TAU", "400001921fb54442d18469898cc51701b839a252049c1114cf98e804177d4c76273644a29410f31c6809bbdf2a33679a748636605614dbe4be286e9fc26adadb"),
            ]
        );

        // W = 192, MB = 1500 -> stage1_limbs = 25 > 22: exercises dynamic 2/pi fallback!
        const F192_FRAC_1_PI: F192 = F192::FRAC_1_PI;
        const F192_FRAC_2_PI: F192 = F192::FRAC_2_PI;
        assert_eq!(
            F192_FRAC_1_PI.to_be_bytes(),
            hex_to_be_bytes::<192>(
                "3fffffffd45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd778ac36e48dc74849ba5c00c925dd413a32439fc3bd63962534e7dd1046bea5d768909d338e04d68befc827323ac7306a673e93908bf177bf250763ff12fffbc0b301fde5e2316b414da3eda6cfd9e4f96136e9e8c7ecd3cbfd45aea4f758fd7cbe2f67a0e73ef14a525d4d7f6bf623f1aba10ac06608df8f6d757e19f784135e86c3b53c722c2bdcc3610cb330abe2940d0811bffb1009ae64e620c0c2ab"
            )
        );
        assert_eq!(
            F192_FRAC_2_PI.to_be_bytes(),
            hex_to_be_bytes::<192>(
                "3fffffffe45f306dc9c882a53f84eafa3ea69bb81b6c52b3278872083fca2c757bd778ac36e48dc74849ba5c00c925dd413a32439fc3bd63962534e7dd1046bea5d768909d338e04d68befc827323ac7306a673e93908bf177bf250763ff12fffbc0b301fde5e2316b414da3eda6cfd9e4f96136e9e8c7ecd3cbfd45aea4f758fd7cbe2f67a0e73ef14a525d4d7f6bf623f1aba10ac06608df8f6d757e19f784135e86c3b53c722c2bdcc3610cb330abe2940d0811bffb1009ae64e620c0c2ab"
            )
        );
    }

    #[test]
    fn math_constants_all_rounding_modes() {
        const R_EVEN: u8 = RoundingMode::TiesToEven as u8;
        const R_AWAY: u8 = RoundingMode::TiesToAway as u8;
        const R_POS: u8 = RoundingMode::TowardPositive as u8;
        const R_NEG: u8 = RoundingMode::TowardNegative as u8;
        const R_ZERO: u8 = RoundingMode::TowardZero as u8;
        const R_AFZ: u8 = RoundingMode::AwayFromZero as u8;
        const R_ODD: u8 = RoundingMode::ToOdd as u8;
        const R_TPOS: u8 = RoundingMode::TiesTowardPositive as u8;
        const R_TNEG: u8 = RoundingMode::TiesTowardNegative as u8;
        const R_TZER: u8 = RoundingMode::TiesTowardZero as u8;
        const R_TODD: u8 = RoundingMode::TiesToOdd as u8;
        const R_FAITH: u8 = RoundingMode::Faithful as u8;
        const R_STOCH: u8 = RoundingMode::Stochastic as u8;

        macro_rules! check_rm_constant {
            ($W:expr, $MB:expr, $C:ident) => {{
                let c_even = Float::<$W, $MB, R_EVEN>::$C.to_bits();
                let c_away = Float::<$W, $MB, R_AWAY>::$C.to_bits();
                let c_pos = Float::<$W, $MB, R_POS>::$C.to_bits();
                let c_neg = Float::<$W, $MB, R_NEG>::$C.to_bits();
                let c_zero = Float::<$W, $MB, R_ZERO>::$C.to_bits();
                let c_afz = Float::<$W, $MB, R_AFZ>::$C.to_bits();
                let c_odd = Float::<$W, $MB, R_ODD>::$C.to_bits();
                let c_tpos = Float::<$W, $MB, R_TPOS>::$C.to_bits();
                let c_tneg = Float::<$W, $MB, R_TNEG>::$C.to_bits();
                let c_tzer = Float::<$W, $MB, R_TZER>::$C.to_bits();
                let c_todd = Float::<$W, $MB, R_TODD>::$C.to_bits();
                let c_faith = Float::<$W, $MB, R_FAITH>::$C.to_bits();
                let c_stoch = Float::<$W, $MB, R_STOCH>::$C.to_bits();

                // All 18 constants are strictly positive irrationals, so floor + 1 == ceil.
                assert_eq!(c_neg, c_zero, "TowardNegative != TowardZero for {}", stringify!($C));
                assert_eq!(c_pos, c_afz, "TowardPositive != AwayFromZero for {}", stringify!($C));
                assert_eq!(c_neg + Uint::ONE, c_pos, "floor + 1 != ceil for {}", stringify!($C));

                // All 6 round-to-nearest modes agree because no irrational constant is a tie.
                assert!(c_even == c_neg || c_even == c_pos);
                assert_eq!(c_away, c_even);
                assert_eq!(c_tpos, c_even);
                assert_eq!(c_tneg, c_even);
                assert_eq!(c_tzer, c_even);
                assert_eq!(c_todd, c_even);

                // ToOdd selects whichever of {floor, ceil} has an odd LSB.
                let expected_odd = if c_neg.is_odd() { c_neg } else { c_pos };
                assert_eq!(c_odd, expected_odd, "ToOdd mismatch for {}", stringify!($C));

                // Faithful and Stochastic bracket within {floor, ceil}.
                assert!(c_faith == c_neg || c_faith == c_pos);
                assert!(c_stoch == c_neg || c_stoch == c_pos);
            }};
        }

        macro_rules! check_all_18_rm {
            ($W:expr, $MB:expr) => {{
                check_rm_constant!($W, $MB, E);
                check_rm_constant!($W, $MB, FRAC_1_PI);
                check_rm_constant!($W, $MB, FRAC_1_SQRT_2);
                check_rm_constant!($W, $MB, FRAC_2_PI);
                check_rm_constant!($W, $MB, FRAC_PI_2);
                check_rm_constant!($W, $MB, FRAC_PI_3);
                check_rm_constant!($W, $MB, FRAC_PI_4);
                check_rm_constant!($W, $MB, FRAC_PI_6);
                check_rm_constant!($W, $MB, FRAC_PI_8);
                check_rm_constant!($W, $MB, LN_2);
                check_rm_constant!($W, $MB, LN_10);
                check_rm_constant!($W, $MB, LOG2_10);
                check_rm_constant!($W, $MB, LOG2_E);
                check_rm_constant!($W, $MB, LOG10_2);
                check_rm_constant!($W, $MB, LOG10_E);
                check_rm_constant!($W, $MB, PI);
                check_rm_constant!($W, $MB, SQRT_2);
                check_rm_constant!($W, $MB, TAU);
            }};
        }

        check_all_18_rm!(2, 10); // F16
        check_all_18_rm!(8, 52); // F64
        check_all_18_rm!(16, 112); // F128

        // Overflowing TAU in Float<2, 13> respects directed rounding modes!
        assert_eq!(Float::<2, 13, R_EVEN>::TAU.to_bits(), Float::<2, 13, R_EVEN>::INFINITY.to_bits());
        assert_eq!(Float::<2, 13, R_POS>::TAU.to_bits(), Float::<2, 13, R_POS>::INFINITY.to_bits());
        assert_eq!(Float::<2, 13, R_NEG>::TAU.to_bits(), Float::<2, 13, R_NEG>::MAX.to_bits());
        assert_eq!(Float::<2, 13, R_ZERO>::TAU.to_bits(), Float::<2, 13, R_ZERO>::MAX.to_bits());
    }

    #[test]
    fn math_constants_match_transcendental_functions() {
        macro_rules! check_identities {
            ($F:ty) => {{
                let one = <$F>::one_val();
                let neg_one = <$F>::neg_one_val();
                let two = <$F>::two_val();
                let half = <$F>::half_val();
                let ten = <$F>::from_str("10.0").unwrap();

                assert_eq!(<$F>::E.to_bits(), one.exp().to_bits());
                assert_eq!(<$F>::LN_2.to_bits(), two.ln().to_bits());
                assert_eq!(<$F>::LN_10.to_bits(), ten.ln().to_bits());
                assert_eq!(<$F>::LOG2_10.to_bits(), ten.log2().to_bits());
                assert_eq!(<$F>::LOG10_2.to_bits(), two.log10().to_bits());
                assert_eq!(<$F>::SQRT_2.to_bits(), two.sqrt().to_bits());
                assert_eq!(<$F>::FRAC_1_SQRT_2.to_bits(), half.sqrt().to_bits());
                assert_eq!(<$F>::PI.to_bits(), neg_one.acos().to_bits());
                assert_eq!(<$F>::FRAC_PI_2.to_bits(), one.asin().to_bits());
                assert_eq!(<$F>::FRAC_PI_3.to_bits(), half.acos().to_bits());
                assert_eq!(<$F>::FRAC_PI_4.to_bits(), one.atan().to_bits());
                assert_eq!(<$F>::FRAC_PI_6.to_bits(), half.asin().to_bits());
            }};
        }

        check_identities!(F16);
        check_identities!(F32);
        check_identities!(F64);
        check_identities!(F80);
        check_identities!(F128);
        check_identities!(F256);
        check_identities!(Float<64, 488>);
    }
}
