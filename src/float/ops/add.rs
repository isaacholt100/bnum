use super::Float;
use crate::float::RoundingMode;
use crate::{Exponent, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

#[inline]
const fn uint_low_u64<const W: usize>(u: &Uint<W>) -> u64 {
    let mut v = 0u64;
    let mut i = 0;
    while i < W && i < 8 {
        v |= (u.bytes[i] as u64) << (i * 8);
        i += 1;
    }
    v
}

#[inline]
const fn uint_frac64<const W: usize>(u: &Uint<W>, shift: Exponent) -> u64 {
    if shift == 0 {
        0
    } else if shift <= 64 {
        let lo = uint_low_u64(u);
        if shift == 64 {
            lo
        } else {
            (lo & ((1u64 << shift) - 1)) << (64 - shift)
        }
    } else {
        uint_low_u64(&u.shr(shift - 64))
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Adds the magnitudes of two finite non-zero floats using `RM` and applies `negative`
    /// as the result sign.
    #[inline]
    pub(crate) const fn add_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        let (a, b) = match self.abs().to_bits().cmp(&rhs.abs().to_bits()) {
            Ordering::Less => (rhs, self),
            _ => (self, rhs),
        };
        let (_, ea, ma) = a.into_biased_uint_parts();
        let (_, eb, mb) = b.into_biased_uint_parts();

        let exp_max = Self::exp_max_uint();
        let diff = ea.sub(eb);
        let seed_base = uint_low_u64(&ea).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&eb).wrapping_mul(0x517c_c1b7_2722_0a95)
            ^ uint_low_u64(&mb);

        let (res_exp, res_mant) = if diff.is_zero() {
            let sum = ma.add(mb);
            if sum.bit(Self::MB + 1) {
                let half = sum.bit(0);
                let mut m = sum.shr(1);
                let frac64 = if half { 1u64 << 63 } else { 0 };
                if RoundingMode::should_round_up::<RM>(
                    negative,
                    m.is_odd(),
                    half,
                    false,
                    frac64,
                    seed_base ^ uint_low_u64(&m),
                ) {
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
                let a_signed = if negative { a.abs().neg() } else { a.abs() };
                if RM == RoundingMode::Stochastic as u8 {
                    let stoch_limit = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 65);
                    if diff.lt(&stoch_limit) {
                        let d = diff.cast_to_unsigned_float_exponent() as Exponent;
                        let frac64 = if d <= 64 {
                            uint_low_u64(&mb) << (64 - d)
                        } else {
                            uint_low_u64(&mb.shr(d - 64))
                        };
                        let seed = seed_base ^ uint_low_u64(&ma);
                        if RoundingMode::should_round_up::<RM>(
                            negative,
                            ma.is_odd(),
                            false,
                            true,
                            frac64,
                            seed,
                        ) {
                            let abs_bits = a_signed.abs().to_bits();
                            if abs_bits.eq(&Self::MAX.to_bits()) {
                                return Self::overflow_value(negative);
                            }
                            return Self::from_bits(a_signed.to_bits().add(Uint::ONE));
                        }
                    }
                    return a_signed;
                }
                return Self::round_with_tiny_delta(a_signed, !negative);
            } else {
                let d = diff.cast_to_unsigned_float_exponent() as Exponent;
                let q = mb.shr(d);
                let round_bit = mb.bit(d - 1);
                let sticky = mb.trailing_zeros() < d - 1;
                let frac64_b = if RM == RoundingMode::Stochastic as u8 {
                    uint_frac64(&mb, d)
                } else {
                    0
                };

                let sum = ma.add(q);
                if sum.bit(Self::MB + 1) {
                    let new_round = sum.bit(0);
                    let new_sticky = round_bit || sticky;
                    let new_frac64 = if RM == RoundingMode::Stochastic as u8 {
                        ((new_round as u64) << 63) | (frac64_b >> 1)
                    } else {
                        0
                    };
                    let mut m = sum.shr(1);
                    let mut e = ea.add(Uint::ONE);
                    if RoundingMode::should_round_up::<RM>(
                        negative,
                        m.is_odd(),
                        new_round,
                        new_sticky,
                        new_frac64,
                        seed_base ^ uint_low_u64(&m),
                    ) {
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
                    if RoundingMode::should_round_up::<RM>(
                        negative,
                        m.is_odd(),
                        round_bit,
                        sticky,
                        frac64_b,
                        seed_base ^ uint_low_u64(&m),
                    ) {
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
            return Self::overflow_value(negative);
        }

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
                } else if self_negative != rhs_negative {
                    Self::zero_from_cancellation()
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
