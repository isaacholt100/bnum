use super::Float;
use crate::{Exponent, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Adds the magnitudes of two finite non-zero floats with IEEE 754 round-to-nearest-even
    /// and applies `negative` as the result sign.
    #[inline]
    pub(crate) const fn add_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
        }

        let (a, b) = match self.abs().to_bits().cmp(&rhs.abs().to_bits()) {
            Ordering::Less => (rhs, self),
            _ => (self, rhs),
        };
        let (_, ea, ma) = a.into_biased_uint_parts();
        let (_, eb, mb) = b.into_biased_uint_parts();

        let exp_max = Self::exp_max_uint();
        let diff = ea.sub(eb);

        let (mut res_exp, mut res_mant) = if diff.is_zero() {
            let sum = ma.add(mb);
            if sum.bit(Self::MB + 1) {
                let half = sum.bit(0);
                let mut m = sum.shr(1);
                if half && m.is_odd() {
                    m = m.add(Uint::ONE);
                }
                let mut e = ea.add(Uint::ONE);
                if m.bit(Self::MB + 1) {
                    m = m.shr(1);
                    e = e.add(Uint::ONE);
                }
                (e, m)
            } else {
                (ea, sum)
            }
        } else {
            let max_shift = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 2);
            if diff.gt(&max_shift) {
                (ea, ma)
            } else {
                let d = diff.cast_to_unsigned_float_exponent() as Exponent;
                let q = mb.shr(d);
                let round_bit = mb.bit(d - 1);
                let sticky = mb.trailing_zeros() < d - 1;

                let sum = ma.add(q);
                if sum.bit(Self::MB + 1) {
                    let new_round = sum.bit(0);
                    let new_sticky = round_bit || sticky;
                    let mut m = sum.shr(1);
                    let mut e = ea.add(Uint::ONE);
                    if new_round && (new_sticky || m.is_odd()) {
                        m = m.add(Uint::ONE);
                        if m.bit(Self::MB + 1) {
                            m = m.shr(1);
                            e = e.add(Uint::ONE);
                        }
                    }
                    (e, m)
                } else {
                    let mut m = sum;
                    let mut e = ea;
                    if round_bit && (sticky || m.is_odd()) {
                        m = m.add(Uint::ONE);
                        if m.bit(Self::MB + 1) {
                            m = m.shr(1);
                            e = e.add(Uint::ONE);
                        }
                    }
                    (e, m)
                }
            }
        };

        if res_exp.ge(&exp_max) {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        // Prevent unused-mut lint on res_exp/res_mant if needed
        let _ = (&mut res_exp, &mut res_mant);
        Self::from_biased_uint_parts(negative, res_exp, res_mant)
    }

    #[inline]
    pub(crate) const fn add(self, rhs: Self) -> Self {
        let self_negative = self.is_sign_negative();
        let rhs_negative = rhs.is_sign_negative();

        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                if self_negative != rhs_negative {
                    Self::NAN
                } else {
                    self
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs,
            (FpCategory::Zero, FpCategory::Zero) => {
                if self_negative && rhs_negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (FpCategory::Zero, _) => rhs,
            (_, FpCategory::Zero) => self,
            (_, _) => {
                if self_negative != rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    self.add_internal(rhs, self_negative)
                }
            }
        }
    }
}
