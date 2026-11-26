use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_div, ext_from_float, ext_ln1p, ext_one, ext_round_to_float, ext_scale_pow2,
    ext_sub, int_from_i64, max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `atanh(self) = 0.5 * ln((1 + self) / (1 - self))` for `|self| <= 1`
    /// without cancellation near `0` or `±1`, with IEEE 754 round-to-nearest-even.
    pub(super) const fn atanh_internal(self) -> Self {
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

        match abs_x.total_cmp(&one) {
            Ordering::Greater => return Self::NAN,
            Ordering::Equal => {
                return if neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            Ordering::Less => {}
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(abs_x, l1);
        // Tiny |x| < 2^-(MB/2 + 4): atanh(x) = x + x^3/3 + ... rounds directly to self.
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return self;
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let a = ext_from_float(abs_x, limbs);
            // atanh(|x|) = 0.5 * ln1p(2|x| / (1 - |x|))
            let one_minus_a = ext_sub(&ext_one(limbs), &a, limbs);
            let arg = ext_div(&ext_scale_pow2(a, 1), &one_minus_a, limbs);
            let mut res = ext_scale_pow2(ext_ln1p(&arg, limbs), -1);
            res.sign = neg;

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
