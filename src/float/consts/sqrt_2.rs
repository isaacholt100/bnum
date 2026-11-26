use super::Float;
use crate::float::math::fixed::{
    Ext, Fixed, ZIV_ERR_BITS, ext_from_u64, ext_round_to_float, ext_sqrt, fp_from_be_table,
    max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};

/// Exact top 512 bits of `sqrt(2) / 2` (`floor((sqrt(2) / 2) * 2^512)`), big-endian `u64` limbs.
const SQRT2_DIV2_512_BE: [u64; 8] = [
    0xb504_f333_f9de_6484,
    0x597d_89b3_754a_be9f,
    0x1d6f_60ba_893b_a84c,
    0xed17_ac85_8333_9915,
    0x4afc_8304_3ab8_a2c3,
    0xa8b1_fe6f_dc83_db39,
    0x0f74_a85e_439c_7b4a,
    0x7804_8736_3dfa_2768,
];

/// Returns `sqrt(2) / 2` normalized in `[0.5, 1.0)` at scale `2^(64 * limbs)`
/// (`exp = 0` for `sqrt(2)`, `exp = -1` for `1 / sqrt(2)`).
pub(super) const fn const_sqrt2_div2_sig<const W: usize>(limbs: usize) -> Fixed<W> {
    if limbs <= SQRT2_DIV2_512_BE.len() {
        fp_from_be_table(&SQRT2_DIV2_512_BE, limbs)
    } else {
        let s = ext_sqrt(&ext_from_u64(2, limbs), limbs);
        debug_assert!(s.exp.eq(&Int::ZERO));
        s.sig
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `sqrt(2)` rounded to `Self` according to `RM`.
    pub(super) const fn sqrt_2_internal() -> Self {
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
                sig: const_sqrt2_div2_sig(limbs),
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
