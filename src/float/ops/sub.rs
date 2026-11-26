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
    /// Subtracts the magnitudes of two finite non-zero floats: `(-1)^negative * (|self| - |rhs|)`
    /// using rounding mode `RM`.
    #[inline]
    pub(crate) const fn sub_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        let (a, b, negative) = match self.abs().to_bits().cmp(&rhs.abs().to_bits()) {
            Ordering::Equal => return Self::zero_from_cancellation(),
            Ordering::Less => (rhs, self, !negative),
            Ordering::Greater => (self, rhs, negative),
        };

        let (_, ea, ma) = a.into_biased_uint_parts();
        let (_, eb, mb) = b.into_biased_uint_parts();
        let diff = ea.sub(eb);
        let seed_base = uint_low_u64(&ea).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&eb).wrapping_mul(0x517c_c1b7_2722_0a95)
            ^ uint_low_u64(&mb);

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
            let m2 = ma.shl(1).sub(mb);
            if m2.bit(Self::MB + 1) {
                let half = m2.bit(0);
                let mut m = m2.shr(1);
                let frac64 = if half { 1u64 << 63 } else { 0 };
                let mut e = ea;
                if RoundingMode::should_round_up::<RM>(
                    negative,
                    m.is_odd(),
                    half,
                    false,
                    frac64,
                    seed_base ^ uint_low_u64(&m),
                ) {
                    m = m.add(Uint::ONE);
                    if m.bit(Self::MB + 1) {
                        m = m.shr(1);
                        e = e.add(Uint::ONE);
                    }
                }
                return Self::from_biased_uint_parts(negative, e, m);
            }
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

        // diff >= 2: exact difference is in (2^(MB - 1), 2^(MB + 1)) and stays strictly normal.
        let max_shift = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 3);
        if diff.gt(&max_shift) {
            let a_signed = if negative { a.abs().neg() } else { a.abs() };
            if RM == RoundingMode::Stochastic as u8 {
                let stoch_limit = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 66);
                if diff.lt(&stoch_limit) {
                    let d = diff.cast_to_unsigned_float_exponent() as Exponent;
                    // Below a power of two (normal), ULP_below is half of ULP_above.
                    let d_eff = if ma.is_power_of_two() && ea.gt(&Uint::ONE) {
                        d - 1
                    } else {
                        d
                    };
                    let sub_frac64 = if d_eff >= Self::MB + 65 {
                        0
                    } else if d_eff <= 64 {
                        uint_low_u64(&mb) << (64 - d_eff)
                    } else {
                        uint_low_u64(&mb.shr(d_eff - 64))
                    };
                    let frac64 = if sub_frac64 == 0 {
                        u64::MAX
                    } else {
                        0u64.wrapping_sub(sub_frac64)
                    };
                    let prev_bits = a_signed.to_bits().sub(Uint::ONE);
                    let seed = seed_base ^ uint_low_u64(&ma);
                    if RoundingMode::should_round_up::<RM>(
                        negative,
                        prev_bits.is_odd(),
                        true,
                        true,
                        frac64,
                        seed,
                    ) {
                        return a_signed;
                    } else {
                        return Self::from_bits(prev_bits);
                    }
                }
                return a_signed;
            }
            return Self::round_with_tiny_delta(a_signed, negative);
        }

        let d = diff.cast_to_unsigned_float_exponent() as Exponent;
        let (b_int, sticky, frac64_sub) = if d == 2 {
            (mb, false, 0u64)
        } else {
            let shift = d - 2;
            let st = mb.trailing_zeros() < shift;
            let f64_s = if RM == RoundingMode::Stochastic as u8 && st {
                uint_frac64(&mb, shift).wrapping_neg()
            } else {
                0
            };
            (mb.shr(shift), st, f64_s)
        };

        let mut diff4 = ma.shl(2).sub(b_int);
        if sticky {
            diff4 = diff4.sub(Uint::ONE);
        }

        let (res_exp, res_mant) = if diff4.bit(Self::MB + 2) {
            let mut m = diff4.shr(2);
            let round_bit = diff4.bit(1);
            let lower_sticky = diff4.bit(0) || sticky;
            let frac64 = if RM == RoundingMode::Stochastic as u8 {
                ((diff4.bit(1) as u64) << 63)
                    | ((diff4.bit(0) as u64) << 62)
                    | (frac64_sub >> 2)
            } else {
                0
            };
            let mut e = ea;
            if RoundingMode::should_round_up::<RM>(
                negative,
                m.is_odd(),
                round_bit,
                lower_sticky,
                frac64,
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
            let mut m = diff4.shr(1);
            let mut e = ea.sub(Uint::ONE);
            let round_bit = diff4.bit(0);
            let frac64 = if RM == RoundingMode::Stochastic as u8 {
                ((diff4.bit(0) as u64) << 63) | (frac64_sub >> 1)
            } else {
                0
            };
            if RoundingMode::should_round_up::<RM>(
                negative,
                m.is_odd(),
                round_bit,
                sticky,
                frac64,
                seed_base ^ uint_low_u64(&m),
            ) {
                m = m.add(Uint::ONE);
                if m.bit(Self::MB + 1) {
                    m = m.shr(1);
                    e = e.add(Uint::ONE);
                }
            }
            (e, m)
        };

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
                } else if self.is_sign_negative() == rhs.is_sign_negative() {
                    Self::zero_from_cancellation()
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
