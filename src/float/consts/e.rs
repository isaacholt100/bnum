use super::Float;
use crate::float::math::fixed::{
    Ext, Fixed, ZIV_ERR_BITS, ext_round_to_float, fp_add, fp_div_u64, fp_from_be_table,
    fp_is_zero, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};

/// Exact top 512 bits of `e / 4` (`floor((e / 4) * 2^512)`), big-endian `u64` limbs.
const E_DIV4_512_BE: [u64; 8] = [
    0xadf8_5458_a2bb_4a9a,
    0xafdc_5620_273d_3cf1,
    0xd8b9_c583_ce2d_3695,
    0xa9e1_3641_1464_33fb,
    0xcc93_9dce_249b_3ef9,
    0x7d2f_e363_630c_75d8,
    0xf681_b202_aec4_617a,
    0xd3df_1ed5_d5fd_6561,
];

/// Computes `e / 4 = 1/4 + 1/4 + sum_{n >= 2} 1 / (4 * n!)` at scale `2^(64 * limbs)`.
const fn compute_e_div4_taylor<const W: usize>(limbs: usize) -> Fixed<W> {
    let mut sum = Fixed::<W>::ALL_ZEROS;
    sum.set(limbs - 1, 1u64 << 63);
    let mut term = Fixed::<W>::ALL_ZEROS;
    term.set(limbs - 1, 1u64 << 62);
    let mut n: u64 = 2;
    loop {
        term = fp_div_u64(&term, n, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        sum = fp_add(&sum, &term, limbs).0;
        n += 1;
    }
    sum
}

/// Returns Euler's number `e` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
const fn const_e<const W: usize>(limbs: usize) -> Ext<W> {
    let sig = if limbs <= E_DIV4_512_BE.len() {
        fp_from_be_table(&E_DIV4_512_BE, limbs)
    } else {
        compute_e_div4_taylor(limbs)
    };
    Ext {
        sign: false,
        zero: false,
        exp: Int::ONE,
        sig,
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes Euler's number `e` rounded to `Self` according to `RM`.
    pub(super) const fn e_internal() -> Self {
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

            let res = const_e(limbs);
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
