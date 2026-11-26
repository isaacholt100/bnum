use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, ext_add, ext_from_float, ext_mul, ext_round_to_float, ext_scale_pow2_int,
    ext_sqrt, int_from_u64, max_limbs, stage1_limbs,
};
use crate::{Exponent, Int};

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes the Euclidean hypotenuse `sqrt(self^2 + other^2)` without intermediate
    /// overflow or underflow, using rounding mode `RM`.
    pub(super) const fn hypot_internal(self, other: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        if self.is_infinite() || other.is_infinite() {
            return Self::INFINITY;
        }
        if self.is_nan() || other.is_nan() {
            return Self::NAN;
        }

        let mut a = self.abs();
        let mut b = other.abs();
        if a.to_bits().lt(&b.to_bits()) {
            let tmp = a;
            a = b;
            b = tmp;
        }
        if b.is_zero() {
            return a;
        }

        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        // Quick check: if a / b >= 2^(MB/2 + 3), then 0 < sqrt(a^2 + b^2) - a < 0.25 * ULP(a).
        let ea0 = ext_from_float(a, l1);
        let eb0 = ext_from_float(b, l1);
        let diff_exp = ea0.exp.sub(eb0.exp);
        if diff_exp.gt(&int_from_u64(((Self::MB / 2) + 3) as u64)) {
            return Self::round_with_tiny_delta(a, true);
        }

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let mut ea = ext_from_float(a, limbs);
            let mut eb = ext_from_float(b, limbs);
            let scale = ea.exp;
            ea.exp = Int::ZERO;
            eb.exp = eb.exp.sub(scale);

            let a2 = ext_mul(&ea, &ea, limbs);
            let b2 = ext_mul(&eb, &eb, limbs);
            let sum = ext_add(&a2, &b2, limbs);
            let root = ext_sqrt(&sum, limbs);
            let res = ext_scale_pow2_int(root, scale);

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
