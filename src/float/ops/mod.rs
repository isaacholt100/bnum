use super::Float;
use crate::helpers::full_op_impl;
use core::iter::{Iterator, Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

mod add;
mod div;
mod mul;
mod rem;
mod sub;

impl<const W: usize, const MB: usize, const RM: u8> Add for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

full_op_impl!(<const W: usize, const MB: usize, const RM: u8> Add, AddAssign, Float<W, MB, RM>, add, add_assign for Float<W, MB, RM>);

impl<const W: usize, const MB: usize, const RM: u8> Sum for Float<W, MB, RM> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const W: usize, const MB: usize, const RM: u8> Sum<&'a Self> for Float<W, MB, RM> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + *b)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Sub for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

full_op_impl!(<const W: usize, const MB: usize, const RM: u8> Sub, SubAssign, Float<W, MB, RM>, sub, sub_assign for Float<W, MB, RM>);

impl<const W: usize, const MB: usize, const RM: u8> Mul for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs)
    }
}

full_op_impl!(<const W: usize, const MB: usize, const RM: u8> Mul, MulAssign, Float<W, MB, RM>, mul, mul_assign for Float<W, MB, RM>);

impl<const W: usize, const MB: usize, const RM: u8> Product for Float<W, MB, RM> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MB: usize, const RM: u8> Product<&'a Self> for Float<W, MB, RM> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Div for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

full_op_impl!(<const W: usize, const MB: usize, const RM: u8> Div, DivAssign, Float<W, MB, RM>, div, div_assign for Float<W, MB, RM>);

impl<const W: usize, const MB: usize, const RM: u8> Rem for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

full_op_impl!(<const W: usize, const MB: usize, const RM: u8> Rem, RemAssign, Float<W, MB, RM>, rem, rem_assign for Float<W, MB, RM>);

impl<const W: usize, const MB: usize, const RM: u8> Neg for Float<W, MB, RM> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Neg for &Float<W, MB, RM> {
    type Output = Float<W, MB, RM>;

