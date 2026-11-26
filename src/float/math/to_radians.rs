use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_pi, ext_div, ext_from_float, ext_from_u64, ext_mul, ext_round_to_float,
    max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Converts degrees to radians: `self * (pi / 180)` with single IEEE 754 round-to-nearest-even.
    pub(super) const fn to_radians_internal(self) -> Self {
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

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let num = ext_mul(&x, &const_pi(limbs), limbs);
            let res = ext_div(&num, &ext_from_u64(180, limbs), limbs);

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
