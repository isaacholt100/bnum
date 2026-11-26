use super::Float;
use crate::Exponent;
use crate::float::math::fixed::{
    Ext, Fixed, TWO_OVER_PI_1408_BE, ZIV_ERR_BITS, const_pi, ext_recip, ext_round_to_float,
    fp_from_be_table, int_from_i64, max_limbs, stage1_limbs,
};

/// Returns `2 / pi` normalized in `[0.5, 1.0)` at scale `2^(64 * limbs)`
/// (`exp = -2` for `1 / pi`, `exp = -1` for `2 / pi`).
pub(super) const fn const_two_over_pi_sig<const W: usize>(limbs: usize) -> Fixed<W> {
    if limbs <= TWO_OVER_PI_1408_BE.len() {
        fp_from_be_table(&TWO_OVER_PI_1408_BE, limbs)
    } else {
        let inv_pi = ext_recip(&const_pi(limbs), limbs);
        debug_assert!(inv_pi.exp.eq(&int_from_i64(-2)));
        inv_pi.sig
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `1 / pi` rounded to `Self` according to `RM`.
    pub(super) const fn frac_1_pi_internal() -> Self {
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
                exp: int_from_i64(-2),
                sig: const_two_over_pi_sig(limbs),
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
