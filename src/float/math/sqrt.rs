use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_from_float, ext_round_to_float, ext_sqrt, int_from_u64, stage1_limbs,
    uint_low_u64,
};
use crate::float::RoundingMode;
use crate::{Exponent, Int, Uint};
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes `sqrt(self)` using rounding mode `RM`, valid for all `W > 1` and
    /// `2 < MB < Self::BITS - 2` (including `EXPONENT_BITS == 2` and `EXPONENT_BITS >= 32`).
    pub(super) const fn sqrt_internal(self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        match self.classify() {
            FpCategory::Nan => return self,
            FpCategory::Zero => return self,
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

        // For Stochastic mode, use extended-precision Newton sqrt so full 64-bit fractional
        // distance drives the rounding probability.
        if RM == RoundingMode::Stochastic as u8 {
            let l1 = stage1_limbs::<W, MB>();
            let r = ext_sqrt(&ext_from_float(self, l1), l1);
            return match ext_round_to_float::<W, MB, RM>(&r, l1, ZIV_ERR_BITS, true) {
                Some(v) => v,
                None => unreachable!(),
            };
        }

        // Exact bit-by-bit square root producing exact (quotient, round_bit, remainder != 0).
        let (_, biased_exp, mant) = self.into_biased_uint_parts();
        let bias = Self::exp_bias_uint().cast_signed();
        let bw = mant.bit_width();
        let norm_shift = (Self::MB + 1) - bw;
        let mut ix = if norm_shift != 0 {
            mant.shl(norm_shift)
        } else {
            mant
        };
        let e = biased_exp
            .cast_signed()
            .sub(bias)
            .sub(int_from_u64(norm_shift as u64));

        // Make unbiased exponent even: |self| = (ix / 2^MB) * 2^e with ix in [2^MB, 2^(MB+2)).
        let e_odd = (e.bytes[0] & 1) != 0;
        let half_e = if e_odd {
            ix = ix.shl(1);
            e.sub(Int::ONE).shr(1)
        } else {
            e.shr(1)
        };

        // Determine target precision `s` (normal: MB; subnormal when EXPONENT_BITS is tiny: < MB).
        let min_normal_exp = Int::<W>::ONE.sub(bias);
        let (is_subnormal, s): (bool, Exponent) = if half_e.lt(&min_normal_exp) {
            let diff = min_normal_exp
                .sub(half_e)
                .cast_unsigned()
                .cast_to_unsigned_float_exponent();
            (true, Self::MB - diff)
        } else {
            (false, Self::MB)
        };

        // Generate `s + 2` bits of sqrt: top `s + 1` bits form `y`, lowest bit is `round_bit`.
        // Track a 1-bit overflow carry `rem_hi` for `rem << 1` so this works even at the extreme
        // boundary `MB == BITS - 3` (e.g. `Float<2, 13>`) where `rem` reaches bit `BITS - 1`.
        let mut rem_hi = false;
        let mut rem = ix.shl(1);
        let mut q = Uint::<W>::ZERO;
        let mut s_acc = Uint::<W>::ZERO;
        let mut r = Uint::<W>::ONE.shl(Self::MB + 1);
        let steps = s + 2;
        let mut step: Exponent = 0;
        while step < steps {
            let t = s_acc.add(r);
            if rem_hi || t.le(&rem) {
                let (new_rem, _) = rem.overflowing_sub(t);
                rem = new_rem;
                s_acc = t.add(r);
                q = q.add(r);
            }
            rem_hi = rem.bit(Self::BITS - 1);
            rem = rem.shl(1);
            r = r.shr(1);
            step += 1;
        }

        // Shift `q` down so its lowest bit is `round_bit` and `y = q >> 1` has `s + 1` bits.
        let tail_bits = Self::MB - s;
        let q_aligned = if tail_bits != 0 { q.shr(tail_bits) } else { q };
        let round_bit = q_aligned.bit(0);
        // If `round_bit` is 1, `(2y + 1)^2` is odd while `4 * m * 2^(2s - MB)` is even (2s >= MB),
        // so remainder is strictly positive (`rem != 0` always when `round_bit == true`).
        let sticky = rem_hi || !rem.is_zero() || round_bit;
        let mut y = q_aligned.shr(1);

        let seed = uint_low_u64(&half_e.cast_unsigned()) ^ uint_low_u64(&y);
        if RoundingMode::should_round_up::<RM>(false, y.is_odd(), round_bit, sticky, 0, seed) {
            y = y.add(Uint::ONE);
        }

        if is_subnormal {
            Self::from_biased_uint_parts(false, Uint::ONE, y)
        } else {
            let mut out_be = half_e.add(bias);
            if y.bit(Self::MB + 1) {
                y = y.shr(1);
                out_be = out_be.add(Int::ONE);
            }
            Self::from_biased_uint_parts(false, out_be.cast_unsigned(), y)
        }
    }
}
