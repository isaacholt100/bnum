use super::Float;
use crate::Exponent;
use crate::float::math::fixed::{
    Ext, ZIV_ERR_BITS, const_ln2, const_ln10, ext_div, ext_round_to_float, fp_from_be_table,
    int_from_i64, max_limbs, stage1_limbs,
};

/// Exact top 512 bits of `log10(2) * 2` (`floor((log10(2) * 2) * 2^512)`), big-endian `u64` limbs.
const LOG10_2_MUL2_512_BE: [u64; 8] = [
    0x9a20_9a84_fbcf_f798,
    0x8f89_59ac_0b7c_9178,
    0x26ad_30c5_43d1_f349,
    0x8a5e_6f26_b7cc_63cb,
    0x286a_2d81_919f_abd0,
    0x9e5c_bc73_3471_bd12,
    0x9147_3495_b18b_921e,
    0x58b5_27f5_2552_d2cc,
];

/// Returns `log10(2)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
const fn const_log10_2<const W: usize>(limbs: usize) -> Ext<W> {
    if limbs <= LOG10_2_MUL2_512_BE.len() {
        Ext {
            sign: false,
            zero: false,
            exp: int_from_i64(-2),
            sig: fp_from_be_table(&LOG10_2_MUL2_512_BE, limbs),
        }
    } else {
        ext_div(&const_ln2(limbs), &const_ln10(limbs), limbs)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `log10(2)` rounded to `Self` according to `RM`.
    pub(super) const fn log10_2_internal() -> Self {
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

            let res = const_log10_2(limbs);
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
