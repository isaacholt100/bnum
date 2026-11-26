use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_exp, ext_from_float, ext_ln, ext_mul, ext_round_to_float, ext_sqrt,
    int_from_i64, int_from_u64, max_limbs, stage1_limbs, uint_low_u64,
};
use crate::{Exponent, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Classifies a finite non-zero float as `(is_integer, is_odd_integer)` without relying
    /// on `i32` `EXP_BIAS`.
    #[inline]
    const fn classify_int_parity(self) -> (bool, bool) {
        let (_, be, mant) = self.into_biased_uint_parts();
        let bias = Self::exp_bias_uint();
        if be.lt(&bias) {
            return (false, false);
        }
        let e = be.sub(bias);
        let mb_u = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB);
        match e.cmp(&mb_u) {
            Ordering::Greater => (true, false),
            Ordering::Equal => (true, mant.bit(0)),
            Ordering::Less => {
                let frac_bits = Self::MB - e.cast_to_unsigned_float_exponent();
                if mant.trailing_zeros() >= frac_bits {
                    (true, mant.bit(frac_bits))
                } else {
                    (false, false)
                }
            }
        }
    }

    /// If `self` is an exact integer with `|self| <= 1_000_000`, returns `Some(val as i32)`.
    #[inline]
    const fn as_small_i32(self) -> Option<i32> {
        let (sign, be, mant) = self.into_biased_uint_parts();
        let bias = Self::exp_bias_uint();
        if be.lt(&bias) {
            return None;
        }
        let e = be.sub(bias);
        if e.gt(&Uint::cast_from_unsigned_float_exponent(19)) {
            return None;
        }
        let e_u = e.cast_to_unsigned_float_exponent();
        let mag = if e_u >= Self::MB {
            let lsh = e_u - Self::MB;
            uint_low_u64(&mant) << lsh
        } else {
            let frac_bits = Self::MB - e_u;
            if mant.trailing_zeros() < frac_bits {
                return None;
            }
            uint_low_u64(&mant.shr(frac_bits))
        };
        if mag > 1_000_000 {
            return None;
        }
        let v = mag as i32;
        Some(if sign { -v } else { v })
    }

    /// Computes `self^y` matching IEEE 754 / Rust std `f32::powf` / `f64::powf` semantics
    /// with IEEE 754 round-to-nearest-even via two-stage Ziv rounding.
    pub(super) const fn powf_internal(self, y: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let one = Self::one_val();

        // IEEE 754: x^±0 == 1.0 even for NaN, and (+1.0)^y == 1.0 even for NaN.
        if y.is_zero() || self.to_bits().eq(&one.to_bits()) {
            return one;
        }
        if self.is_nan() || y.is_nan() {
            return Self::NAN;
        }

        let abs_x = self.abs();
        let x_eq_neg_one = self.is_sign_negative() && abs_x.to_bits().eq(&one.to_bits());

        if y.is_infinite() {
            if x_eq_neg_one {
                return one;
            }
            let gt_one = matches!(abs_x.total_cmp(&one), Ordering::Greater);
            return if gt_one ^ y.is_sign_negative() {
                Self::INFINITY
            } else {
                Self::ZERO
            };
        }

        let (y_is_int, y_is_odd) = y.classify_int_parity();

        match self.classify() {
            FpCategory::Zero => {
                let res_neg = self.is_sign_negative() && y_is_odd;
                return if y.is_sign_negative() {
                    if res_neg {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    }
                } else if res_neg {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            FpCategory::Infinite => {
                let res_neg = self.is_sign_negative() && y_is_odd;
                return if y.is_sign_negative() {
                    if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                } else if res_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        // Both self and y are finite and non-zero.
        let res_neg = if self.is_sign_negative() {
            if !y_is_int {
                return Self::NAN;
            }
            y_is_odd
        } else {
            false
        };

        if x_eq_neg_one {
            return if res_neg { Self::neg_one_val() } else { one };
        }

        // Exact algebraic fast paths for small integer exponents and ±0.5.
        if let Some(n) = y.as_small_i32() {
            return self.powi_internal(n);
        }
        let half = Self::half_val();
        if y.to_bits().eq(&half.to_bits()) {
            let l1 = stage1_limbs::<W, MB>();
            let l2 = max_limbs::<W>();
            let mut stage = 0;
            while stage < 2 {
                let limbs = if stage == 0 { l1 } else { l2 };
                let final_round = stage == 1 || l1 == l2;
                let r = ext_sqrt(&ext_from_float(abs_x, limbs), limbs);
                if let Some(out) = ext_round_to_float::<W, MB>(&r, limbs, ZIV_ERR_BITS, final_round) {
                    return out;
                }
                stage += 1;
            }
            unreachable!()
        }

        // General case: |x|^y = exp(y * ln(|x|)).
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        // Pre-screen huge exponents to avoid overflowing `Int<W>` in `ext_mul` or `ext_exp`.
        let y_ext0 = ext_from_float(y, l1);
        let ln_x0 = ext_ln(&ext_from_float(abs_x, l1), l1);
        let prod_neg = y_ext0.sign ^ ln_x0.sign;
        let max_prod_exp = int_from_u64((Self::EXPONENT_BITS + 3) as u64);

        let exp_sum = y_ext0.exp.checked_add(ln_x0.exp);
        match exp_sum {
            None => {
                if y_ext0.exp.is_negative() {
                    // Both exponents are deeply negative: |y * ln(|x|)| is vanishingly tiny => 1.0.
                    return if res_neg { Self::neg_one_val() } else { one };
                } else {
                    return if prod_neg {
                        if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                    } else if res_neg {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    };
                }
            }
            Some(s) => {
                if s.gt(&max_prod_exp) {
                    return if prod_neg {
                        if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                    } else if res_neg {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    };
                }
                if s.lt(&int_from_i64(-((Self::MB as i64) + 64))) {
                    return if res_neg { Self::neg_one_val() } else { one };
                }
            }
        }

        // If `prod` has > 32 integer bits (possible on huge-exponent formats), start directly at l2.
        let need_wide = match exp_sum {
            Some(s) => s.gt(&int_from_u64(32)),
            None => true,
        };
        let bias = Self::exp_bias_uint().cast_signed();
        // Conservative bound: |k| > bias + MB + 64 guarantees overflow/underflow.
        let exp_bound = bias.add(int_from_u64((Self::MB + 64) as u64));

        let mut stage = if need_wide { 1 } else { 0 };
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let y_ext = ext_from_float(y, limbs);
            let ln_x = ext_ln(&ext_from_float(abs_x, limbs), limbs);
            let prod = ext_mul(&y_ext, &ln_x, limbs);

            // Check if |prod| > exp_bound * ln(2) (conservatively |prod| > exp_bound)
            // before calling `ext_exp` so `k` stays well within `Int<W>`.
            let bound_ext = super::fixed::ext_from_int(exp_bound, limbs);
            if matches!(
                super::fixed::ext_cmp_abs(&prod, &bound_ext, limbs),
                Ordering::Greater
            ) {
                return if prod.sign {
                    if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                } else if res_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }

            let mut res = ext_exp(&prod, limbs);
            res.sign = res_neg;
            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
