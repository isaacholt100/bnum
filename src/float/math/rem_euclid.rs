use super::Float;
use crate::{Exponent, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes exact floating-point remainder `self % rhs` valid for all `W > 1`,
    /// `2 < MB < Self::BITS - 2` (including `EXPONENT_BITS >= 32` and `EXPONENT_BITS == 2`).
    pub(super) const fn rem_exact(self, rhs: Self) -> Self {
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => return Self::NAN,
            (FpCategory::Infinite, _) | (_, FpCategory::Zero) => return Self::NAN,
            (FpCategory::Zero, _) | (_, FpCategory::Infinite) => return self,
            _ => {}
        }

        let neg = self.is_sign_negative();
        match self.abs().to_bits().cmp(&rhs.abs().to_bits()) {
            Ordering::Less => return self,
            Ordering::Equal => {
                return if neg { Self::NEG_ZERO } else { Self::ZERO };
            }
            Ordering::Greater => {}
        }

        let (_, mut ex, mx) = self.into_biased_uint_parts();
        let (_, mut ey, my) = rhs.into_biased_uint_parts();

        // Normalize subnormals into virtual exponents (note we work with non-negative
        // exponent difference `d = (ex - shx) - (ey - shy)`).
        let shx = (Self::MB + 1) - mx.bit_width();
        let shy = (Self::MB + 1) - my.bit_width();
        let mut ux = if shx != 0 { mx.shl(shx) } else { mx };
        let uy = if shy != 0 { my.shl(shy) } else { my };

        // Signed exponent difference: since |self| > |rhs|, eff_ex >= eff_ey.
        let eff_ex = ex.cast_signed().sub(Uint::cast_from_unsigned_float_exponent(shx).cast_signed());
        let eff_ey = ey.cast_signed().sub(Uint::cast_from_unsigned_float_exponent(shy).cast_signed());
        let diff_u = eff_ex.sub(eff_ey).cast_unsigned();

        // Compute rem = (ux * 2^diff_u) mod uy.
        // For moderate exponent differences (<= 4096, covers all F16/F32/F64), simple
        // shift-subtract is very fast; for huge exponent differences (F128 / EXPONENT_BITS >= 32),
        // use O(log diff) modular exponentiation of 2^diff mod uy.
        let thresh = Uint::<W>::cast_from_unsigned_float_exponent(4096);
        let rem_u = if diff_u.le(&thresh) {
            let mut steps = diff_u.cast_to_unsigned_float_exponent();
            while steps > 0 {
                if ux.ge(&uy) {
                    ux = ux.sub(uy);
                    if ux.is_zero() {
                        return if neg { Self::NEG_ZERO } else { Self::ZERO };
                    }
                }
                ux = ux.shl(1);
                steps -= 1;
            }
            if ux.ge(&uy) {
                ux = ux.sub(uy);
            }
            ux
        } else {
            // Compute pow2 = 2^diff_u mod uy using widened Digits multiplication/division.
            let wid_uy = uy.to_digits::<u64>().grow::<1, W>();
            let mut pow2 = Uint::<W>::ONE;
            let mut base = Uint::<W>::from_le_bytes({
                let mut b = [0u8; W];
                b[0] = 2;
                b
            });
            // Ensure base < uy (always true since uy >= 2^MB >= 8).
            let bw = diff_u.bit_width();
            let mut bit: Exponent = 0;
            while bit < bw {
                if diff_u.bit(bit) {
                    let prod = pow2
                        .to_digits::<u64>()
                        .grow::<1, W>()
                        .long_mul::<true>(base.to_digits::<u64>().grow::<1, W>())
                        .0;
                    pow2 = match prod.cmp(&wid_uy) {
                        Ordering::Less => prod.remove_tail().to_integer(),
                        Ordering::Equal => Uint::ZERO,
                        Ordering::Greater => prod.div_rem_unchecked(wid_uy).1.remove_tail().to_integer(),
                    };
                }
                let sq = base
                    .to_digits::<u64>()
                    .grow::<1, W>()
                    .long_mul::<true>(base.to_digits::<u64>().grow::<1, W>())
                    .0;
                base = match sq.cmp(&wid_uy) {
                    Ordering::Less => sq.remove_tail().to_integer(),
                    Ordering::Equal => Uint::ZERO,
                    Ordering::Greater => sq.div_rem_unchecked(wid_uy).1.remove_tail().to_integer(),
                };
                bit += 1;
            }
            let prod = ux
                .to_digits::<u64>()
                .grow::<1, W>()
                .long_mul::<true>(pow2.to_digits::<u64>().grow::<1, W>())
                .0;
            match prod.cmp(&wid_uy) {
                Ordering::Less => prod.remove_tail().to_integer(),
                Ordering::Equal => Uint::ZERO,
                Ordering::Greater => prod.div_rem_unchecked(wid_uy).1.remove_tail().to_integer(),
            }
        };

        if rem_u.is_zero() {
            return if neg { Self::NEG_ZERO } else { Self::ZERO };
        }

        // Value is rem_u * 2^(eff_ey - bias - MB) where 0 < rem_u < uy < 2^(MB + 1).
        let _ = (&mut ex, &mut ey);
        let bw = rem_u.bit_width();
        let lead_shift = (Self::MB + 1) - bw;
        let out_e = eff_ey.sub(Uint::cast_from_unsigned_float_exponent(lead_shift).cast_signed());
        let one_i = crate::Int::<W>::ONE;
        if out_e.ge(&one_i) {
            let m = if lead_shift != 0 { rem_u.shl(lead_shift) } else { rem_u };
            Self::from_biased_uint_parts(neg, out_e.cast_unsigned(), m)
        } else {
            // Subnormal result: shift rem_u so exponent becomes 1.
            let target_shift_signed = eff_ey.sub(one_i);
            let m = if target_shift_signed.is_negative() {
                let rsh = target_shift_signed.wrapping_neg().cast_unsigned().cast_to_unsigned_float_exponent();
                rem_u.shr(rsh)
            } else {
                let lsh = target_shift_signed.cast_unsigned().cast_to_unsigned_float_exponent();
                if lsh != 0 { rem_u.shl(lsh) } else { rem_u }
            };
            Self::from_biased_uint_parts(neg, Uint::ONE, m)
        }
    }

    /// Euclidean remainder: least non-negative remainder of `self / rhs`, matching Rust std.
    pub(super) const fn rem_euclid_internal(self, rhs: Self) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let r = self.rem_exact(rhs);
        if r.lt(&Self::ZERO) {
            r.add(rhs.abs())
        } else {
            r
        }
    }
}
