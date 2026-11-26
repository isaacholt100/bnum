use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_add, ext_div, ext_exp, ext_expm1_small, ext_from_float, ext_from_u64,
    ext_one, ext_round_to_float, ext_scale_pow2, ext_sub, int_from_i64, int_from_u64, max_limbs,
    stage1_limbs,
};
use crate::Exponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `tanh(self)` accurately across all inputs with IEEE 754 round-to-nearest-even.
    pub(super) const fn tanh_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let neg = self.is_sign_negative();
        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return self,
            FpCategory::Infinite => {
                return if neg {
                    Self::neg_one_val()
                } else {
                    Self::one_val()
                };
            }
            _ => {}
        }

        let abs_x = self.abs();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(abs_x, l1);
        // Tiny |x| < 2^-(MB/2 + 4): tanh(x) = x - x^3/3 + ... rounds directly to self.
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return self;
        }

        // Saturation: for |x| >= MB + 4, 1 - tanh(|x|) < 2 * e^(-2*(MB+4)) < 2^-(2*MB + 7) => ±1.0.
        let sat_exp = int_from_u64(((Self::MB + 4) as u64).ilog2() as u64 + 1);
        if x0.exp.ge(&sat_exp) {
            return if neg {
                Self::neg_one_val()
            } else {
                Self::one_val()
            };
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            let two_a = ext_scale_pow2(a, 1);
            let mut res = if a.exp.lt(&int_from_i64(-1)) {
                // |x| < 0.5 => 2|x| < 1: d = expm1(2|x|), tanh(|x|) = d / (2 + d).
                let d = ext_expm1_small(&two_a, limbs);
                ext_div(&d, &ext_add(&ext_from_u64(2, limbs), &d, limbs), limbs)
            } else {
                // |x| >= 0.5 => e^(2|x|) >= e: tanh(|x|) = (e^(2|x|) - 1) / (e^(2|x|) + 1).
                let e2 = ext_exp(&two_a, limbs);
                let one = ext_one(limbs);
                ext_div(&ext_sub(&e2, &one, limbs), &ext_add(&e2, &one, limbs), limbs)
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
