use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_ln2, ext_add, ext_div, ext_from_float, ext_ln, ext_ln1p, ext_mul, ext_one,
    ext_round_to_float, ext_sqrt, int_from_i64, int_from_u64, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `asinh(self) = ln(self + sqrt(self^2 + 1))` without overflow for large `|self|`
    /// and without cancellation near `0`, with IEEE 754 round-to-nearest-even.
    pub(super) const fn asinh_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan | FpCategory::Zero | FpCategory::Infinite => return self,
            _ => {}
        }

        let neg = self.is_sign_negative();
        let abs_x = self.abs();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(abs_x, l1);
        // Tiny |x| < 2^-(MB/2 + 4): asinh(x) = x - x^3/6 + ... rounds directly to self.
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return self;
        }

        let huge_thresh = int_from_u64((Self::MB + 64) as u64);
        let is_huge = x0.exp.gt(&huge_thresh);

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            let mut res = if is_huge {
                // Huge |x| > 2^(MB + 64): sqrt(x^2 + 1) = |x|*(1 + O(2^-(2*MB+129))),
                // so asinh(|x|) = ln(|x|) + ln(2) without squaring overflow.
                ext_add(&ext_ln(&a, limbs), &const_ln2(limbs), limbs)
            } else if a.exp.lt(&Int::ZERO) {
                // |x| < 1: asinh(|x|) = ln1p(|x| + x^2 / (1 + sqrt(1 + x^2))).
                let one = ext_one(limbs);
                let x2 = ext_mul(&a, &a, limbs);
                let s = ext_sqrt(&ext_add(&one, &x2, limbs), limbs);
                let t = ext_add(&a, &ext_div(&x2, &ext_add(&one, &s, limbs), limbs), limbs);
                ext_ln1p(&t, limbs)
            } else {
                // 1 <= |x| <= 2^(MB + 64): asinh(|x|) = ln(|x| + sqrt(x^2 + 1)).
                let x2 = ext_mul(&a, &a, limbs);
                let s = ext_sqrt(&ext_add(&x2, &ext_one(limbs), limbs), limbs);
                ext_ln(&ext_add(&a, &s, limbs), limbs)
            };
            res.sign = neg;

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
