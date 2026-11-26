use super::Float;
use super::fixed::{
    ZIV_ERR_BITS, const_pi, const_pi_over_2, const_pi_over_4, ext_add, ext_atan, ext_div,
    ext_from_float, ext_mul, ext_neg, ext_round_to_float, ext_sub, ext_from_u64, int_from_u64,
    max_limbs, stage1_limbs,
};
use crate::Exponent;
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes the four-quadrant arctangent `atan2(self, other)` in `[-pi, pi]`,
    /// matching IEEE 754 / Rust std semantics with round-to-nearest-even.
    pub(super) const fn atan2_internal(self, other: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        if self.is_nan() || other.is_nan() {
            return Self::NAN;
        }

        let y_neg = self.is_sign_negative();
        let x_neg = other.is_sign_negative();
        let l1 = stage1_limbs::<W, MB>();
        let l2 = max_limbs::<W>();

        match (self.classify(), other.classify()) {
            (FpCategory::Zero, _) => {
                if !x_neg {
                    return if y_neg { Self::NEG_ZERO } else { Self::ZERO };
                }
                let mut res = const_pi(l2);
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            (_, FpCategory::Zero) => {
                let mut res = const_pi_over_2(l2);
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            (FpCategory::Infinite, FpCategory::Infinite) => {
                let pi4 = const_pi_over_4(l2);
                let mut res = if x_neg {
                    ext_mul(&pi4, &ext_from_u64(3, l2), l2)
                } else {
                    pi4
                };
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            (FpCategory::Infinite, _) => {
                let mut res = const_pi_over_2(l2);
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            (_, FpCategory::Infinite) => {
                if !x_neg {
                    return if y_neg { Self::NEG_ZERO } else { Self::ZERO };
                }
                let mut res = const_pi(l2);
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            _ => {}
        }

        // Both y (self) and x (other) are finite and non-zero.
        let abs_y = self.abs();
        let abs_x = other.abs();
        let ay0 = ext_from_float(abs_y, l1);
        let ax0 = ext_from_float(abs_x, l1);
        let thresh = int_from_u64((Self::MB + 64) as u64);

        // Guard extreme exponent gaps to prevent Int<W> overflow in ext_div on huge-exp formats.
        if let Some(d) = ay0.exp.checked_sub(ax0.exp) {
            if d.gt(&thresh) {
                let mut res = const_pi_over_2(l2);
                if y_neg {
                    res = ext_neg(res);
                }
                return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                    Some(v) => v,
                    None => unreachable!(),
                };
            }
            if d.lt(&thresh.wrapping_neg()) {
                if x_neg {
                    let mut res = const_pi(l2);
                    if y_neg {
                        res = ext_neg(res);
                    }
                    return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                        Some(v) => v,
                        None => unreachable!(),
                    };
                } else {
                    // |y/x| < 2^-(MB + 64): atan(y/x) = y/x - (y/x)^3/3 (strictly smaller magnitude).
                    let q = self.div(other);
                    if q.is_zero() {
                        return Self::underflow_nonzero(y_neg);
                    }
                    if q.mul(other).to_bits().eq(&self.to_bits()) {
                        return Self::round_with_tiny_delta(q, y_neg);
                    }
                    return q;
                }
            }
        } else if ay0.exp.gt(&ax0.exp) {
            let mut res = const_pi_over_2(l2);
            if y_neg {
                res = ext_neg(res);
            }
            return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                Some(v) => v,
                None => unreachable!(),
            };
        } else if x_neg {
            let mut res = const_pi(l2);
            if y_neg {
                res = ext_neg(res);
            }
            return match ext_round_to_float::<W, MB, RM>(&res, l2, ZIV_ERR_BITS, true) {
                Some(v) => v,
                None => unreachable!(),
            };
        } else {
            return Self::underflow_nonzero(y_neg);
        }

        let y_le_x = !matches!(abs_y.total_cmp(&abs_x), Ordering::Greater);

        let mut stage = 0;
        while stage < 2 {
            let limbs = if stage == 0 { l1 } else { l2 };
            let final_round = stage == 1 || l1 == l2;

            let ay = ext_from_float(abs_y, limbs);
            let ax = ext_from_float(abs_x, limbs);

            let mut res = if y_le_x {
                let t = ext_atan(&ext_div(&ay, &ax, limbs), limbs);
                if x_neg {
                    ext_sub(&const_pi(limbs), &t, limbs)
                } else {
                    t
                }
            } else {
                let t = ext_atan(&ext_div(&ax, &ay, limbs), limbs);
                if x_neg {
                    ext_add(&const_pi_over_2(limbs), &t, limbs)
                } else {
                    ext_sub(&const_pi_over_2(limbs), &t, limbs)
                }
            };
            res.sign = y_neg;

            if let Some(out) = ext_round_to_float::<W, MB, RM>(&res, limbs, ZIV_ERR_BITS, final_round) {
                return out;
            }
            stage += 1;
        }
        unreachable!()
    }
}
