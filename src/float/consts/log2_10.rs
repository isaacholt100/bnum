use super::Float;
use crate::float::math::fixed::{
    Ext, ZIV_ERR_BITS, const_ln2, const_ln10, ext_div, ext_round_to_float, fp_from_be_table,
    max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};

/// Exact top 512 bits of `log2(10) / 4` (`floor((log2(10) / 4) * 2^512)`), big-endian `u64` limbs.
const LOG2_10_DIV4_512_BE: [u64; 8] = [
    0xd49a_784b_cd1b_8afe,
    0x492b_f6ff_4daf_db4c,
    0xd96c_55fe_37b3_ad4e,
    0x91b6_ac80_82e7_859d,
    0x0665_0fde_9dd5_1f3a,
    0x3e24_beab_63ad_0bd6,
    0x1435_ee48_0565_ca54,
    0xcff1_62cd_4f60_3381,
];

/// Returns `log2(10)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
const fn const_log2_10<const W: usize>(limbs: usize) -> Ext<W> {
    if limbs <= LOG2_10_DIV4_512_BE.len() {
        Ext {
            sign: false,
            zero: false,
            exp: Int::ONE,
            sig: fp_from_be_table(&LOG2_10_DIV4_512_BE, limbs),
        }
    } else {
        ext_div(&const_ln10(limbs), &const_ln2(limbs), limbs)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `log2(10)` rounded to `Self` according to `RM`.
    pub(super) const fn log2_10_internal() -> Self {
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

            let res = const_log2_10(limbs);
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
