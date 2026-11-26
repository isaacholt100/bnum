use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_ln10, ext_div, ext_from_float, ext_ln, ext_round_to_float, max_limbs,
    stage1_limbs,
};
use crate::Exponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `log10(self)` with IEEE 754 round-to-nearest-even via two-stage Ziv rounding.
    pub(super) const fn log10_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return Self::NEG_INFINITY,
            FpCategory::Infinite => {
                return if self.is_sign_negative() {
                    Self::NAN
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        if self.is_sign_negative() {
            return Self::NAN;
        }

        let one = Self::one_val();
        if self.to_bits().eq(&one.to_bits()) {
            return Self::ZERO;
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let ln_x = ext_ln(&x, limbs);
            let res = ext_div(&ln_x, &const_ln10(limbs), limbs);

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
