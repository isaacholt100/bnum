use super::Float;
use super::fixed::{
    Fixed, fp_add, fp_bit, fp_bit_width, fp_cmp, fp_from_uint_shl, fp_has_low_bits, fp_shl,
    fp_shr, fp_shr_to_uint, fp_sub, int_from_i64, int_from_u64, max_limbs, uint_low_u64,
};
use crate::{Exponent, Int, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Fused multiply-add: computes `(self * a) + b` with a single IEEE 754
    /// round-to-nearest-even rounding.
    pub(super) const fn mul_add_internal(self, a: Self, b: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let prod_neg = self.is_sign_negative() ^ a.is_sign_negative();
        let b_neg = b.is_sign_negative();

        match (self.classify(), a.classify(), b.classify()) {
            (FpCategory::Nan, _, _) | (_, FpCategory::Nan, _) | (_, _, FpCategory::Nan) => {
                return Self::NAN;
            }
            (FpCategory::Infinite, FpCategory::Zero, _)
            | (FpCategory::Zero, FpCategory::Infinite, _) => {
                return Self::NAN;
            }
            (FpCategory::Infinite, _, FpCategory::Infinite)
            | (_, FpCategory::Infinite, FpCategory::Infinite) => {
                return if prod_neg != b_neg {
                    Self::NAN
                } else if prod_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            (FpCategory::Infinite, _, _) | (_, FpCategory::Infinite, _) => {
                return if prod_neg {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                };
            }
            (_, _, FpCategory::Infinite) => return b,
            (FpCategory::Zero, _, FpCategory::Zero) | (_, FpCategory::Zero, FpCategory::Zero) => {
                return if prod_neg && b_neg {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            (FpCategory::Zero, _, _) | (_, FpCategory::Zero, _) => return b,
            (_, _, FpCategory::Zero) => return self.mul_internal(a, prod_neg),
            _ => {}
        }

        // All three inputs are finite and non-zero.
        let (_, e_self, m_self) = self.into_biased_uint_parts();
        let (_, e_a, m_a) = a.into_biased_uint_parts();
        let (_, e_b, m_b) = b.into_biased_uint_parts();

        let bias = Self::exp_bias_uint().cast_signed();
        let exp_max = Self::exp_max_uint().cast_signed();
        let mb_int = int_from_u64(Self::MB as u64);

        // Compute exact product m_self * m_a into Fixed<W> (uses max_limbs >= 2*MB + 136 bits).
        let limbs = max_limbs::<W>();
        let p_bits = (limbs as Exponent) * 64;

        let u_limbs = W.div_ceil(8);
        let d_self = m_self.as_digits::<u64>();
        let d_a = m_a.as_digits::<u64>();
        let mut prod_fp = Fixed::<W>::ALL_ZEROS;
        let mut i = 0;
        while i < u_limbs {
            let si = d_self.get(i) as u128;
            if si != 0 {
                let mut carry: u128 = 0;
                let mut j = 0;
                while j < u_limbs {
                    let idx = i + j;
                    let sum = si * (d_a.get(j) as u128) + (prod_fp.get(idx) as u128) + carry;
                    prod_fp.set(idx, sum as u64);
                    carry = sum >> 64;
                    j += 1;
                }
                let mut k = i + u_limbs;
                while carry != 0 && k < limbs {
                    let sum = (prod_fp.get(k) as u128) + carry;
                    prod_fp.set(k, sum as u64);
                    carry = sum >> 64;
                    k += 1;
                }
            }
            i += 1;
        }

        let bw_p = fp_bit_width(&prod_fp, limbs);
        // Exact value of product: prod_fp * 2^ep where ep = (e_self - bias) + (e_a - bias) - 2*MB.
        // Top bit of product has unbiased exponent top_p = ep + bw_p - 1.
        let ep = e_self
            .cast_signed()
            .sub(bias)
            .add(e_a.cast_signed().sub(bias))
            .sub(mb_int)
            .sub(mb_int);
        let top_p = ep.add(int_from_u64((bw_p - 1) as u64));

        // Exact value of b: m_b * 2^eb_raw where eb_raw = (e_b - bias) - MB.
        let bw_b = m_b.bit_width();
        let eb_raw = e_b.cast_signed().sub(bias).sub(mb_int);
        let top_b = eb_raw.add(int_from_u64((bw_b - 1) as u64));

        // Align both magnitudes with their MSBs at bit `p_bits - 2` (leaving 1 bit for add carry).
        let p_aligned = fp_shl(&prod_fp, (p_bits - 2) - (bw_p - 1), limbs);
        let b_aligned = fp_from_uint_shl(&m_b, (p_bits - 2) - (bw_b - 1), limbs);

        let p_ge_b = match top_p.cmp(&top_b) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => !matches!(fp_cmp(&p_aligned, &b_aligned, limbs), Ordering::Less),
        };

        let (hi_fp, lo_fp, hi_top, lo_top, res_neg) = if p_ge_b {
            (p_aligned, b_aligned, top_p, top_b, prod_neg)
        } else {
            (b_aligned, p_aligned, top_b, top_p, b_neg)
        };

        let same_sign = prod_neg == b_neg;
        let diff_exp = hi_top.sub(lo_top);
        let max_align = int_from_u64((p_bits - 2) as u64);

        let (lo_shifted, sticky_lo) = if diff_exp.gt(&max_align) {
            (Fixed::<W>::ALL_ZEROS, true)
        } else {
            let d = uint_low_u64(&diff_exp.cast_unsigned()) as Exponent;
            if d == 0 {
                (lo_fp, false)
            } else {
                (fp_shr(&lo_fp, d, limbs), fp_has_low_bits(&lo_fp, d))
            }
        };

        // Combine hi_fp and (lo_shifted + eps), where eps = 0 if !sticky_lo else 0 < eps < 1 ULP of Fixed.
        // Because p_bits >= 2*MB + 136, whenever diff_exp <= 1 both terms fit 100% exactly
        // (sticky_lo == false), and whenever diff_exp >= 2 bit 0 is > 130 bits below the rounding bit.
        let (res_fp, res_top, extra_sticky) = if same_sign {
            let (sum, _) = fp_add(&hi_fp, &lo_shifted, limbs);
            if fp_bit(&sum, p_bits - 1) {
                (sum, hi_top.add(Int::ONE), sticky_lo)
            } else {
                (fp_shl(&sum, 1, limbs), hi_top, sticky_lo)
            }
        } else {
            let (mut diff, _) = fp_sub(&hi_fp, &lo_shifted, limbs);
            if sticky_lo {
                let mut one_fp = Fixed::<W>::ALL_ZEROS;
                one_fp.set(0, 1);
                diff = fp_sub(&diff, &one_fp, limbs).0;
            }
            let bw = fp_bit_width(&diff, limbs);
            if bw == 0 {
                debug_assert!(!sticky_lo);
                return Self::ZERO;
            }
            // Bit `p_bits - 2` corresponded to `hi_top`, so bit `bw - 1` corresponds to:
            let drop = (p_bits - 2) - (bw - 1);
            let norm = fp_shl(&diff, p_bits - bw, limbs);
            (norm, hi_top.sub(int_from_u64(drop as u64)), sticky_lo)
        };

        // Now exact magnitude is `(res_fp + frac) * 2^(res_top - (p_bits - 1))`
        // where `res_fp` has bit `p_bits - 1` set and `frac == 0` iff `!extra_sticky` (else `0 < frac < 1`).
        let be = res_top.add(bias);
        if be.ge(&exp_max) {
            return if res_neg {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let neg_mb = int_from_i64(-(Self::MB as i64));
        if be.lt(&neg_mb) {
            return if res_neg { Self::NEG_ZERO } else { Self::ZERO };
        }
        if be.eq(&neg_mb) {
            // Value in [0.5, 1.0) ULPs of MIN_POSITIVE_SUBNORMAL.
            let gt_half = extra_sticky || fp_has_low_bits(&res_fp, p_bits - 1);
            return if gt_half {
                Self::from_biased_uint_parts(res_neg, Uint::ONE, Uint::ONE)
            } else if res_neg {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }

        let (mut out_be, sub_drop): (Int<W>, Exponent) = if be.ge(&Int::ONE) {
            (be, 0)
        } else {
            let drop = uint_low_u64(&Int::<W>::ONE.sub(be).cast_unsigned()) as Exponent;
            (Int::ONE, drop)
        };

        let keep_bits = (Self::MB + 1) - sub_drop;
        let shift = p_bits - keep_bits;
        let mut q = fp_shr_to_uint(&res_fp, shift, limbs);
        let round_bit = fp_bit(&res_fp, shift - 1);
        let sticky = extra_sticky || fp_has_low_bits(&res_fp, shift - 1);

        if round_bit && (sticky || q.is_odd()) {
            q = q.add(Uint::ONE);
            if q.bit(Self::MB + 1) {
                q = q.shr(1);
                out_be = out_be.add(Int::ONE);
            }
        }

        if out_be.ge(&exp_max) {
            return if res_neg {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        Self::from_biased_uint_parts(res_neg, out_be.cast_unsigned(), q)
    }
}
