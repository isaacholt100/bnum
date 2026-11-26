use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_pi, const_pi_over_2, ext_add, ext_atan, ext_div, ext_from_float, ext_mul,
    ext_one, ext_round_to_float, ext_sqrt, ext_sub, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `acos(self)` in `[0, pi]` with IEEE 754 round-to-nearest-even.
    pub(super) const fn acos_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        match self.classify() {
            FpCategory::Nan | FpCategory::Infinite => return Self::NAN,
            FpCategory::Zero => {
                let res = const_pi_over_2(l2);
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            _ => {}
        }

        let neg = self.is_sign_negative();
        let abs_x = self.abs();
        let one = Self::one_val();

        match abs_x.total_cmp(&one) {
            Ordering::Greater => return Self::NAN,
            Ordering::Equal => {
                return if neg {
                    let res = const_pi(l2);
                    match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                        Some(v) => v,
                        None => unreachable!(),
                    }
                } else {
                    Self::ZERO
                };
            }
            Ordering::Less => {}
        }

        let half = Self::half_val();
        let large = !matches!(abs_x.total_cmp(&half), Ordering::Less);

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            let e_one = ext_one(limbs);
            let d = ext_sub(&e_one, &a, limbs);
            let s = ext_sqrt(&ext_mul(&d, &ext_add(&e_one, &a, limbs), limbs), limbs);

            let res = if !neg {
                if large {
                    // x >= 0.5: acos(x) = atan(sqrt((1-x)(1+x)) / x), cancellation-free near 1.
                    ext_atan(&ext_div(&s, &a, limbs), limbs)
                } else {
                    // 0 < x < 0.5: acos(x) = pi/2 - atan(x / sqrt(1 - x^2))
                    let t = ext_atan(&ext_div(&a, &s, limbs), limbs);
                    ext_sub(&const_pi_over_2(limbs), &t, limbs)
                }
            } else if large {
                // -1 < x <= -0.5: acos(x) = pi - atan(sqrt((1-|x|)(1+|x|)) / |x|)
                let t = ext_atan(&ext_div(&s, &a, limbs), limbs);
                ext_sub(&const_pi(limbs), &t, limbs)
            } else {
                // -0.5 < x < 0: acos(x) = pi/2 + atan(|x| / sqrt(1 - x^2))
                let t = ext_atan(&ext_div(&a, &s, limbs), limbs);
                ext_add(&const_pi_over_2(limbs), &t, limbs)
            };

            if let Some(out) = ext_round_to_float::<W, MB, RM>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
