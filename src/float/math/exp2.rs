use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_ln2, ext_add, ext_expm1_small, ext_from_float, ext_from_int, ext_mul,
    ext_one, ext_round_to_float, ext_round_to_int, ext_scale_pow2_int, ext_sub, int_from_u64,
    max_limbs, stage1_limbs,
};
use crate::{Exponent, Int, Uint};
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `2^self` with IEEE 754 round-to-nearest-even via two-stage Ziv rounding.
    pub(super) const fn exp2_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return Self::one_val(),
            FpCategory::Infinite => {
                return if self.is_sign_negative() {
                    Self::ZERO
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        let bias = Self::exp_bias_uint().cast_signed();
        let exp_max = Self::exp_max_uint().cast_signed();
        let max_exp_plus1 = bias.add(Int::ONE); // 2^(bias + 1) overflows to +inf
        let min_sub_exp_m1 = bias
            .add(int_from_u64(Self::MB as u64))
            .add(Int::ONE)
            .wrapping_neg(); // 2^(-(bias + MB + 1)) = 0.25 * MIN_POSITIVE_SUBNORMAL

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        // Quick overflow / underflow check on the unbiased exponent of self.
        let x0 = ext_from_float(self, l1);
        let max_x_exp = int_from_u64((Self::EXPONENT_BITS + 2) as u64);
        if x0.exp.ge(&max_x_exp) {
            return if x0.sign { Self::ZERO } else { Self::INFINITY };
        }

        let k = ext_round_to_int(&x0, l1);
        if k.gt(&max_exp_plus1) {
            return Self::INFINITY;
        }
        if k.lt(&min_sub_exp_m1) {
            return Self::ZERO;
        }

        // Exact integer power-of-two fast path (zero transcendental work).
        let k_ext0 = ext_from_int(k, l1);
        let f0 = ext_sub(&x0, &k_ext0, l1);
        if f0.zero {
            let out_be = k.add(bias);
            if out_be.ge(&exp_max) {
                return Self::INFINITY;
            }
            if out_be.ge(&Int::ONE) {
                return Self::from_biased_uint_parts(
                    false,
                    out_be.cast_unsigned(),
                    Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK,
                );
            }
            let drop_i = Int::<W>::ONE.sub(out_be);
            let mb_i = int_from_u64(Self::MB as u64);
            if drop_i.gt(&mb_i) {
                // Exact 0.5 * MIN_POSITIVE_SUBNORMAL (drop == MB + 1) rounds to 0 (ties-to-even).
                return Self::ZERO;
            }
            let drop = drop_i.cast_unsigned().cast_to_unsigned_float_exponent();
            let sub_m = Uint::<W>::ONE.shl(Self::MB - drop);
            return Self::from_biased_uint_parts(false, Uint::ONE, sub_m);
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let k_ext = ext_from_int(k, limbs);
            let f = ext_sub(&x, &k_ext, limbs);
            let r = ext_mul(&f, &const_ln2(limbs), limbs);
            let d = ext_expm1_small(&r, limbs);
            let m = ext_add(&ext_one(limbs), &d, limbs);
            let res = ext_scale_pow2_int(m, k);

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
