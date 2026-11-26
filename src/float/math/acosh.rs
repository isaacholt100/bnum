use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_ln2, ext_add, ext_from_float, ext_from_u64, ext_ln, ext_ln1p, ext_mul,
    ext_one, ext_round_to_float, ext_sqrt, ext_sub, int_from_u64, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `acosh(self) = ln(self + sqrt(self^2 - 1))` for `self >= 1` without
    /// overflow for large `self` and without cancellation near `1`, with IEEE 754
    /// round-to-nearest-even.
    pub(super) const fn acosh_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Infinite => {
                return if self.is_sign_negative() {
                    Self::NAN
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        let one = Self::one_val();
        match self.total_cmp(&one) {
            Ordering::Less => return Self::NAN,
            Ordering::Equal => return Self::ZERO,
            Ordering::Greater => {}
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(self, l1);
        let is_huge = x0.exp.gt(&int_from_u64((Self::MB + 64) as u64));

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let res = if is_huge {
                // Huge x > 2^(MB + 64): acosh(x) = ln(x) + ln(2) without x^2 overflow.
                ext_add(&ext_ln(&x, limbs), &const_ln2(limbs), limbs)
            } else {
                // Compute d = x - 1 exactly, then acosh(x) = ln1p(d + sqrt(d * (d + 2))).
                let d = ext_sub(&x, &ext_one(limbs), limbs);
                let d_plus_2 = ext_add(&d, &ext_from_u64(2, limbs), limbs);
                let root = ext_sqrt(&ext_mul(&d, &d_plus_2, limbs), limbs);
                ext_ln1p(&ext_add(&d, &root, limbs), limbs)
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
