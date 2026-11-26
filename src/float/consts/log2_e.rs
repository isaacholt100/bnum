use super::Float;
use crate::float::math::fixed::{
    Ext, ZIV_ERR_BITS, const_ln2, ext_recip, ext_round_to_float, fp_from_be_table, max_limbs,
    stage1_limbs,
};
use crate::{Exponent, Int};

/// Exact top 512 bits of `log2(e) / 2` (`floor((log2(e) / 2) * 2^512)`), big-endian `u64` limbs.
const LOG2_E_DIV2_512_BE: [u64; 8] = [
    0xb8aa_3b29_5c17_f0bb,
    0xbe87_fed0_691d_3e88,
    0xeb57_7aa8_dd69_5a58,
    0x8b25_166c_d1a1_3247,
    0xde1c_43f7_5517_6cd6,
    0x24d9_2f75_c16b_e0b3,
    0xea90_b9e6_0c4a_909f,
    0xc4bf_af03_53df_39b3,
];

/// Returns `log2(e) = 1 / ln(2)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
const fn const_log2_e<const W: usize>(limbs: usize) -> Ext<W> {
    if limbs <= LOG2_E_DIV2_512_BE.len() {
        Ext {
            sign: false,
            zero: false,
            exp: Int::ZERO,
            sig: fp_from_be_table(&LOG2_E_DIV2_512_BE, limbs),
        }
    } else {
        ext_recip(&const_ln2(limbs), limbs)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `log2(e)` rounded to `Self` according to `RM`.
    pub(super) const fn log2_e_internal() -> Self {
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

            let res = const_log2_e(limbs);
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