    #[inline]
    fn neg(self) -> Float<W, MB, RM> {
        (*self).neg()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use super::*;

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest as Add>::add(a: FTestBase, b: FTestBase),
            cases: [(1.3952888382785755e33, 1.466527384898436e33)]
        }
        test_bignum! {
            function: <FTest as Sub>::sub(a: FTestBase, b: FTestBase),
            cases: [
                (-0.0, 0.0),
                (74482736000000.0, 11088044000000.0),
                (-128.0, -115.12566)
            ]
        }
        test_bignum! {
            function: <FTest as Mul>::mul(a: FTestBase, b: FTestBase),
            cases: [
                (5.6143642e23f64 as FTestBase, 35279.223f64 as FTestBase)
            ]
        }
        test_bignum! {
            function: <FTest as Div>::div(a: FTestBase, b: FTestBase)
        }
        test_bignum! {
            function: <FTest as Rem>::rem(a: FTestBase, b: FTestBase)
        }
        test_bignum! {
            function: <FTest as Neg>::neg(f: FTestBase)
        }
    }

    #[test]
    fn ops_edge_cases_and_diverse_formats() {
        use crate::cast::CastFrom;
        use crate::types::{F32, F64, F128};
        use crate::Uint;

        // Negative overflow in div (previously returned +inf!)
        assert_eq!((-F64::MAX / F64::MIN_POSITIVE).to_bits(), F64::NEG_INFINITY.to_bits());
        assert_eq!((-F32::MAX / F32::MIN_POSITIVE).to_bits(), F32::NEG_INFINITY.to_bits());

        // Rounding carry across exponent boundary in mul & add & sub & div
        let one = F64::ONE;
        let eps = F64::EPSILON;
        let half_eps = eps / F64::TWO;
        let qtr_eps = half_eps / F64::TWO;
        assert_eq!((one - half_eps).to_bits(), Uint::cast_from((1.0f64 - f64::EPSILON / 2.0).to_bits()));
        assert_eq!((one - qtr_eps).to_bits(), Uint::cast_from((1.0f64 - f64::EPSILON / 4.0).to_bits()));
        assert_eq!((one - F64::MIN_POSITIVE_SUBNORMAL).to_bits(), one.to_bits());
        assert_eq!((F64::MAX * F64::from(1.0 + f64::EPSILON)).to_bits(), F64::INFINITY.to_bits());
        assert_eq!((-F64::MAX * F64::from(1.0 + f64::EPSILON)).to_bits(), F64::NEG_INFINITY.to_bits());

        // Exact mul carry from 2^(MB+1)-1 to 2^(MB+1)
        let a = F64::from_bits(Uint::cast_from(0x3fef_ffff_ffff_ffffu64)); // 1 - 2^-53
        let b = F64::from_bits(Uint::cast_from(0x3ff0_0000_0000_0001u64)); // 1 + 2^-52
        let fa = f64::from_bits(0x3fef_ffff_ffff_ffffu64);
        let fb = f64::from_bits(0x3ff0_0000_0000_0001u64);
        assert_eq!((a * b).to_bits(), Uint::cast_from((fa * fb).to_bits()));

        // Subnormal div sticky bits
        let s_min = F64::MIN_POSITIVE_SUBNORMAL;
        let f_min = f64::from_bits(1);
        for d in [1.0f64, 1.5, 2.0, 2.0f64.next_down(), 2.0f64.next_up(), 3.0] {
            let actual = (s_min / F64::from(d)).to_bits();
            let expected = Uint::cast_from((f_min / d).to_bits());
            assert_eq!(actual, expected, "div subnormal by {d}");
        }

        // Exact cancellation and signed zero rules
        assert_eq!((F64::NEG_ONE - F64::NEG_ONE).to_bits(), F64::ZERO.to_bits());
        assert_eq!((F64::ONE + F64::NEG_ONE).to_bits(), F64::ZERO.to_bits());
        assert_eq!((F64::NEG_ZERO + F64::NEG_ZERO).to_bits(), F64::NEG_ZERO.to_bits());
        assert_eq!((F64::NEG_ZERO - F64::ZERO).to_bits(), F64::NEG_ZERO.to_bits());
        assert_eq!((F64::NEG_ZERO - F64::NEG_ZERO).to_bits(), F64::ZERO.to_bits());

        // Extreme exponent width (EXPONENT_BITS = 124 >= 32)
        type HugeExp = Float<16, 3>;
        let h1 = HugeExp::one_val();
        let h2 = HugeExp::two_val();
        let h3 = h1 + h2;
        assert_eq!((h3 - h2).to_bits(), h1.to_bits());
        assert_eq!((h3 * h2 / h2).to_bits(), h3.to_bits());

        // Tiny exponent width (EXPONENT_BITS = 2, MB = 13)
        type TinyExp = Float<2, 13>;
        let t1 = TinyExp::one_val();
        let th = TinyExp::half_val();
        let t2 = TinyExp::two_val();
        assert_eq!((th + th).to_bits(), t1.to_bits());
        assert_eq!((t1 + t1).to_bits(), t2.to_bits());
        assert_eq!((t2 + t2).to_bits(), TinyExp::INFINITY.to_bits());
        assert_eq!((t1 - th).to_bits(), th.to_bits());
        assert_eq!((t1 / t2).to_bits(), th.to_bits());
        assert_eq!((th * t2).to_bits(), t1.to_bits());

        // F128 multi-limb division hitting Knuth D u_dash == v_dash
        let x128 = F128::from_str("3.14159265358979323846264338327950288").unwrap();
        let y128 = F128::from_str("2.71828182845904523536028747135266250").unwrap();
        let q128 = x128 / y128;
        let back = q128 * y128;
        assert!(back.to_bits().abs_diff(x128.to_bits()) <= Uint::cast_from(2u8));
    }

    #[test]
    fn exhaustive_16bit_ops_against_f64() {
        use alloc::vec::Vec;
        use crate::cast::CastFrom;
        // Exhaustively test sampled pairs across ALL categories for Float<2, 10> (IEEE binary16),
        // Float<2, 13> (EXPONENT_BITS = 2), and Float<2, 5> (EXPONENT_BITS = 10, fits in f64)
        // against exact f64 oracle!
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

        fn encode_rne<const MB: usize>(val: f64) -> u16 {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1i32 << (exp_bits - 1)) - 1;
            let sign_bit: u16 = if val.is_sign_negative() { 0x8000 } else { 0 };
            if val.is_nan() {
                return (exp_max << MB) | (1u16 << (MB - 1));
            }
            if val.is_infinite() {
                return sign_bit | (exp_max << MB);
            }
            let a = val.abs();
            if a == 0.0 {
                return sign_bit;
            }
            // Exact binary float -> round-to-nearest-even to Float<2, MB>
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
                return sign_bit;
            }
            let mut m = if shift >= 64 { 0 } else { f_sig >> shift };
            let round = if shift > 0 && shift <= 64 { (f_sig >> (shift - 1)) & 1 != 0 } else { false };
            let sticky = if shift > 1 {
                let mask = if shift - 1 >= 64 { u64::MAX } else { (1u64 << (shift - 1)) - 1 };
                (f_sig & mask) != 0
            } else {
                false
            };
            if round && (sticky || (m & 1) != 0) {
                m += 1;
                if m == (1u64 << (MB + 1)) {
                    m >>= 1;
                    e += 1;
                }
            }
            if e >= exp_max as i32 {
                return sign_bit | (exp_max << MB);
            }
            if m == 0 {
                return sign_bit;
            }
            if (m & (1u64 << MB)) != 0 {
                sign_bit | ((e as u16) << MB) | ((m as u16) & ((1u16 << MB) - 1))
            } else {
                sign_bit | (m as u16)
            }
        }

        fn check_format<const MB: usize>() {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let mant_mask = (1u16 << MB) - 1;
            let mut vals = Vec::new();
            for s in [0u16, 0x8000] {
                for &m in &[0u16, 1, 2, 3, mant_mask >> 1, (mant_mask >> 1) + 1, mant_mask - 1, mant_mask] {
                    vals.push(s | m);
                    vals.push(s | (1u16 << MB) | m);
                    vals.push(s | ((exp_max - 1) << MB) | m);
                    if exp_max > 3 {
                        let mid = exp_max / 2;
                        vals.push(s | (mid << MB) | m);
                        vals.push(s | ((mid - 1) << MB) | m);
                        vals.push(s | ((mid + 1) << MB) | m);
                    }
                }
                vals.push(s | (exp_max << MB));
            }
            vals.sort_unstable();
            vals.dedup();

            for &ba in &vals {
                let a = Float::<2, MB>::from_bits(crate::Uint::cast_from(ba));
                let fa = decode::<MB>(ba);
                for &bb in &vals {
                    let b = Float::<2, MB>::from_bits(crate::Uint::cast_from(bb));
                    let fb = decode::<MB>(bb);

                    for (op_name, res, expected_f) in [
                        ("add", a + b, fa + fb),
                        ("sub", a - b, fa - fb),
                        ("mul", a * b, fa * fb),
                        ("div", a / b, fa / fb),
                    ] {
                        if expected_f.is_nan() {
                            assert!(res.is_nan(), "{op_name}({ba:#06x}, {bb:#06x}) expected NaN");
                        } else {
                            let expected = encode_rne::<MB>(expected_f);
                            let actual: u16 = u16::from_le_bytes(res.to_bits().bytes);
                            assert_eq!(
                                actual, expected,
                                "MB={MB} {op_name}({ba:#06x} [{fa}], {bb:#06x} [{fb}]): got {actual:#06x}, expected {expected:#06x} ({expected_f})"
                            );
                        }
                    }
                }
            }
        }

        check_format::<10>(); // binary16 (EXPONENT_BITS = 5)
        check_format::<13>(); // EXPONENT_BITS = 2
        check_format::<5>();  // EXPONENT_BITS = 10 (fits in f64)
    }
}
