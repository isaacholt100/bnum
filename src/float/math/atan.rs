use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_pi_over_2, ext_atan, ext_from_float, ext_neg, ext_round_to_float,
    int_from_i64, int_from_u64, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `atan(self)` in `[-pi/2, pi/2]` with IEEE 754 round-to-nearest-even.
    pub(super) const fn atan_internal(self) -> Self {
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
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return self,
            FpCategory::Infinite => {
                let mut res = const_pi_over_2(l2);
                if self.is_sign_negative() {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            _ => {}
        }

        let x0 = ext_from_float(self, l1);
        // Tiny |x| < 2^-(MB/2 + 4): atan(x) = x - x^3/3 + ... rounds directly to self.
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return self;
        }
        // Huge |x| >= 2^(MB + 64): atan(x) = ±pi/2 - 1/x rounds identically to ±pi/2.
        if x0.exp.ge(&int_from_u64((Self::MB + 64) as u64)) {
            let mut res = const_pi_over_2(l2);
            if self.is_sign_negative() {
                res = ext_neg(res);
            }
            return match ext_round_to_float::<W, MB>(&res, l2, ZIV_ERR_BITS, true) {
                Some(v) => v,
                None => unreachable!(),
            };
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let res = ext_atan(&x, limbs);

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
