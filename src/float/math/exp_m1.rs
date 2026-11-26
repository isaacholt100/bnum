use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_exp, ext_expm1_small, ext_from_float, ext_one, ext_round_to_float, ext_sub,
    int_from_i64, int_from_u64, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `e^self - 1` accurately even for tiny `|self|` using rounding mode `RM`.
    pub(super) const fn exp_m1_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return self,
            FpCategory::Infinite => {
                return if self.is_sign_negative() {
                    Self::neg_one_val()
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(self, l1);

        // Tiny |x|: e^x - 1 = x + x^2/2 + ... > x strictly for all x != 0.
        if x0.exp.lt(&int_from_i64(-((Self::MB as i64) + 4))) {
            return Self::round_with_tiny_delta(self, true);
        }

        // Large negative x: e^x - 1 = -1 + e^x > -1 strictly.
        let neg_sat_exp = int_from_u64(((Self::MB + 4) as u64).ilog2() as u64 + 1);
        if x0.sign && x0.exp.ge(&neg_sat_exp) {
            return Self::round_with_tiny_delta(Self::neg_one_val(), true);
        }
        let pos_exp_thresh = int_from_u64(((Self::MB + 140) as u64).ilog2() as u64 + 1);
        if !x0.sign && x0.exp.ge(&pos_exp_thresh) {
            return self.exp_internal();
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let res = if x.exp.lt(&Int::ZERO) {
                ext_expm1_small(&x, limbs)
            } else {
                ext_sub(&ext_exp(&x, limbs), &ext_one(limbs), limbs)
            };

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
