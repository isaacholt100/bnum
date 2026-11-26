use super::Float;
use crate::Exponent;
use crate::float::math::fixed::{
    Ext, ZIV_ERR_BITS, const_ln10, ext_recip, ext_round_to_float, fp_from_be_table, int_from_i64,
    max_limbs, stage1_limbs,
};

/// Exact top 512 bits of `log10(e) * 2` (`floor((log10(e) * 2) * 2^512)`), big-endian `u64` limbs.
const LOG10_E_MUL2_512_BE: [u64; 8] = [
    0xde5b_d8a9_3728_7195,
    0x355b_aaaf_ad33_dc32,
    0x3ee3_4602_45c9_a202,
    0x3a3f_2d44_f78e_a53c,
    0x7542_4efa_1402_f3f2,
    0x9223_5592_c646_4a15,
    0x18ce_3bd9_fd38_dcbc,
    0x6fa2_b8d2_c8cd_a7b3,
];

/// Returns `log10(e) = 1 / ln(10)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
const fn const_log10_e<const W: usize>(limbs: usize) -> Ext<W> {
    if limbs <= LOG10_E_MUL2_512_BE.len() {
        Ext {
            sign: false,
            zero: false,
            exp: int_from_i64(-2),
            sig: fp_from_be_table(&LOG10_E_MUL2_512_BE, limbs),
        }
    } else {
        ext_recip(&const_ln10(limbs), limbs)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `log10(e)` rounded to `Self` according to `RM`.
    pub(super) const fn log10_e_internal() -> Self {
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

            let res = const_log10_e(limbs);
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
