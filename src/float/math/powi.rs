use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_from_float, ext_mul, ext_one, ext_recip, ext_round_to_float, int_from_u64,
    max_limbs, stage1_limbs,
};
use crate::{Exponent, Int, Uint};
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Raises `self` to an integer power `n`, matching IEEE 754 / Rust std semantics
    /// with extended-precision binary exponentiation and two-stage Ziv rounding.
    pub(super) const fn powi_internal(self, n: i32) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        if n == 0 {
            return Self::one_val();
        }
        if self.is_nan() {
            return Self::NAN;
        }
        if n == 1 {
            return self;
        }
        if n == -1 {
            return Self::one_val().div(self);
        }

        let n_neg = n < 0;
        let n_abs = n.unsigned_abs();
        let res_neg = self.is_sign_negative() && (n_abs & 1 == 1);

        match self.classify() {
            FpCategory::Zero => {
                return if n_neg {
                    if res_neg {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    }
                } else if res_neg {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            FpCategory::Infinite => {
                return if n_neg {
                    if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                } else if res_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        let abs_self = self.abs();
        let one = Self::one_val();
        if abs_self.to_bits().eq(&one.to_bits()) {
            return if res_neg { Self::neg_one_val() } else { one };
        }

        // Exact fast path for powers of two (single bit in significand).
        let (_, biased_exp, mant) = abs_self.into_biased_uint_parts();
        if mant.is_power_of_two() {
            let bias = Self::exp_bias_uint().cast_signed();
            let exp_max = Self::exp_max_uint().cast_signed();
            let bw = mant.bit_width();
            let e_unbiased: Int<W> = biased_exp
                .cast_signed()
                .sub(bias)
                .sub(int_from_u64(((Self::MB + 1) - bw) as u64));

            // Compute e_unbiased * n with overflow/underflow saturation.
            let e_neg = e_unbiased.is_negative();
            let e_mag = if e_neg {
                e_unbiased.wrapping_neg().cast_unsigned()
            } else {
                e_unbiased.cast_unsigned()
            };
            let prod_neg = e_neg ^ n_neg;
            let limit = exp_max.add(int_from_u64((Self::MB + 4) as u64)).cast_unsigned();
            // Multiply e_mag * n_abs via widened digits to avoid overflow on small W.
            let wid_e = e_mag.to_digits::<u128>().grow::<1, W>();
            let wid_n = Uint::<W>::cast_from_unsigned_float_exponent(n_abs)
                .to_digits::<u128>()
                .grow::<1, W>();
            let wid_prod = wid_e.long_mul::<true>(wid_n).0;
            let wid_limit = limit.to_digits::<u128>().grow::<1, W>();
            if wid_prod.cmp(&wid_limit).is_gt() {
                return if prod_neg {
                    if res_neg { Self::NEG_ZERO } else { Self::ZERO }
                } else if res_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            let p_u: Uint<W> = wid_prod.remove_tail().to_integer();
            let total_e = if prod_neg {
                p_u.cast_signed().wrapping_neg()
            } else {
                p_u.cast_signed()
            };
            let out_be = total_e.add(bias);
            if out_be.ge(&exp_max) {
                return if res_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            if out_be.ge(&Int::ONE) {
                return Self::from_biased_uint_parts(
                    res_neg,
                    out_be.cast_unsigned(),
                    Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK,
                );
            }
            let drop_i = Int::<W>::ONE.sub(out_be);
            let mb_i = int_from_u64(Self::MB as u64);
            if drop_i.gt(&mb_i) {
                return if res_neg { Self::NEG_ZERO } else { Self::ZERO };
            }
            let drop = drop_i.cast_unsigned().cast_to_unsigned_float_exponent();
            let sub_m = Uint::<W>::ONE.shl(Self::MB - drop);
            return Self::from_biased_uint_parts(res_neg, Uint::ONE, sub_m);
        }

        // Exact fast path for n == 2 (single hardware-exact multiplication).
        if n == 2 {
            return self.mul_internal(self, false);
        }

        // Two-stage Ziv binary exponentiation in Ext<W>.
        let exp_max = Self::exp_max_uint().cast_signed();
        let limit = exp_max.add(int_from_u64((Self::MB + 64) as u64));
        let neg_limit = limit.wrapping_neg();
        let gt_one = abs_self.gt(&one);

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let mut base = ext_from_float(abs_self, limbs);
            let mut acc = ext_one(limbs);
            let mut k = n_abs;
            let mut saturated = false;

            while k > 0 {
                if (k & 1) != 0 {
                    acc = ext_mul(&acc, &base, limbs);
                    if (gt_one && acc.exp.gt(&limit)) || (!gt_one && acc.exp.lt(&neg_limit)) {
                        saturated = true;
                        break;
                    }
                }
                k >>= 1;
                if k > 0 {
                    base = ext_mul(&base, &base, limbs);
                    if (gt_one && base.exp.gt(&limit)) || (!gt_one && base.exp.lt(&neg_limit)) {
                        saturated = true;
                        break;
                    }
                }
            }

            if saturated {
                let overflows = gt_one ^ n_neg;
                return if overflows {
                    if res_neg {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    }
                } else if res_neg {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }

            let mut res = if n_neg { ext_recip(&acc, limbs) } else { acc };
            res.sign = res_neg;
            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
