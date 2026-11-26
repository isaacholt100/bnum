use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_add, ext_cmp_abs, ext_div, ext_exp, ext_expm1_small, ext_from_float,
    ext_from_int, ext_mul, ext_one, ext_recip, ext_round_to_float, ext_scale_pow2, int_from_i64,
    int_from_u64, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `cosh(self) = (e^self + e^-self) / 2` using rounding mode `RM`.
    pub(super) const fn cosh_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return Self::one_val(),
            FpCategory::Infinite => return Self::INFINITY,
            _ => {}
        }

        let abs_x = self.abs();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(abs_x, l1);
        // Tiny |x|: cosh(x) = 1 + x^2/2 > 1 strictly.
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return Self::round_with_tiny_delta(Self::one_val(), true);
        }

        let exp_max = Self::exp_max_uint().cast_signed();
        let sat_bound = ext_from_int(exp_max.add(int_from_u64(4)), l1);
        if matches!(ext_cmp_abs(&x0, &sat_bound, l1), Ordering::Greater) {
            return Self::overflow_value(false);
        }

        let need_wide = x0.exp.gt(&int_from_u64(32));
        let large_e_thresh = int_from_u64((Self::MB + 64) as u64);

        let mut stage = if need_wide { 1 } else { 0 };
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            let res = if a.exp.lt(&Int::ZERO) {
                let d = ext_expm1_small(&a, limbs);
                let one = ext_one(limbs);
                let num = ext_mul(&d, &d, limbs);
                let den = ext_scale_pow2(ext_add(&one, &d, limbs), 1);
                ext_add(&one, &ext_div(&num, &den, limbs), limbs)
            } else {
                let e = ext_exp(&a, limbs);
                if e.exp.gt(&large_e_thresh) {
                    ext_scale_pow2(e, -1)
                } else {
                    let inv_e = ext_recip(&e, limbs);
                    ext_scale_pow2(ext_add(&e, &inv_e, limbs), -1)
                }
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
