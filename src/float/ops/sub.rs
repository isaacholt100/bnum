use super::Float;
use crate::{Exponent, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Subtracts the magnitudes of two finite non-zero floats: `(-1)^negative * (|self| - |rhs|)`
    /// with IEEE 754 round-to-nearest-even.
    #[inline]
    pub(crate) const fn sub_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let (a, b, negative) = match self.abs().to_bits().cmp(&rhs.abs().to_bits()) {
            Ordering::Equal => return Self::ZERO,
            Ordering::Less => (rhs, self, !negative),
            Ordering::Greater => (self, rhs, negative),
        };

        let (_, ea, ma) = a.into_biased_uint_parts();
        let (_, eb, mb) = b.into_biased_uint_parts();
        let diff = ea.sub(eb);

        if diff.is_zero() {
            let mut m = ma.sub(mb);
            if ea.eq(&Uint::ONE) {
                return Self::from_biased_uint_parts(negative, Uint::ONE, m);
            }
            let lead_shift = (Self::MB + 1) - m.bit_width();
            let max_drop = ea.sub(Uint::ONE);
            let lead_u = Uint::<W>::cast_from_unsigned_float_exponent(lead_shift);
            let shift = if max_drop.lt(&lead_u) {
                max_drop.cast_to_unsigned_float_exponent() as Exponent
            } else {
                lead_shift
            };
            if shift != 0 {
                m = m.shl(shift);
            }
            let res_exp = ea.sub(Uint::cast_from_unsigned_float_exponent(shift));
            return Self::from_biased_uint_parts(negative, res_exp, m);
        }

        if diff.eq(&Uint::ONE) {
            // Exact difference scaled by 2: m2 = 2*ma - mb (fits in MB + 2 < BITS bits).
            let m2 = ma.shl(1).sub(mb);
            if m2.bit(Self::MB + 1) {
                let half = m2.bit(0);
                let mut m = m2.shr(1);
                if half && m.is_odd() {
                    m = m.add(Uint::ONE);
                }
                return Self::from_biased_uint_parts(negative, ea, m);
            }
            // Exact cancellation at exponent ea - 1 >= 1; normalize as far as allowed.
            let e1 = ea.sub(Uint::ONE);
            let lead_shift = (Self::MB + 1) - m2.bit_width();
            let max_drop = e1.sub(Uint::ONE);
            let lead_u = Uint::<W>::cast_from_unsigned_float_exponent(lead_shift);
            let shift = if max_drop.lt(&lead_u) {
                max_drop.cast_to_unsigned_float_exponent() as Exponent
            } else {
                lead_shift
            };
            let m = if shift != 0 { m2.shl(shift) } else { m2 };
            let res_exp = e1.sub(Uint::cast_from_unsigned_float_exponent(shift));
            return Self::from_biased_uint_parts(negative, res_exp, m);
        }

        // diff >= 2: since a is normal (ea >= 3, ma >= 2^MB) and mb * 2^(-diff) < 2^(MB - 1),
        // the exact difference is in (2^(MB - 1), 2^(MB + 1)) and stays strictly normal.
        // Scale by 4 (<< 2) — fits in MB + 3 <= BITS bits — with a single sticky bit below 2^(-2).
        let max_shift = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 3);
        let (b_int, sticky) = if diff.gt(&max_shift) {
            (Uint::<W>::ZERO, true)
        } else {
            let d = diff.cast_to_unsigned_float_exponent() as Exponent;
            if d == 2 {
                (mb, false)
            } else {
                let shift = d - 2;
                (mb.shr(shift), mb.trailing_zeros() < shift)
            }
        };

        // True scaled value is (diff4 + frac), where frac = 0 if !sticky else 0 < frac < 1.
        let mut diff4 = ma.shl(2).sub(b_int);
        if sticky {
            diff4 = diff4.sub(Uint::ONE);
        }

        let (mut res_exp, mut res_mant) = if diff4.bit(Self::MB + 2) {
            let mut m = diff4.shr(2);
            let round_bit = diff4.bit(1);
            let lower_sticky = diff4.bit(0) || sticky;
            if round_bit && (lower_sticky || m.is_odd()) {
                m = m.add(Uint::ONE);
            }
            (ea, m)
        } else {
            let mut m = diff4.shr(1);
            let mut e = ea.sub(Uint::ONE);
            let round_bit = diff4.bit(0);
            if round_bit && (sticky || m.is_odd()) {
                m = m.add(Uint::ONE);
                if m.bit(Self::MB + 1) {
                    m = m.shr(1);
                    e = e.add(Uint::ONE);
                }
            }
            (e, m)
        };

        let _ = (&mut res_exp, &mut res_mant);
        Self::from_biased_uint_parts(negative, res_exp, res_mant)
    }

    #[inline]
    pub(crate) const fn sub(self, rhs: Self) -> Self {
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                match (self.is_sign_negative(), rhs.is_sign_negative()) {
                    (true, false) => Self::NEG_INFINITY,
                    (false, true) => Self::INFINITY,
                    _ => Self::NAN,
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs.neg(),
            (FpCategory::Zero, FpCategory::Zero) => {
                if self.is_sign_negative() && rhs.is_sign_positive() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (FpCategory::Zero, _) => rhs.neg(),
            (_, FpCategory::Zero) => self,
            (_, _) => {
                let self_negative = self.is_sign_negative();
                let rhs_negative = rhs.is_sign_negative();
                if self_negative != rhs_negative {
                    self.add_internal(rhs, self_negative)
                } else {
                    self.sub_internal(rhs, self_negative)
                }
            }
        }
    }
}
