use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_from_float, ext_round_to_float, ext_sin_cos_float, int_from_i64, max_limbs,
    stage1_limbs,
};
use crate::Exponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `sin(self)` using rounding mode `RM`.
    pub(super) const fn sin_internal(self) -> Self {
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

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let x0 = ext_from_float(self, l1);
        // Tiny |x|: sin(x) = x - x^3/6 + ... (smaller magnitude than |x|).
        if x0.exp.lt(&int_from_i64(-(((Self::MB / 2) as i64) + 4))) {
            return Self::round_with_tiny_delta(self, self.is_sign_negative());
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let (s_ext, _) = ext_sin_cos_float(self, limbs);
            if let Some(out) =
                ext_round_to_float::<W, MB, RM>(&s_ext, limbs, ZIV_ERR_BITS, final_round)
            {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
