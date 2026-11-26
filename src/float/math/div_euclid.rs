use super::Float;
use crate::Exponent;
use crate::Uint;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Truncates `self` toward zero without relying on `i32` `EXP_BIAS`.
    pub(super) const fn trunc_exact(self) -> Self {
        if self.is_nan() || self.is_infinite() || self.is_zero() {
            return self;
        }
        let (sign, biased_exp, mant) = self.into_biased_uint_parts();
        let bias = Self::exp_bias_uint();
        if biased_exp.lt(&bias) {
            return if sign { Self::NEG_ZERO } else { Self::ZERO };
        }
        let e = biased_exp.sub(bias);
        let mb_u = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB);
        if e.ge(&mb_u) {
            return self;
        }
        let frac_bits = Self::MB - e.cast_to_unsigned_float_exponent();
        let mask = Uint::<W>::MAX.shl(frac_bits);
        let trunc_mant = mant.bitand(mask);
        Self::from_biased_uint_parts(sign, biased_exp, trunc_mant)
    }

    /// Euclidean division matching Rust std `f32::div_euclid` / `f64::div_euclid`.
    pub(super) const fn div_euclid_internal(self, rhs: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let q = self.div(rhs).trunc_exact();
        if self.rem_exact(rhs).lt(&Self::ZERO) {
            if rhs.gt(&Self::ZERO) {
                q.sub(Self::one_val())
            } else {
                q.add(Self::one_val())
            }
        } else {
            q
        }
    }
}
