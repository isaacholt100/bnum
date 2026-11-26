use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_from_float, ext_ln1p, ext_round_to_float, int_from_i64, int_from_u64,
    max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `ln(1 + self)` accurately even for tiny `|self|` (avoiding catastrophic
    /// cancellation near `0`), with IEEE 754 round-to-nearest-even.
    pub(super) const fn ln_1p_internal(self) -> Self {
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
                    Self::NAN
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        let neg_one = Self::neg_one_val();
        match self.total_cmp(&neg_one) {
            Ordering::Less => return Self::NAN,
            Ordering::Equal => return Self::NEG_INFINITY,
            Ordering::Greater => {}
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();
        let x0 = ext_from_float(self, l1);

        // Tiny |x| < 2^-(MB + 4): ln(1 + x) = x - x^2/2 + ... rounds directly to self.
        if x0.exp.lt(&int_from_i64(-((Self::MB as i64) + 4))) {
            return self;
        }

        // Huge positive x >= 2^(MB + 64): ln(1 + x) = ln(x) + O(1/x) rounds identically to ln(x).
        if !x0.sign && x0.exp.ge(&int_from_u64((Self::MB + 64) as u64)) {
            return self.ln_internal();
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let res = ext_ln1p(&x, limbs);

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
