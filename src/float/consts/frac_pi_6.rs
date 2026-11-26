use super::Float;
use super::frac_pi_3::const_pi_div6_sig;
use crate::Exponent;
use crate::float::math::fixed::{
    Ext, ZIV_ERR_BITS, ext_round_to_float, int_from_i64, max_limbs, stage1_limbs,
};

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `pi / 6` rounded to `Self` according to `RM`.
    pub(super) const fn frac_pi_6_internal() -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let res = Ext {
                sign: false,
                zero: false,
                exp: int_from_i64(-1),
                sig: const_pi_div6_sig(limbs),
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
