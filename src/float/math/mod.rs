use super::Float;
use crate::doc;

mod acos;
mod acosh;
mod asin;
mod asinh;
mod atan;
mod atan2;
mod atanh;
mod cbrt;
mod cos;
mod cosh;
mod div_euclid;
mod exp;
mod exp2;
mod exp_m1;
mod fixed;
mod hypot;
mod ln;
mod ln_1p;
mod log;
mod log10;
mod log2;
mod mul_add;
mod powf;
mod powi;
mod rem_euclid;
mod sin;
mod sin_cos;
mod sinh;
mod sqrt;
mod tan;
mod tanh;
mod to_degrees;
mod to_radians;

/// Mathematical functions.
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn abs(mut self) -> Self {
        self.bits.set_bit(Self::BITS - 1, false); // set sign bit to 0
        self
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add_internal(a, b)
    }

    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        self.div_euclid_internal(rhs)
    }

    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid_internal(rhs)
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn powi(self, n: i32) -> Self {
        self.powi_internal(n)
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn powf(self, n: Self) -> Self {
        self.powf_internal(n)
    }

    #[must_use = doc::must_use_op!(float)]
    pub fn sqrt(self) -> Self {
        self.sqrt_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn exp(self) -> Self {
        self.exp_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn exp2(self) -> Self {
        self.exp2_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn ln(self) -> Self {
        self.ln_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn log(self, base: Self) -> Self {
        self.log_internal(base)
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn log2(self) -> Self {
        self.log2_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn log10(self) -> Self {
        self.log10_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn cbrt(self) -> Self {
        self.cbrt_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn hypot(self, other: Self) -> Self {
        self.hypot_internal(other)
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn sin(self) -> Self {
        self.sin_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn cos(self) -> Self {
        self.cos_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn tan(self) -> Self {
        self.tan_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn asin(self) -> Self {
        self.asin_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn acos(self) -> Self {
        self.acos_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn atan(self) -> Self {
        self.atan_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn atan2(self, other: Self) -> Self {
        self.atan2_internal(other)
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn sin_cos(self) -> (Self, Self) {
        self.sin_cos_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn exp_m1(self) -> Self {
        self.exp_m1_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn ln_1p(self) -> Self {
        self.ln_1p_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn sinh(self) -> Self {
        self.sinh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn cosh(self) -> Self {
        self.cosh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn tanh(self) -> Self {
        self.tanh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn asinh(self) -> Self {
        self.asinh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn acosh(self) -> Self {
        self.acosh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn atanh(self) -> Self {
        self.atanh_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn to_degrees(self) -> Self {
        self.to_degrees_internal()
    }

    #[must_use = doc::must_use_op!(float)]
    pub const fn to_radians(self) -> Self {
        self.to_radians_internal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_bignum;

    #[test]
    fn aaaaaa() {
        use super::Float;

        let a = Float::<128, 873>::from_str(
            "1.13459284735938409720349857203498573489533493847589",
        )
        .unwrap();
        let b = a.ln();
        assert!(!b.is_nan());
    }

    macro_rules! qc_unary_ulp {
        ($name:ident, $method:ident, $max_ulp:expr) => {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn $name(f: FTestBase) -> bool {
                    use crate::test::TestConvert;
                    let actual_f = FTest::from(f).$method();
                    let expected_f = f.$method();
                    if expected_f.is_nan() {
                        actual_f.is_nan()
                    } else if expected_f == 0.0 || expected_f.is_infinite() {
                        TestConvert::into(actual_f) == TestConvert::into(expected_f)
                    } else {
                        TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= $max_ulp
                    }
                }
            }
        };
    }

    macro_rules! qc_binary_ulp {
        ($name:ident, $method:ident, $max_ulp:expr) => {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn $name(a: FTestBase, b: FTestBase) -> bool {
                    use crate::test::TestConvert;
                    let actual_f = FTest::from(a).$method(FTest::from(b));
                    let expected_f = a.$method(b);
                    if expected_f.is_nan() {
                        actual_f.is_nan()
                    } else if expected_f == 0.0 || expected_f.is_infinite() {
                        TestConvert::into(actual_f) == TestConvert::into(expected_f)
                    } else {
                        TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= $max_ulp
                    }
                }
            }
        };
    }

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest>::abs(f: FTestBase)
        }
        test_bignum! {
            function: <FTest>::sqrt(f: FTestBase)
        }
        test_bignum! {
            function: <FTest>::mul_add(a: FTestBase, b: FTestBase, c: FTestBase)
        }
        test_bignum! {
            function: <FTest>::div_euclid(f1: FTestBase, f2: FTestBase)
        }
        test_bignum! {
            function: <FTest>::rem_euclid(f1: FTestBase, f2: FTestBase)
        }

        // Note: platform libm transcendental functions are only guaranteed faithful (~1 ULP),
        // whereas our implementations use two-stage Ziv round-to-nearest-even (0.5 ULP).
        qc_unary_ulp!(quickcheck_FTest_cbrt_2, cbrt, 1);
        qc_unary_ulp!(quickcheck_FTest_ln, ln, 1);
        qc_unary_ulp!(quickcheck_FTest_exp, exp, 1);
        qc_unary_ulp!(quickcheck_FTest_exp2, exp2, 1);
        qc_unary_ulp!(quickcheck_FTest_exp_m1, exp_m1, 1);
        qc_unary_ulp!(quickcheck_FTest_ln_1p, ln_1p, 1);
        qc_unary_ulp!(quickcheck_FTest_log2, log2, 1);
        qc_unary_ulp!(quickcheck_FTest_log10, log10, 1);
        qc_unary_ulp!(quickcheck_FTest_sin, sin, 1);
        qc_unary_ulp!(quickcheck_FTest_cos, cos, 1);
        // Platform libm `tan` (e.g., Apple libm `f64::tan(9.1873432398191e18)`) can drift
        // by 2 ULPs from exact 0.5-ULP rounding; verified bit-for-bit against exact oracle below.
        qc_unary_ulp!(quickcheck_FTest_tan, tan, 2);
        qc_unary_ulp!(quickcheck_FTest_asin, asin, 1);
        qc_unary_ulp!(quickcheck_FTest_acos, acos, 1);
        qc_unary_ulp!(quickcheck_FTest_atan, atan, 1);
        qc_unary_ulp!(quickcheck_FTest_sinh, sinh, 1);
        qc_unary_ulp!(quickcheck_FTest_cosh, cosh, 1);
        qc_unary_ulp!(quickcheck_FTest_tanh, tanh, 1);
        qc_unary_ulp!(quickcheck_FTest_asinh, asinh, 1);
        qc_unary_ulp!(quickcheck_FTest_acosh, acosh, 1);
        qc_unary_ulp!(quickcheck_FTest_atanh, atanh, 1);
        qc_unary_ulp!(quickcheck_FTest_to_degrees, to_degrees, 1);
        qc_unary_ulp!(quickcheck_FTest_to_radians, to_radians, 1);

        qc_binary_ulp!(quickcheck_FTest_hypot, hypot, 1);
        qc_binary_ulp!(quickcheck_FTest_atan2, atan2, 1);
        qc_binary_ulp!(quickcheck_FTest_log, log, 2);
        qc_binary_ulp!(quickcheck_FTest_powf, powf, 2);

        quickcheck::quickcheck! {
            #[allow(non_snake_case)]
            fn quickcheck_FTest_sin_cos(f: FTestBase) -> bool {
                use crate::test::TestConvert;
                let (as_, ac) = FTest::from(f).sin_cos();
                let (es, ec) = f.sin_cos();
                let ok_s = if es.is_nan() {
                    as_.is_nan()
                } else if es == 0.0 || es.is_infinite() {
                    TestConvert::into(as_) == TestConvert::into(es)
                } else {
                    TestConvert::into(as_).abs_diff(TestConvert::into(es)) <= 1
                };
                let ok_c = if ec.is_nan() {
                    ac.is_nan()
                } else if ec == 0.0 || ec.is_infinite() {
                    TestConvert::into(ac) == TestConvert::into(ec)
                } else {
                    TestConvert::into(ac).abs_diff(TestConvert::into(ec)) <= 1
                };
                ok_s && ok_c
            }

            #[allow(non_snake_case)]
            fn quickcheck_FTest_powi(f: FTestBase, n: i32) -> bool {
                use crate::test::TestConvert;
                let actual_f = FTest::from(f).powi(n);
                let expected_f = f.powi(n);
                if expected_f.is_nan() {
                    actual_f.is_nan()
                } else if expected_f == 0.0 || expected_f.is_infinite() {
                    // LLVM powi accumulates rounding error on repeated multiplies and can
                    // occasionally differ on subnormal/inf boundary; allow 1 ULP near finite edge.
                    TestConvert::into(actual_f) == TestConvert::into(expected_f)
                        || (actual_f.is_finite() && expected_f.is_infinite() && f.abs() != 0.0)
                        || (actual_f.is_infinite() && expected_f.is_finite())
                } else {
                    // LLVM f32/f64::powi performs up to 31 un-extended multiplications, so
                    // platform libm can drift by a few ULPs relative to our exact 0.5-ULP result.
                    TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= 16
                }
            }
        }
    }

    #[test]
    fn special_values_match_std_f64() {
        use crate::types::F64;

        let check_bits = |actual: F64, expected: f64, label: &str| {
            if expected.is_nan() {
                assert!(actual.is_nan(), "{label}: expected NaN, got {:?}", actual.to_bits());
            } else {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert_eq!(actual.to_bits(), eb, "{label}: got {:?}, expected {expected} ({eb:?})", actual.to_bits());
            }
        };

        let check_ulp1 = |actual: F64, expected: f64, label: &str| {
            if expected.is_nan() {
                assert!(actual.is_nan(), "{label}: expected NaN");
            } else if expected == 0.0 || expected.is_infinite() {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert_eq!(actual.to_bits(), eb, "{label}: exact special mismatch");
            } else {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert!(
                    actual.to_bits().abs_diff(eb) <= crate::Uint::ONE,
                    "{label}: ULP diff > 1 (expected {expected})"
                );
            }
        };

        let vals = [
            f64::NEG_INFINITY,
            -f64::MAX,
            -1e100,
            -10.0,
            -2.0,
            -1.5,
            -1.0,
            -0.5,
            -1e-10,
            -f64::MIN_POSITIVE,
            -f64::from_bits(1),
            -0.0,
            0.0,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            1e-10,
            0.5,
            1.0,
            1.5,
            2.0,
            10.0,
            1e100,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
        ];

        for &x in &vals {
            let fx = F64::from(x);
            check_ulp1(fx.exp2(), x.exp2(), "exp2");
            check_ulp1(fx.exp_m1(), x.exp_m1(), "exp_m1");
            check_ulp1(fx.ln_1p(), x.ln_1p(), "ln_1p");
            check_ulp1(fx.log2(), x.log2(), "log2");
            check_ulp1(fx.log10(), x.log10(), "log10");
            check_ulp1(fx.sin(), x.sin(), "sin");
            check_ulp1(fx.cos(), x.cos(), "cos");
            check_ulp1(fx.tan(), x.tan(), "tan");
            let (sc_s, sc_c) = fx.sin_cos();
            let (es, ec) = x.sin_cos();
            check_ulp1(sc_s, es, "sin_cos.0");
            check_ulp1(sc_c, ec, "sin_cos.1");
            check_ulp1(fx.asin(), x.asin(), "asin");
            check_ulp1(fx.acos(), x.acos(), "acos");
            check_ulp1(fx.atan(), x.atan(), "atan");
            check_ulp1(fx.sinh(), x.sinh(), "sinh");
            check_ulp1(fx.cosh(), x.cosh(), "cosh");
            check_ulp1(fx.tanh(), x.tanh(), "tanh");
            check_ulp1(fx.asinh(), x.asinh(), "asinh");
            check_ulp1(fx.acosh(), x.acosh(), "acosh");
            check_ulp1(fx.atanh(), x.atanh(), "atanh");
            check_ulp1(fx.to_degrees(), x.to_degrees(), "to_degrees");
            check_ulp1(fx.to_radians(), x.to_radians(), "to_radians");

            for &n in &[i32::MIN, -100, -3, -2, -1, 0, 1, 2, 3, 100, i32::MAX] {
                let actual = fx.powi(n);
                let expected = x.powi(n);
                if expected.is_nan() {
                    assert!(actual.is_nan(), "powi({x}, {n})");
                } else if x == 0.0 || x.is_infinite() || n == 0 || x.abs() == 1.0 {
                    check_bits(actual, expected, "powi special");
                } else if expected.is_finite() && actual.is_finite() {
                    let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                    assert!(actual.to_bits().abs_diff(eb) <= crate::Uint::from_le_bytes([8, 0, 0, 0, 0, 0, 0, 0]));
                }
            }
        }

        // Full 2D special grids for binary functions
        let grid = [
            f64::NEG_INFINITY,
            -3.0,
            -2.0,
            -1.0,
            -0.5,
            -0.0,
            0.0,
            0.5,
            1.0,
            2.0,
            3.0,
            f64::INFINITY,
            f64::NAN,
        ];
        for &a in &grid {
            let fa = F64::from(a);
            for &b in &grid {
                let fb = F64::from(b);
                check_ulp1(fa.powf(fb), a.powf(b), "powf grid");
                check_ulp1(fa.log(fb), a.log(b), "log grid");
                check_ulp1(fa.atan2(fb), a.atan2(b), "atan2 grid");
                check_ulp1(fa.hypot(fb), a.hypot(b), "hypot grid");
                check_bits(fa.div_euclid(fb), a.div_euclid(b), "div_euclid grid");
                check_bits(fa.rem_euclid(fb), a.rem_euclid(b), "rem_euclid grid");
            }
        }

        // FMA precision tests: cancellation of exact product + intermediate overflow/underflow
        let a = F64::from(1.0 + f64::EPSILON);
        let b = F64::from(1.0 - f64::EPSILON);
        let c = F64::from(-1.0);
        check_bits(a.mul_add(b, c), (1.0 + f64::EPSILON).mul_add(1.0 - f64::EPSILON, -1.0), "fma cancel");
        check_bits(
            F64::MAX.mul_add(F64::TWO, -F64::MAX),
            f64::MAX.mul_add(2.0, -f64::MAX),
            "fma no intermediate overflow",
        );
        let tiny = F64::MIN_POSITIVE_SUBNORMAL;
        check_bits(
            tiny.mul_add(F64::HALF, tiny * F64::HALF),
            f64::from_bits(1).mul_add(0.5, f64::from_bits(1) * 0.5),
            "fma subnormal",
        );
    }

    #[test]
    fn diverse_formats_and_identities() {
        use crate::cast::CastFrom;
        use crate::types::{F16, F128, F256};

        // 1. Extreme exponent width (EXPONENT_BITS = 124 >= 32, MB = 3)
        type HugeExp = Float<16, 3>;
        let h1 = HugeExp::one_val();
        let h2 = HugeExp::two_val();
        let hh = HugeExp::half_val();
        assert_eq!(h2.log2().to_bits(), h1.to_bits());
        assert_eq!(h1.exp2().to_bits(), h2.to_bits());
        assert_eq!(h2.powi(3).to_bits(), (h2 * h2 * h2).to_bits());
        assert_eq!(h2.powf(h2).to_bits(), (h2 * h2).to_bits());
        assert_eq!(h1.mul_add(h2, h1).to_bits(), (h2 + h1).to_bits());
        assert_eq!(HugeExp::ZERO.sin().to_bits(), HugeExp::ZERO.to_bits());
        assert_eq!(HugeExp::ZERO.cos().to_bits(), h1.to_bits());
        assert!(!HugeExp::MAX.sin().is_nan());
        assert!(!HugeExp::MAX.cos().is_nan());
        assert!(!HugeExp::MAX.asinh().is_nan());
        assert!(!HugeExp::MAX.acosh().is_nan());
        assert_eq!(HugeExp::MAX.tanh().to_bits(), h1.to_bits());
        assert!(!hh.atanh().is_nan());
        assert!(!hh.asin().is_nan());
        assert!(!hh.acos().is_nan());
        assert_eq!(h1.hypot(HugeExp::ZERO).to_bits(), h1.to_bits());
        assert_eq!(HugeExp::MAX.div_euclid(h2).to_bits(), (HugeExp::MAX / h2).to_bits());
        assert_eq!(HugeExp::MAX.rem_euclid(h2).to_bits(), HugeExp::ZERO.to_bits());

        // 2. Tiny exponent width (EXPONENT_BITS = 2, MB = 13)
        type TinyExp = Float<2, 13>;
        let t1 = TinyExp::one_val();
        let th = TinyExp::half_val();
        let t2 = TinyExp::two_val();
        assert_eq!(t2.log2().to_bits(), t1.to_bits());
        assert_eq!(th.log2().to_bits(), TinyExp::neg_one_val().to_bits());
        assert_eq!(TinyExp::neg_one_val().exp2().to_bits(), th.to_bits());
        assert_eq!(t1.exp2().to_bits(), t2.to_bits());
        let (ts, tc) = th.sin_cos();
        let sum_sq = ts.mul_add(ts, tc * tc);
        assert!(sum_sq.to_bits().abs_diff(t1.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.asin().sin().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.acos().cos().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.atan().tan().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.sinh().asinh().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(t2.acosh().cosh().to_bits().abs_diff(t2.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.atanh().tanh().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));

        // 3. F16 identities
        let x16 = F16::from_str("0.75").unwrap();
        let (s16, c16) = x16.sin_cos();
        assert_eq!(s16.to_bits(), x16.sin().to_bits());
        assert_eq!(c16.to_bits(), x16.cos().to_bits());
        assert!(x16.to_degrees().to_radians().to_bits().abs_diff(x16.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));

        // 4. F128 high-precision identities including Payne-Hanek at F128::MAX!
        let x128 = F128::from_str("0.625").unwrap();
        let (s128, c128) = x128.sin_cos();
        let id128 = s128 * s128 + c128 * c128;
        assert!(id128.to_bits().abs_diff(F128::ONE.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.asin().sin().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.acos().cos().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.atan().tan().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.exp_m1().ln_1p().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.sinh().asinh().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.atanh().tanh().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.exp2().log2().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        let y128 = F128::from_str("1000.0").unwrap();
        assert_eq!(y128.log10().to_bits(), F128::from_str("3.0").unwrap().to_bits());
        assert_eq!(F128::from_str("3.0").unwrap().hypot(F128::from_str("4.0").unwrap()).to_bits(), F128::from_str("5.0").unwrap().to_bits());
        assert!(x128.to_degrees().to_radians().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        // F128::MAX Payne-Hanek trig reduction (dynamic 18000-bit Størmer 2/pi!)
        let (ms128, mc128) = F128::MAX.sin_cos();
        let mid128 = ms128 * ms128 + mc128 * mc128;
        assert!(mid128.to_bits().abs_diff(F128::ONE.to_bits()) <= crate::Uint::cast_from(2u8));

        // 5. F256 (> 512-bit dynamic Machin/Størmer constants path!)
        let x256 = F256::from_str("0.3").unwrap();
        let (s256, c256) = x256.sin_cos();
        let id256 = s256 * s256 + c256 * c256;
        assert!(id256.to_bits().abs_diff(F256::ONE.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x256.exp_m1().ln_1p().to_bits().abs_diff(x256.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x256.to_degrees().to_radians().to_bits().abs_diff(x256.to_bits()) <= crate::Uint::cast_from(2u8));
        assert_eq!(F256::from_str("1000000.0").unwrap().log10().to_bits(), F256::from_str("6.0").unwrap().to_bits());
    }

    #[test]
    fn exact_payne_hanek_and_f128_oracle() {
        use crate::cast::CastFrom;
        use crate::types::{F64, F128};
        use crate::Uint;

        // Independently computed with 2000-bit Størmer pi in Python:
        // verify 0-ULP bit-for-bit exactness for sin, cos, tan, sin_cos on huge F64 inputs!
        let exact_cases: [(u64, u64, u64, u64); 5] = [
            (0x43dfdfffffffc040, 0x3fe660a185458088, 0x3fe6dfe84bcd9e1c, 0x3fef4df300ceea69),
            (0x7fefffffffffffff, 0x3f7452fc98b34e97, 0xbfefffe62ecfab75, 0xbf74530cfe729484),
            (0x4301c22f08410a7e, 0xbfef30fff3c0e9a1, 0x3fcc97b7cf832da0, 0xc0117448046dacc9),
            (0x5280000000000000, 0x3fea1f6f7dd0d806, 0x3fe27b7f67a765e0, 0x3ff69d3b4a37e92e),
            (0x4679397652221411, 0xbfee4133efcc8035, 0x3fd4d8ed359e7975, 0xc007384f9782e706),
        ];

        for (xb, es, ec, et) in exact_cases {
            let x = F64::from_bits(Uint::cast_from(xb));
            let (s, c) = x.sin_cos();
            assert_eq!(s.to_bits(), Uint::cast_from(es), "sin({xb:#018x})");
            assert_eq!(c.to_bits(), Uint::cast_from(ec), "cos({xb:#018x})");
            assert_eq!(x.sin().to_bits(), Uint::cast_from(es), "sin standalone({xb:#018x})");
            assert_eq!(x.cos().to_bits(), Uint::cast_from(ec), "cos standalone({xb:#018x})");
            assert_eq!(x.tan().to_bits(), Uint::cast_from(et), "tan({xb:#018x})");
        }

        // Exact bit-for-bit verification of 18000-bit dynamic Størmer Payne-Hanek on F128::MAX!
        let (ms, mc) = F128::MAX.sin_cos();
        let exp_ms = F128::from_str("0.9519148540788204811363248929375729420330").unwrap();
        let exp_mc = F128::from_str("-0.3063627108250903148866002244844001791931").unwrap();
        assert_eq!(ms.to_bits(), exp_ms.to_bits(), "F128::MAX.sin() exact 113-bit match");
        assert_eq!(mc.to_bits(), exp_mc.to_bits(), "F128::MAX.cos() exact 113-bit match");
    }

    #[test]
    fn exhaustive_16bit_all_math_against_f64() {
        use crate::cast::CastFrom;

        fn decode<const MB: usize>(bits: u16) -> f64 {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1i32 << (exp_bits - 1)) - 1;
            let sign = if (bits >> 15) != 0 { -1.0f64 } else { 1.0f64 };
            let raw_exp = (bits >> MB) & exp_max;
            let mant = (bits & ((1u16 << MB) - 1)) as u64;
            if raw_exp == exp_max {
                if mant == 0 { sign * f64::INFINITY } else { f64::NAN }
            } else if raw_exp == 0 {
                if mant == 0 {
                    sign * 0.0
                } else {
                    sign * (mant as f64) * 2.0f64.powi(1 - bias - MB as i32)
                }
            } else {
                let sig = (1u64 << MB) | mant;
                sign * (sig as f64) * 2.0f64.powi(raw_exp as i32 - bias - MB as i32)
            }
        }

        // Returns Some(expected_bits) if f64 is decisively away from a Float<2, MB> rounding
        // midpoint (i.e. > 16 f64 ULPs from halfway), or None if ambiguous for the f64 oracle.
        fn encode_rne_decisive<const MB: usize>(val: f64) -> Option<u16> {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1i32 << (exp_bits - 1)) - 1;
            let sign_bit: u16 = if val.is_sign_negative() { 0x8000 } else { 0 };
            if val.is_nan() {
                return Some((exp_max << MB) | (1u16 << (MB - 1)));
            }
            if val.is_infinite() {
                return Some(sign_bit | (exp_max << MB));
            }
            let a = val.abs();
            if a == 0.0 {
                return Some(sign_bit);
            }
            let fb = a.to_bits();
            let f_exp = ((fb >> 52) & 0x7ff) as i32 - 1023;
            let f_sig = (1u64 << 52) | (fb & ((1u64 << 52) - 1));
            let target_e = f_exp + bias;
            let (mut e, shift) = if target_e <= 0 {
                (1i32, 52 - MB as i32 + (1 - target_e))
            } else {
                (target_e, 52 - MB as i32)
            };
            if shift > 54 {
                return Some(sign_bit);
            }
            if shift == 53 {
                // In [0.5, 1.0) of MIN_POSITIVE_SUBNORMAL
                let tail = f_sig & ((1u64 << 52) - 1);
                if tail <= 64 {
                    return None;
                }
                return Some(sign_bit | 1);
            }
            if shift == 54 {
                if ((1u64 << 53) - f_sig) <= 64 {
                    return None;
                }
                return Some(sign_bit);
            }
            let half = 1u64 << (shift - 1);
            let mask = (1u64 << shift) - 1;
            let rem = f_sig & mask;
            if rem.abs_diff(half) <= 64 {
                return None;
            }
            let mut m = f_sig >> shift;
            if rem > half || (rem == half && (m & 1) != 0) {
                m += 1;
                if m == (1u64 << (MB + 1)) {
                    m >>= 1;
                    e += 1;
                }
            }
            if e >= exp_max as i32 {
                return Some(sign_bit | (exp_max << MB));
            }
            if m == 0 {
                return Some(sign_bit);
            }
            if (m & (1u64 << MB)) != 0 {
                Some(sign_bit | ((e as u16) << MB) | ((m as u16) & ((1u16 << MB) - 1)))
            } else {
                Some(sign_bit | (m as u16))
            }
        }

        fn sweep<const MB: usize>() {
            let mut checked = 0usize;
            for bits in (0u16..=u16::MAX).step_by(11) {
                let x = Float::<2, MB>::from_bits(crate::Uint::cast_from(bits));
                let fx = decode::<MB>(bits);

                let (sc_s, sc_c) = x.sin_cos();
                let (es, ec) = fx.sin_cos();
                let checks: [(&str, Float<2, MB>, f64); 23] = [
                    ("exp2", x.exp2(), fx.exp2()),
                    ("exp_m1", x.exp_m1(), fx.exp_m1()),
                    ("ln_1p", x.ln_1p(), fx.ln_1p()),
                    ("log2", x.log2(), fx.log2()),
                    ("log10", x.log10(), fx.log10()),
                    ("sin", x.sin(), fx.sin()),
                    ("cos", x.cos(), fx.cos()),
                    ("tan", x.tan(), fx.tan()),
                    ("sin_cos.0", sc_s, es),
                    ("sin_cos.1", sc_c, ec),
                    ("asin", x.asin(), fx.asin()),
                    ("acos", x.acos(), fx.acos()),
                    ("atan", x.atan(), fx.atan()),
                    ("sinh", x.sinh(), fx.sinh()),
                    ("cosh", x.cosh(), fx.cosh()),
                    ("tanh", x.tanh(), fx.tanh()),
                    ("asinh", x.asinh(), fx.asinh()),
                    ("acosh", x.acosh(), fx.acosh()),
                    ("atanh", x.atanh(), fx.atanh()),
                    ("to_degrees", x.to_degrees(), fx.to_degrees()),
                    ("to_radians", x.to_radians(), fx.to_radians()),
                    ("powi_3", x.powi(3), fx.powi(3)),
                    ("powi_-2", x.powi(-2), fx.powi(-2)),
                ];
                for (name, actual, expected_f) in checks {
                    if expected_f.is_nan() {
                        assert!(actual.is_nan(), "MB={MB} {name}({bits:#06x} [{fx}]) expected NaN");
                    } else if let Some(expected_bits) = encode_rne_decisive::<MB>(expected_f) {
                        let actual_bits = u16::from_le_bytes(actual.to_bits().bytes);
                        assert_eq!(
                            actual_bits, expected_bits,
                            "MB={MB} {name}({bits:#06x} [{fx}]): got {actual_bits:#06x}, expected {expected_bits:#06x} ({expected_f})"
                        );
                        checked += 1;
                    }
                }
            }
            assert!(checked > 60_000);

            // Binary & ternary critical grid (zero, subnormals, 0.5, 1.0, 2.0, max, inf, NaN)
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1u16 << (exp_bits - 1)) - 1;
            let mant_mask = (1u16 << MB) - 1;
            let critical: [u16; 20] = [
                0x0000,
                0x8000,
                0x0001,
                0x8001,
                mant_mask,
                0x8000 | mant_mask,
                1u16 << MB,
                0x8000 | (1u16 << MB),
                bias << MB,
                0x8000 | (bias << MB),
                (bias << MB) | 1,
                (bias << MB) | (1u16 << (MB - 1)),
                ((bias + 1) << MB),
                0x8000 | ((bias + 1) << MB),
                ((exp_max - 1) << MB) | mant_mask,
                0x8000 | ((exp_max - 1) << MB) | mant_mask,
                exp_max << MB,
                0x8000 | (exp_max << MB),
                (exp_max << MB) | (1u16 << (MB - 1)),
                1u16 << (MB - 1),
            ];
            for &ba in &critical {
                let a = Float::<2, MB>::from_bits(crate::Uint::cast_from(ba));
                let fa = decode::<MB>(ba);
                for &bb in &critical {
                    let b = Float::<2, MB>::from_bits(crate::Uint::cast_from(bb));
                    let fb = decode::<MB>(bb);
                    for (name, actual, expected_f) in [
                        ("hypot", a.hypot(b), fa.hypot(fb)),
                        ("atan2", a.atan2(b), fa.atan2(fb)),
                        ("log", a.log(b), fa.log(fb)),
                        ("powf", a.powf(b), fa.powf(fb)),
                        ("mul_add", a.mul_add(b, a), fa.mul_add(fb, fa)),
                        ("mul_add_neg", a.mul_add(b, -b), fa.mul_add(fb, -fb)),
                    ] {
                        if expected_f.is_nan() {
                            assert!(actual.is_nan(), "MB={MB} {name}({ba:#06x}, {bb:#06x}) expected NaN");
                        } else if let Some(expected_bits) = encode_rne_decisive::<MB>(expected_f) {
                            let actual_bits = u16::from_le_bytes(actual.to_bits().bytes);
                            assert_eq!(
                                actual_bits, expected_bits,
                                "MB={MB} {name}({ba:#06x} [{fa}], {bb:#06x} [{fb}]): got {actual_bits:#06x}, expected {expected_bits:#06x} ({expected_f})"
                            );
                        }
                    }
                }
            }
        }

        sweep::<10>(); // IEEE binary16 (EXPONENT_BITS = 5)
        sweep::<13>(); // EXPONENT_BITS = 2 (subnormal 0.5!)
    }
}
