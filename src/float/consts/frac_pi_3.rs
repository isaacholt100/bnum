use super::Float;
use crate::float::math::fixed::{
    Ext, Fixed, ZIV_ERR_BITS, ext_round_to_float, max_limbs, pi_div4_sig, stage1_limbs,
};
use crate::{Exponent, Int};

/// Returns `pi / 6 = (2 * (pi / 4)) / 3` normalized in `[0.5, 1.0)` at scale `2^(64 * limbs)`
/// (`exp = 0` for `pi / 3`, `exp = -1` for `pi / 6`).
pub(super) const fn const_pi_div6_sig<const W: usize>(limbs: usize) -> Fixed<W> {
    let pi_div4 = pi_div4_sig::<W>(limbs);
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut rem: u128 = (pi_div4.get(limbs - 1) >> 63) as u128;
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let lo_carry = if i > 0 { pi_div4.get(i - 1) >> 63 } else { 0 };
        let limb_2x = (pi_div4.get(i) << 1) | lo_carry;
        let cur = (rem << 64) | (limb_2x as u128);
        out.set(i, (cur / 3) as u64);
        rem = cur % 3;
    }
    debug_assert!(out.get(limbs - 1) >= (1u64 << 63));
    out
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `pi / 3` rounded to `Self` according to `RM`.
    pub(super) const fn frac_pi_3_internal() -> Self {
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
                exp: Int::ZERO,
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
