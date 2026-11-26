use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_div, ext_from_float, ext_ln, ext_round_to_float, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `log_{base}(self) = ln(self) / ln(base)` with single IEEE 754
    /// round-to-nearest-even rounding, matching Rust std special-value semantics.
    pub(super) const fn log_internal(self, base: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        if self.is_nan() || base.is_nan() {
            return Self::NAN;
        }
        if (self.is_sign_negative() && !self.is_zero())
            || (base.is_sign_negative() && !base.is_zero())
        {
            return Self::NAN;
        }

        let one = Self::one_val();
        let x_cat = self.classify();
        let b_cat = base.classify();
        let x_eq_one = self.to_bits().eq(&one.to_bits());
        let b_eq_one = base.to_bits().eq(&one.to_bits());

        // Handle all special combinations identically to `self.ln() / base.ln()`:
        // ln(0) = -inf, ln(+inf) = +inf, ln(1) = +0.0, ln(0 < v < 1) < 0, ln(v > 1) > 0.
        match (x_cat, b_cat) {
            (FpCategory::Zero, FpCategory::Zero)
            | (FpCategory::Zero, FpCategory::Infinite)
            | (FpCategory::Infinite, FpCategory::Zero)
            | (FpCategory::Infinite, FpCategory::Infinite) => return Self::NAN,
            (FpCategory::Zero, _) => {
                // -inf / ln(base): +inf if base < 1, -inf if base >= 1
                return if matches!(base.total_cmp(&one), Ordering::Less) {
                    Self::INFINITY
                } else {
                    Self::NEG_INFINITY
                };
            }
            (FpCategory::Infinite, _) => {
                // +inf / ln(base): -inf if base < 1, +inf if base >= 1
                return if matches!(base.total_cmp(&one), Ordering::Less) {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            (_, FpCategory::Zero) => {
                // ln(x) / -inf: +0.0 if x < 1 (ln(x) < 0), -0.0 if x >= 1 (ln(x) >= +0.0)
                return if matches!(self.total_cmp(&one), Ordering::Less) {
                    Self::ZERO
                } else {
                    Self::NEG_ZERO
                };
            }
            (_, FpCategory::Infinite) => {
                // ln(x) / +inf: -0.0 if x < 1, +0.0 if x >= 1
                return if matches!(self.total_cmp(&one), Ordering::Less) {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            _ => {}
        }

        if x_eq_one {
            if b_eq_one {
                return Self::NAN;
            }
            return if matches!(base.total_cmp(&one), Ordering::Less) {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }
        if b_eq_one {
            return if matches!(self.total_cmp(&one), Ordering::Less) {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        // Exact self == base fast path
        if self.to_bits().eq(&base.to_bits()) {
            return one;
        }
        // Fast path for base == 2.0
        if base.to_bits().eq(&Self::two_val().to_bits()) {
            return self.log2_internal();
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let b = ext_from_float(base, limbs);
            let res = ext_div(&ext_ln(&x, limbs), &ext_ln(&b, limbs), limbs);

            if let Some(out) =
                ext_round_to_float::<W, MB, RM>(&res, limbs, ZIV_ERR_BITS, final_round)
            {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
