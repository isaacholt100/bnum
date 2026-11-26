use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_pi_over_2, ext_add, ext_atan, ext_div, ext_from_float, ext_mul, ext_neg,
    ext_one, ext_round_to_float, ext_sqrt, ext_sub, int_from_i64, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `asin(self)` in `[-pi/2, pi/2]` using rounding mode `RM`.
    pub(super) const fn asin_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan | FpCategory::Infinite => return Self::NAN,
            FpCategory::Zero => return self,
            _ => {}
        }

        let neg = self.is_sign_negative();
        let abs_x = self.abs();
        let one = Self::one_val();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        match abs_x.total_cmp(&one) {
            Ordering::Greater => return Self::NAN,
            Ordering::Equal => {
                let mut res = const_pi_over_2(l2);
                if neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            Ordering::Less => {}
        }

        let x0 = ext_from_float(abs_x, l1);
        // Tiny |x|: asin(x) = x + x^3/6 + ... (larger magnitude than |x|).
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return Self::round_with_tiny_delta(self, !neg);
        }

        let half = Self::half_val();
        let use_complement = matches!(abs_x.total_cmp(&half), Ordering::Greater);

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            let e_one = ext_one(limbs);
            let mut res = if !use_complement {
                let x2 = ext_mul(&a, &a, limbs);
                let den = ext_sqrt(&ext_sub(&e_one, &x2, limbs), limbs);
                ext_atan(&ext_div(&a, &den, limbs), limbs)
            } else {
                let d = ext_sub(&e_one, &a, limbs);
                let s = ext_sqrt(&ext_mul(&d, &ext_add(&e_one, &a, limbs), limbs), limbs);
                let t = ext_atan(&ext_div(&s, &a, limbs), limbs);
                ext_sub(&const_pi_over_2(limbs), &t, limbs)
            };
            if neg {
                res = ext_neg(res);
            }

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
