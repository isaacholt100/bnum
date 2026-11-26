use super::Float;
use super::fixed::{
    Ext, ZIV_ERR_BITS, const_ln2, ext_add, ext_div, ext_from_float, ext_from_int, ext_ln,
    ext_round_to_float, int_from_i64, int_from_u64, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Computes `log2(self)` with IEEE 754 round-to-nearest-even via two-stage Ziv rounding.
    pub(super) const fn log2_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        match self.classify() {
            FpCategory::Nan => return Self::NAN,
            FpCategory::Zero => return Self::NEG_INFINITY,
            FpCategory::Infinite => {
                return if self.is_sign_negative() {
                    Self::NAN
                } else {
                    Self::INFINITY
                };
            }
            _ => {}
        }

        if self.is_sign_negative() {
            return Self::NAN;
        }

        let one = Self::one_val();
        if self.to_bits().eq(&one.to_bits()) {
            return Self::ZERO;
        }

        // Exact power-of-two fast path: log2(2^e) = e exactly.
        let (_, biased_exp, mant) = self.into_biased_uint_parts();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();
        if mant.is_power_of_two() {
            let bias = Self::exp_bias_uint().cast_signed();
            let bw = mant.bit_width();
            let e = biased_exp
                .cast_signed()
                .sub(bias)
                .sub(int_from_u64(((Self::MB + 1) - bw) as u64));
            let e_ext = ext_from_int(e, l2);
            return match ext_round_to_float::<W, MB>(&e_ext, l2, 0, true) {
                Some(v) => v,
                None => unreachable!(),
            };
        }

        // Center x = M' * 2^k with M' in (1/sqrt(2), sqrt(2)] so log2(x) = k + ln(M') / ln(2),
        // avoiding multiplying and dividing `k` by `ln(2)`.
        const SQRT2_TOP64: u64 = 0xb504_f333_f9de_6484;

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let x = ext_from_float(self, limbs);
            let (m_exp, k) = if x.sig.get(limbs - 1) > SQRT2_TOP64 {
                (int_from_i64(-1), x.exp.add(Int::ONE))
            } else {
                (Int::ZERO, x.exp)
            };
            let m = Ext {
                sign: false,
                zero: false,
                exp: m_exp,
                sig: x.sig,
            };
            let frac = ext_div(&ext_ln(&m, limbs), &const_ln2(limbs), limbs);
            let res = if k.is_zero() {
                frac
            } else {
                ext_add(&ext_from_int(k, limbs), &frac, limbs)
            };

            if let Some(out) = ext_round_to_float::<W, MB>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
