use super::Float;
use crate::Uint;

/// Rounding mode used for [`Float`] arithmetic, conversions, parsing, and mathematical functions.
///
/// Includes all 5 IEEE 754-2019 rounding-direction attributes (`TiesToEven`, `TiesToAway`,
/// `TowardPositive`, `TowardNegative`, `TowardZero`) as well as commonly used extended rounding
/// modes (`AwayFromZero`, `ToOdd`, `TiesTowardPositive`, `TiesTowardNegative`,
/// `TiesTowardZero`, `TiesToOdd`, `Faithful`, `Stochastic`).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundingMode {
    /// IEEE 754 `roundTiesToEven` (default): round to nearest; halfway ties round to even LSB.
    TiesToEven = 0,
    /// IEEE 754 `roundTiesToAway`: round to nearest; halfway ties round away from zero.
    TiesToAway = 1,
    /// IEEE 754 `roundTowardPositive` (ceiling): round toward $+\infty$.
    TowardPositive = 2,
    /// IEEE 754 `roundTowardNegative` (floor): round toward $-\infty$.
    TowardNegative = 3,
    /// IEEE 754 `roundTowardZero` (truncation): round toward $0$.
    TowardZero = 4,
    /// Directed rounding away from zero (MPFR `RNDA`): round toward larger magnitude.
    AwayFromZero = 5,
    /// Round to odd (jamming / Von Neumann rounding): inexact results round to the adjacent
    /// representable value whose LSB is `1`, avoiding double rounding on subsequent narrowing.
    ToOdd = 6,
    /// Round to nearest; halfway ties round toward $+\infty$.
    TiesTowardPositive = 7,
    /// Round to nearest; halfway ties round toward $-\infty$.
    TiesTowardNegative = 8,
    /// Round to nearest; halfway ties round toward $0$.
    TiesTowardZero = 9,
    /// Round to nearest; halfway ties round to odd LSB.
    TiesToOdd = 10,
    /// Faithful rounding (MPFR `RNDF`): returns either representable value bracketing the exact
    /// result (error $< 1$ ULP), avoiding high-precision tie disambiguation.
    Faithful = 11,
    /// Stochastic rounding: rounds up in magnitude with probability equal to the fractional
    /// distance from the lower representable neighbor (using deterministic `const`-compatible
    /// SplitMix64 mixing so expected rounding error is zero in `#![no_std]` / `const fn`).
    Stochastic = 12,
}

impl RoundingMode {
    /// Default rounding mode (`TiesToEven`), matching IEEE 754 and Rust's primitive floats.
    pub const DEFAULT: Self = Self::TiesToEven;

    /// Maximum valid `u8` discriminant for `RoundingMode`.
    pub(crate) const MAX_U8: u8 = Self::Stochastic as u8;

    /// Converts this rounding mode to its `u8` discriminant for use as the `RM` const generic.
    #[inline(always)]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Converts a `u8` discriminant back to a [`RoundingMode`].
    ///
    /// Panics at compile time (or runtime) if `v` is not a valid [`RoundingMode`] discriminant.
    #[inline(always)]
    pub const fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::TiesToEven,
            1 => Self::TiesToAway,
            2 => Self::TowardPositive,
            3 => Self::TowardNegative,
            4 => Self::TowardZero,
            5 => Self::AwayFromZero,
            6 => Self::ToOdd,
            7 => Self::TiesTowardPositive,
            8 => Self::TiesTowardNegative,
            9 => Self::TiesTowardZero,
            10 => Self::TiesToOdd,
            11 => Self::Faithful,
            12 => Self::Stochastic,
            _ => panic!("invalid RoundingMode discriminant"),
        }
    }

    /// Returns `true` if `rm` is a round-to-nearest mode (discontinuity at `0.5` ULP).
    #[inline(always)]
    pub(crate) const fn is_nearest(rm: u8) -> bool {
        matches!(
            rm,
            0 | 1 | 7 | 8 | 9 | 10
        )
    }

    /// Returns `true` if `rm` is a directed or round-to-odd mode (discontinuity at exact `0.0` ULP).
    #[inline(always)]
    pub(crate) const fn is_directed_or_odd(rm: u8) -> bool {
        matches!(rm, 2 | 3 | 4 | 5 | 6)
    }

    /// Returns `true` if `rm` is `Faithful`.
    #[inline(always)]
    pub(crate) const fn is_faithful(rm: u8) -> bool {
        rm == Self::Faithful as u8
    }

    /// Returns `true` if `rm` is `Stochastic`.
    #[inline(always)]
    pub(crate) const fn is_stochastic(rm: u8) -> bool {
        rm == Self::Stochastic as u8
    }

    /// High-quality 64-bit SplitMix64 bijective mixer for `const fn` stochastic rounding.
    #[inline(always)]
    pub(crate) const fn splitmix64(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^ (x >> 31)
    }

    /// Central rounding-direction decision for an exact value `(-1)^sign * (q + f)` where
    /// `0 <= f < 1`.
    ///
    /// - `sign`: `true` if the exact result is negative.
    /// - `q_odd`: `true` if the truncated magnitude `q` is odd (LSB == 1).
    /// - `round_bit`: `true` iff `f >= 0.5`.
    /// - `sticky`: `true` iff `f != 0 && f != 0.5` (i.e. strictly non-zero bits below `0.5`).
    /// - `frac64`: top 64 fractional bits `floor(f * 2^64)` (used only by `Stochastic`).
    /// - `seed`: caller-provided entropy mix (e.g. low bits of significand/exponent) for `Stochastic`.
    ///
    /// Returns `true` iff magnitude should be incremented from `q` to `q + 1`.
    #[inline(always)]
    pub(crate) const fn should_round_up<const RM: u8>(
        sign: bool,
        q_odd: bool,
        round_bit: bool,
        sticky: bool,
        frac64: u64,
        seed: u64,
    ) -> bool {
        const {
            assert!(RM <= Self::MAX_U8, "invalid RoundingMode discriminant");
        }
        // Exact values never round in any mode.
        if !round_bit && !sticky {
            return false;
        }
        let gt_half = round_bit && sticky;
        let eq_half = round_bit && !sticky;
        match RM {
            0 => gt_half || (eq_half && q_odd),              // TiesToEven
            1 => round_bit,                                  // TiesToAway
            2 => !sign,                                      // TowardPositive
            3 => sign,                                       // TowardNegative
            4 => false,                                      // TowardZero
            5 => true,                                       // AwayFromZero
            6 => !q_odd,                                     // ToOdd
            7 => gt_half || (eq_half && !sign),              // TiesTowardPositive
            8 => gt_half || (eq_half && sign),               // TiesTowardNegative
            9 => gt_half,                                    // TiesTowardZero
            10 => gt_half || (eq_half && !q_odd),            // TiesToOdd
            11 => gt_half || (eq_half && q_odd),             // Faithful (< 1 ULP)
            12 => {
                // Stochastic: round up with probability f ≈ frac64 / 2^64.
                let mix = seed
                    ^ frac64.wrapping_mul(0x517c_c1b7_2722_0a95)
                    ^ (if sign { 0xa5a5_a5a5_a5a5_a5a5 } else { 0 })
                    ^ (if q_odd { 0x3c6e_f372_fe94_f82b } else { 0 });
                let u = Self::splitmix64(mix);
                u < frac64
            }
            _ => unreachable!(),
        }
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// The [`RoundingMode`] configured for this float type.
    pub const ROUNDING_MODE: RoundingMode = RoundingMode::from_u8(RM);

    /// Reinterprets this float's exact bit pattern with a different [`RoundingMode`].
    #[inline(always)]
    pub const fn with_rounding_mode<const NEW_RM: u8>(self) -> Float<W, MB, NEW_RM> {
        const {
            assert!(NEW_RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }
        Float::from_bits(self.to_bits())
    }

    /// Returns the IEEE 754 overflow result for a finite operation whose exact magnitude
    /// exceeds the finite range (`|exact| >= 2^(emax + 1)` or rounding carries past `MAX`).
    #[inline(always)]
    pub(crate) const fn overflow_value(sign: bool) -> Self {
        const {
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }
        match RM {
            4 => {
                // TowardZero: never overflows finite operations to infinity.
                if sign { Self::MIN } else { Self::MAX }
            }
            2 => {
                // TowardPositive: positive -> +inf, negative -> -MAX (Self::MIN).
                if sign { Self::MIN } else { Self::INFINITY }
            }
            3 => {
                // TowardNegative: positive -> +MAX, negative -> -inf.
                if sign { Self::NEG_INFINITY } else { Self::MAX }
            }
            _ => {
                if sign {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
        }
    }

    /// Returns the correctly rounded result for a strictly non-zero exact value whose
    /// magnitude is strictly less than `0.5 * MIN_POSITIVE_SUBNORMAL`.
    #[inline(always)]
    pub(crate) const fn underflow_nonzero(sign: bool) -> Self {
        if RoundingMode::should_round_up::<RM>(sign, false, false, true, 0, sign as u64) {
            Self::from_biased_uint_parts(sign, Uint::ONE, Uint::ONE)
        } else if sign {
            Self::NEG_ZERO
        } else {
            Self::ZERO
        }
    }

    /// Returns the IEEE 754 zero produced by exact cancellation (`x - x`, `(+0) + (-0)`, etc.):
    /// `-0.0` under `TowardNegative`, `+0.0` under all other rounding modes.
    #[inline(always)]
    pub(crate) const fn zero_from_cancellation() -> Self {
        if RM == RoundingMode::TowardNegative as u8 {
            Self::NEG_ZERO
        } else {
            Self::ZERO
        }
    }

    /// Rounds an exact value `base + eps` (when `delta_positive == true`) or `base - eps`
    /// (when `delta_positive == false`), where `base` is finite and non-zero and
    /// `0 < eps << 0.5 * ULP(base)`.
    ///
    /// Used by tiny-argument / huge-gap fast paths so directed and round-to-odd modes still
    /// step by 1 ULP in the exact direction required.
    #[inline]
    pub(crate) const fn round_with_tiny_delta(base: Self, delta_positive: bool) -> Self {
        let sign = base.is_sign_negative();
        let bits = base.to_bits();
        let seed = {
            let mut v = 0u64;
            let mut i = 0;
            while i < W && i < 8 {
                v |= (bits.bytes[i] as u64) << (i * 8);
                i += 1;
            }
            v
        };
        let inc_mag = delta_positive != sign;
        if inc_mag {
            // Exact magnitude is |base| + eps: between |base| and |base| + ULP_above.
            let q_odd = bits.is_odd();
            if RoundingMode::should_round_up::<RM>(sign, q_odd, false, true, 0, seed) {
                let abs_bits = base.abs().to_bits();
                if abs_bits.eq(&Self::MAX.to_bits()) {
                    return Self::overflow_value(sign);
                }
                return Self::from_bits(bits.add(Uint::ONE));
            }
            base
        } else {
            // Exact magnitude is |base| - eps: between prev = (|base| - ULP_below) and |base|.
            let prev_bits = bits.sub(Uint::ONE);
            let prev_odd = prev_bits.is_odd();
            if RoundingMode::should_round_up::<RM>(sign, prev_odd, true, true, u64::MAX, seed) {
                base
            } else {
                Self::from_bits(prev_bits)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RoundingMode;
    use crate::cast::CastFrom;
    use crate::{Float, Int, Uint};

    const R_EVEN: u8 = RoundingMode::TiesToEven as u8;
    const R_AWAY: u8 = RoundingMode::TiesToAway as u8;
    const R_POS: u8 = RoundingMode::TowardPositive as u8;
    const R_NEG: u8 = RoundingMode::TowardNegative as u8;
    const R_ZERO: u8 = RoundingMode::TowardZero as u8;
    const R_AFZ: u8 = RoundingMode::AwayFromZero as u8;
    const R_ODD: u8 = RoundingMode::ToOdd as u8;
    const R_TPOS: u8 = RoundingMode::TiesTowardPositive as u8;
    const R_TNEG: u8 = RoundingMode::TiesTowardNegative as u8;
    const R_TZER: u8 = RoundingMode::TiesTowardZero as u8;
    const R_TODD: u8 = RoundingMode::TiesToOdd as u8;
    const R_FAITH: u8 = RoundingMode::Faithful as u8;
    const R_STOCH: u8 = RoundingMode::Stochastic as u8;

    /// Helper: compute signed ULP distance between two finite floats of the same format.
    fn ulp_diff<const W: usize, const MB: usize, const RM1: u8, const RM2: u8>(
        a: Float<W, MB, RM1>,
        b: Float<W, MB, RM2>,
    ) -> i128 {
        fn ordered_key<const W: usize, const MB: usize, const RM: u8>(
            x: Float<W, MB, RM>,
        ) -> i128 {
            let bits = x.to_bits();
            let mut u: u128 = 0;
            for i in 0..W.min(16) {
                u |= (bits.bytes[i] as u128) << (i * 8);
            }
            let sign_mask: u128 = 1u128 << (W * 8 - 1);
            if (u & sign_mask) != 0 {
                -((u & !sign_mask) as i128)
            } else {
                u as i128
            }
        }
        ordered_key(a) - ordered_key(b)
    }

    #[test]
    fn rounding_mode_discriminants_and_const_api() {
        for v in 0..=RoundingMode::MAX_U8 {
            let rm = RoundingMode::from_u8(v);
            assert_eq!(rm.to_u8(), v);
        }
        assert_eq!(Float::<8, 52>::ROUNDING_MODE, RoundingMode::TiesToEven);
        assert_eq!(
            Float::<8, 52, R_POS>::ROUNDING_MODE,
            RoundingMode::TowardPositive
        );
        assert_eq!(
            Float::<8, 52, R_STOCH>::ROUNDING_MODE,
            RoundingMode::Stochastic
        );
        let a: Float<8, 52, R_POS> = Float::cast_from(1.5_f64);
        let b: Float<8, 52, R_NEG> = a.with_rounding_mode::<R_NEG>();
        assert_eq!(a.to_bits(), b.to_bits());
    }

    #[test]
    fn all_13_modes_tie_breaking_add_sub_mul_div_fma() {
        // Test in binary16 (`Float<2, 10, RM>`): 1.0 has bits 0x3C00 (even), eps = 2^-10 (0x1400),
        // 0.5*eps = 2^-11 (0x1000).
        // Case 1: +1.0 + 0.5*eps (positive, even q = 0x3C00, exact tie between 0x3C00 and 0x3C01)
        // Case 2: +(1.0 + eps) + 0.5*eps (positive, odd q = 0x3C01, exact tie between 0x3C01 and 0x3C02)
        // Case 3: -1.0 - 0.5*eps (negative, even q = 0xBC00, exact tie between 0xBC00 and 0xBC01)
        // Case 4: -(1.0 + eps) - 0.5*eps (negative, odd q = 0xBC01, exact tie between 0xBC01 and 0xBC02)
        macro_rules! check_ties_for_mode {
            ($rm:expr, $pos_even_up:expr, $pos_odd_up:expr, $neg_even_up:expr, $neg_odd_up:expr) => {{
                type F = Float<2, 10, $rm>;
                let one = F::cast_from(1.0_f64);
                let eps = F::cast_from(2.0_f64.powi(-10));
                let half_eps = F::cast_from(2.0_f64.powi(-11));
                let one_p_eps = one + eps;

                let pe_lo = one.to_bits();
                let pe_hi = one_p_eps.to_bits();
                let po_lo = one_p_eps.to_bits();
                let po_hi = (one_p_eps + eps).to_bits();
                let ne_lo = (-one).to_bits();
                let ne_hi = (-one_p_eps).to_bits();
                let no_lo = (-one_p_eps).to_bits();
                let no_hi = (-(one_p_eps + eps)).to_bits();

                let exp_pe = if $pos_even_up { pe_hi } else { pe_lo };
                let exp_po = if $pos_odd_up { po_hi } else { po_lo };
                let exp_ne = if $neg_even_up { ne_hi } else { ne_lo };
                let exp_no = if $neg_odd_up { no_hi } else { no_lo };

                // add
                assert_eq!((one + half_eps).to_bits(), exp_pe, "add pos even RM={}", $rm);
                assert_eq!((one_p_eps + half_eps).to_bits(), exp_po, "add pos odd RM={}", $rm);
                assert_eq!(((-one) + (-half_eps)).to_bits(), exp_ne, "add neg even RM={}", $rm);
                assert_eq!(((-one_p_eps) + (-half_eps)).to_bits(), exp_no, "add neg odd RM={}", $rm);

                // sub (same exact halfway values via subtraction from the upper neighbor)
                assert_eq!((one_p_eps - half_eps).to_bits(), exp_pe, "sub pos even RM={}", $rm);
                assert_eq!(((one_p_eps + eps) - half_eps).to_bits(), exp_po, "sub pos odd RM={}", $rm);
                assert_eq!(((-one_p_eps) - (-half_eps)).to_bits(), exp_ne, "sub neg even RM={}", $rm);
                assert_eq!(((-(one_p_eps + eps)) - (-half_eps)).to_bits(), exp_no, "sub neg odd RM={}", $rm);

                // mul_add
                assert_eq!(one.mul_add(one, half_eps).to_bits(), exp_pe, "fma pos even RM={}", $rm);
                assert_eq!(one_p_eps.mul_add(one, half_eps).to_bits(), exp_po, "fma pos odd RM={}", $rm);
                assert_eq!((-one).mul_add(one, -half_eps).to_bits(), exp_ne, "fma neg even RM={}", $rm);
                assert_eq!((-one_p_eps).mul_add(one, -half_eps).to_bits(), exp_no, "fma neg odd RM={}", $rm);

                // mul: (1 + 2^-5) * (1 + 2^-6) = 1 + 2^-5 + 2^-6 + 2^-11 (exact halfway at even q!)
                let a = F::cast_from(1.0 + 2.0_f64.powi(-5));
                let b = F::cast_from(1.0 + 2.0_f64.powi(-6));
                let mul_even_lo = F::cast_from(1.0 + 2.0_f64.powi(-5) + 2.0_f64.powi(-6)).to_bits();
                let mul_even_hi = mul_even_lo + Uint::ONE;
                assert_eq!(
                    (a * b).to_bits(),
                    if $pos_even_up { mul_even_hi } else { mul_even_lo },
                    "mul pos even RM={}", $rm
                );
                let neg_mul_even_lo = (-F::cast_from(1.0 + 2.0_f64.powi(-5) + 2.0_f64.powi(-6))).to_bits();
                let neg_mul_even_hi = neg_mul_even_lo + Uint::ONE;
                assert_eq!(
                    ((-a) * b).to_bits(),
                    if $neg_even_up { neg_mul_even_hi } else { neg_mul_even_lo },
                    "mul neg even RM={}", $rm
                );

                // mul odd q: (1 + 2^-10 + 2^-5) * (1 + 2^-6)? Wait: 2^-10 * 2^-6 = 2^-16 (sticky!).
                // Instead: (1 + 2^-4) * (1 + 2^-7) = 1 + 2^-4 + 2^-7 + 2^-11 (even q),
                // and (1 + 2^-10) has 11 bits. Use (1 + 2^-5 + 2^-10)? No, multiply by 2:
                // (2048 + 32) * (2048 + 64) -> even. For odd q: (1 + 2^-10) * (1 + 0.5) = 1.5 + 1.5*2^-10 = 1.5 + 2^-10 + 2^-11!
                // Wait: 1.5 + 2^-10 has mantissa 512 + 1 = 513 (ODD!) and exact remainder 2^-11 = 0.5*eps!
                let p_odd_a = one_p_eps;
                let p_odd_b = F::cast_from(1.5_f64);
                let mul_odd_lo = F::cast_from(1.5 + 2.0_f64.powi(-10)).to_bits();
                let mul_odd_hi = mul_odd_lo + Uint::ONE;
                assert_eq!(
                    (p_odd_a * p_odd_b).to_bits(),
                    if $pos_odd_up { mul_odd_hi } else { mul_odd_lo },
                    "mul pos odd RM={}", $rm
                );
                let neg_mul_odd_lo = (-F::cast_from(1.5 + 2.0_f64.powi(-10))).to_bits();
                let neg_mul_odd_hi = neg_mul_odd_lo + Uint::ONE;
                assert_eq!(
                    ((-p_odd_a) * p_odd_b).to_bits(),
                    if $neg_odd_up { neg_mul_odd_hi } else { neg_mul_odd_lo },
                    "mul neg odd RM={}", $rm
                );

                // div: exact halfway ties via (2*q + 1) / 2? Numerator has 12 bits!
                // Instead: (1 + 2^-9) / (1 + 0) is exact; to get exact .5 in div:
                // Let x = 1 + 2^-10 (1025/1024), d = 2.0 is exact.
                // Actually any odd mantissa divided by 1.0? No: let x = 3 * 2^-1 = 1.5; wait
                // in binary16, dividing by a power of 2 is exact UNLESS result is subnormal!
                // Subnormal exact halfway: MIN_SUBNORMAL = 2^-24.
                // 2^-24 / 2.0 = 0.5 * MIN_SUBNORMAL (q = 0, EVEN, exact tie!)
                // (3 * 2^-24) / 2.0 = 1.5 * MIN_SUBNORMAL (q = 1, ODD, exact tie!)
                let min_sub = F::from_bits(Uint::from_le_bytes([1, 0]));
                let three_sub = F::from_bits(Uint::from_le_bytes([3, 0]));
                let two = F::cast_from(2.0_f64);
                assert_eq!(
                    (min_sub / two).to_bits(),
                    if $pos_even_up { Uint::from_le_bytes([1, 0]) } else { Uint::from_le_bytes([0, 0]) },
                    "div subnormal pos even RM={}", $rm
                );
                assert_eq!(
                    (three_sub / two).to_bits(),
                    if $pos_odd_up { Uint::from_le_bytes([2, 0]) } else { Uint::from_le_bytes([1, 0]) },
                    "div subnormal pos odd RM={}", $rm
                );
                assert_eq!(
                    ((-min_sub) / two).to_bits(),
                    if $neg_even_up { (-min_sub).to_bits() } else { F::NEG_ZERO.to_bits() },
                    "div subnormal neg even RM={}", $rm
                );
                assert_eq!(
                    ((-three_sub) / two).to_bits(),
                    if $neg_odd_up { F::from_bits(Uint::from_le_bytes([2, 0x80])).to_bits() } else { (-min_sub).to_bits() },
                    "div subnormal neg odd RM={}", $rm
                );
            }};
        }

        //                              pos_even  pos_odd  neg_even  neg_odd
        check_ties_for_mode!(R_EVEN,    false,    true,    false,    true);
        check_ties_for_mode!(R_AWAY,    true,     true,    true,     true);
        check_ties_for_mode!(R_POS,     true,     true,    false,    false);
        check_ties_for_mode!(R_NEG,     false,    false,   true,     true);
        check_ties_for_mode!(R_ZERO,    false,    false,   false,    false);
        check_ties_for_mode!(R_AFZ,     true,     true,    true,     true);
        check_ties_for_mode!(R_ODD,     true,     false,   true,     false);
        check_ties_for_mode!(R_TPOS,    true,     true,    false,    false);
        check_ties_for_mode!(R_TNEG,    false,    false,   true,     true);
        check_ties_for_mode!(R_TZER,    false,    false,   false,    false);
        check_ties_for_mode!(R_TODD,    true,     false,   true,     false);

        // Faithful and Stochastic must bracket within [q, q + 1] and preserve exact ops.
        macro_rules! check_faithful_stoch {
            ($rm:expr) => {{
                type F = Float<2, 10, $rm>;
                let one = F::cast_from(1.0_f64);
                let eps = F::cast_from(2.0_f64.powi(-10));
                let half_eps = F::cast_from(2.0_f64.powi(-11));
                let r = one + half_eps;
                assert!(r.to_bits() == one.to_bits() || r.to_bits() == (one + eps).to_bits());
                // Exact ops must never jitter
                assert_eq!((one + F::cast_from(0.5_f64)).to_bits(), F::cast_from(1.5_f64).to_bits());
                assert_eq!((F::cast_from(3.0_f64) * F::cast_from(5.0_f64)).to_bits(), F::cast_from(15.0_f64).to_bits());
                assert_eq!((F::cast_from(6.0_f64) / F::cast_from(2.0_f64)).to_bits(), F::cast_from(3.0_f64).to_bits());
                assert_eq!(F::cast_from(2.0_f64).mul_add(F::cast_from(3.0_f64), one).to_bits(), F::cast_from(7.0_f64).to_bits());
            }};
        }
        check_faithful_stoch!(R_FAITH);
        check_faithful_stoch!(R_STOCH);
    }

    #[test]
    fn all_13_modes_non_tie_quarter_and_three_quarters_ulp() {
        // Test non-tie fractions: +1.0 + 0.25*eps (f = 0.25) and +1.0 + 0.75*eps (f = 0.75),
        // and their negatives, for both even and odd q in binary16.
        macro_rules! check_nontie {
            ($rm:expr, $p25_e:expr, $p75_e:expr, $p25_o:expr, $p75_o:expr, $n25_e:expr, $n75_e:expr) => {{
                type F = Float<2, 10, $rm>;
                let one = F::cast_from(1.0_f64);
                let eps = F::cast_from(2.0_f64.powi(-10));
                let q25 = F::cast_from(2.0_f64.powi(-12));
                let q75 = F::cast_from(3.0 * 2.0_f64.powi(-12));
                let one_p = one + eps;

                let pe_lo = one.to_bits();
                let pe_hi = one_p.to_bits();
                let po_lo = one_p.to_bits();
                let po_hi = (one_p + eps).to_bits();
                let ne_lo = (-one).to_bits();
                let ne_hi = (-one_p).to_bits();

                assert_eq!((one + q25).to_bits(), if $p25_e { pe_hi } else { pe_lo }, "p25_e RM={}", $rm);
                assert_eq!((one + q75).to_bits(), if $p75_e { pe_hi } else { pe_lo }, "p75_e RM={}", $rm);
                assert_eq!((one_p + q25).to_bits(), if $p25_o { po_hi } else { po_lo }, "p25_o RM={}", $rm);
                assert_eq!((one_p + q75).to_bits(), if $p75_o { po_hi } else { po_lo }, "p75_o RM={}", $rm);
                assert_eq!(((-one) + (-q25)).to_bits(), if $n25_e { ne_hi } else { ne_lo }, "n25_e RM={}", $rm);
                assert_eq!(((-one) + (-q75)).to_bits(), if $n75_e { ne_hi } else { ne_lo }, "n75_e RM={}", $rm);
            }};
        }

        // All 6 round-to-nearest modes agree identically on non-ties:
        check_nontie!(R_EVEN, false, true, false, true, false, true);
        check_nontie!(R_AWAY, false, true, false, true, false, true);
        check_nontie!(R_TPOS, false, true, false, true, false, true);
        check_nontie!(R_TNEG, false, true, false, true, false, true);
        check_nontie!(R_TZER, false, true, false, true, false, true);
        check_nontie!(R_TODD, false, true, false, true, false, true);
        // Directed and ToOdd modes:
        check_nontie!(R_POS,  true,  true,  true,  true,  false, false);
        check_nontie!(R_NEG,  false, false, false, false, true,  true);
        check_nontie!(R_ZERO, false, false, false, false, false, false);
        check_nontie!(R_AFZ,  true,  true,  true,  true,  true,  true);
        check_nontie!(R_ODD,  true,  true,  false, false, true,  true);
    }

    #[test]
    fn all_13_modes_overflow_underflow_and_cancellation() {
        macro_rules! check_ovf_udf_cancel {
            ($rm:expr, $pos_ovf_inf:expr, $neg_ovf_inf:expr, $pos_udf_sub:expr, $neg_udf_sub:expr, $cancel_neg_zero:expr) => {{
                type F = Float<2, 10, $rm>;
                let exp_pos_ovf = if $pos_ovf_inf { F::INFINITY.to_bits() } else { F::MAX.to_bits() };
                let exp_neg_ovf = if $neg_ovf_inf { F::NEG_INFINITY.to_bits() } else { F::MIN.to_bits() };
                let min_sub = F::from_bits(Uint::from_le_bytes([1, 0]));
                let exp_pos_udf = if $pos_udf_sub { min_sub.to_bits() } else { F::ZERO.to_bits() };
                let exp_neg_udf = if $neg_udf_sub { (-min_sub).to_bits() } else { F::NEG_ZERO.to_bits() };
                let exp_cancel = if $cancel_neg_zero { F::NEG_ZERO.to_bits() } else { F::ZERO.to_bits() };

                let two = F::cast_from(2.0_f64);
                let quarter = F::cast_from(0.25_f64);
                let one = F::cast_from(1.0_f64);

                // Overflow across add, mul, div, exp, from_str, cast
                assert_eq!((F::MAX + F::MAX).to_bits(), exp_pos_ovf, "add +ovf RM={}", $rm);
                assert_eq!((F::MIN + F::MIN).to_bits(), exp_neg_ovf, "add -ovf RM={}", $rm);
                assert_eq!((F::MAX * two).to_bits(), exp_pos_ovf, "mul +ovf RM={}", $rm);
                assert_eq!((F::MIN * two).to_bits(), exp_neg_ovf, "mul -ovf RM={}", $rm);
                assert_eq!((F::MAX / F::cast_from(0.5_f64)).to_bits(), exp_pos_ovf, "div +ovf RM={}", $rm);
                assert_eq!((F::MIN / F::cast_from(0.5_f64)).to_bits(), exp_neg_ovf, "div -ovf RM={}", $rm);
                assert_eq!(F::cast_from(100.0_f64).exp().to_bits(), exp_pos_ovf, "exp +ovf RM={}", $rm);
                assert_eq!(F::from_str("1e100").unwrap().to_bits(), exp_pos_ovf, "from_str +ovf RM={}", $rm);
                assert_eq!(F::from_str("-1e100").unwrap().to_bits(), exp_neg_ovf, "from_str -ovf RM={}", $rm);
                assert_eq!(F::cast_from(1e100_f64).to_bits(), exp_pos_ovf, "cast f64 +ovf RM={}", $rm);
                assert_eq!(F::cast_from(-1e100_f64).to_bits(), exp_neg_ovf, "cast f64 -ovf RM={}", $rm);
                assert_eq!(F::cast_from(1_000_000_i64).to_bits(), exp_pos_ovf, "cast i64 +ovf RM={}", $rm);
                assert_eq!(F::cast_from(-1_000_000_i64).to_bits(), exp_neg_ovf, "cast i64 -ovf RM={}", $rm);

                // Underflow (< 0.5 * MIN_SUBNORMAL) across mul, div, exp, from_str, cast
                assert_eq!((min_sub * quarter).to_bits(), exp_pos_udf, "mul +udf RM={}", $rm);
                assert_eq!(((-min_sub) * quarter).to_bits(), exp_neg_udf, "mul -udf RM={}", $rm);
                assert_eq!((min_sub / F::cast_from(4.0_f64)).to_bits(), exp_pos_udf, "div +udf RM={}", $rm);
                assert_eq!(((-min_sub) / F::cast_from(4.0_f64)).to_bits(), exp_neg_udf, "div -udf RM={}", $rm);
                assert_eq!(F::cast_from(-100.0_f64).exp().to_bits(), exp_pos_udf, "exp +udf RM={}", $rm);
                assert_eq!(F::from_str("1e-100").unwrap().to_bits(), exp_pos_udf, "from_str +udf RM={}", $rm);
                assert_eq!(F::from_str("-1e-100").unwrap().to_bits(), exp_neg_udf, "from_str -udf RM={}", $rm);
                assert_eq!(F::cast_from(1e-100_f64).to_bits(), exp_pos_udf, "cast +udf RM={}", $rm);
                assert_eq!(F::cast_from(-1e-100_f64).to_bits(), exp_neg_udf, "cast -udf RM={}", $rm);

                // Exact cancellation zero sign
                assert_eq!((one - one).to_bits(), exp_cancel, "1 - 1 RM={}", $rm);
                assert_eq!(((-one) + one).to_bits(), exp_cancel, "-1 + 1 RM={}", $rm);
                assert_eq!((F::ZERO + F::NEG_ZERO).to_bits(), exp_cancel, "+0 + -0 RM={}", $rm);
                assert_eq!((F::ZERO - F::ZERO).to_bits(), exp_cancel, "+0 - +0 RM={}", $rm);
                assert_eq!((F::NEG_ZERO - F::NEG_ZERO).to_bits(), exp_cancel, "-0 - -0 RM={}", $rm);
                assert_eq!(two.mul_add(F::cast_from(3.0_f64), F::cast_from(-6.0_f64)).to_bits(), exp_cancel, "fma cancel RM={}", $rm);
            }};
        }

        //                                +ovf_inf -ovf_inf +udf_sub -udf_sub cancel_-0
        check_ovf_udf_cancel!(R_EVEN,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_AWAY,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_POS,      true,    false,   true,    false,   false);
        check_ovf_udf_cancel!(R_NEG,      false,   true,    false,   true,    true);
        check_ovf_udf_cancel!(R_ZERO,     false,   false,   false,   false,   false);
        check_ovf_udf_cancel!(R_AFZ,      true,    true,    true,    true,    false);
        check_ovf_udf_cancel!(R_ODD,      true,    true,    true,    true,    false);
        check_ovf_udf_cancel!(R_TPOS,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_TNEG,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_TZER,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_TODD,     true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_FAITH,    true,    true,    false,   false,   false);
        check_ovf_udf_cancel!(R_STOCH,    true,    true,    false,   false,   false);

        // Overflow boundary MAX + 0.5*ULP(MAX) in binary16: MAX = 65504, ULP(MAX) = 32, half = 16 -> 65520.
        macro_rules! check_max_half_ulp {
            ($rm:expr, $pos_inf:expr, $neg_inf:expr) => {{
                type F = Float<2, 10, $rm>;
                let half_ulp = F::cast_from(16.0_f64);
                let exp_p = if $pos_inf { F::INFINITY.to_bits() } else { F::MAX.to_bits() };
                let exp_n = if $neg_inf { F::NEG_INFINITY.to_bits() } else { F::MIN.to_bits() };
                assert_eq!((F::MAX + half_ulp).to_bits(), exp_p, "MAX+16 RM={}", $rm);
                assert_eq!((F::MIN - half_ulp).to_bits(), exp_n, "MIN-16 RM={}", $rm);
                assert_eq!(F::from_str("65520").unwrap().to_bits(), exp_p, "parse 65520 RM={}", $rm);
                assert_eq!(F::from_str("-65520").unwrap().to_bits(), exp_n, "parse -65520 RM={}", $rm);
                assert_eq!(F::cast_from(65520_i32).to_bits(), exp_p, "cast 65520 RM={}", $rm);
                assert_eq!(F::cast_from(-65520_i32).to_bits(), exp_n, "cast -65520 RM={}", $rm);
            }};
        }
        check_max_half_ulp!(R_EVEN, true,  true);
        check_max_half_ulp!(R_AWAY, true,  true);
        check_max_half_ulp!(R_POS,  true,  false);
        check_max_half_ulp!(R_NEG,  false, true);
        check_max_half_ulp!(R_ZERO, false, false);
        check_max_half_ulp!(R_AFZ,  true,  true);
        check_max_half_ulp!(R_ODD,  false, false); // MAX has odd LSB!
        check_max_half_ulp!(R_TPOS, true,  false);
        check_max_half_ulp!(R_TNEG, false, true);
        check_max_half_ulp!(R_TZER, false, false);
        check_max_half_ulp!(R_TODD, false, false); // MAX has odd LSB!
    }

    #[test]
    fn all_13_modes_from_str_exact_ties_and_grid_boundaries() {
        // Exact halfway midpoints in binary16 (`Float<2, 10, RM>`):
        //   1 + 2^-11       = 1.00048828125 (even q = 0x3C00)
        //   1 + 3 * 2^-11   = 1.00146484375 (odd  q = 0x3C01)
        macro_rules! check_parse {
            ($rm:expr, $pe:expr, $po:expr, $ne:expr, $no:expr, $gt1:expr, $lt1:expr, $neg_gt1:expr, $neg_lt1:expr) => {{
                type F = Float<2, 10, $rm>;
                let one = F::cast_from(1.0_f64);
                let one_p1 = F::from_bits(one.to_bits() + Uint::ONE);
                let one_p2 = F::from_bits(one.to_bits() + Uint::from_le_bytes([2, 0]));
                let one_m1 = F::from_bits(one.to_bits() - Uint::ONE);

                // Exact grid points must NEVER be perturbed!
                for exact_s in ["1.0", "-1.0", "1.5", "-1.5", "0.5", "-0.5", "0.25", "2.0", "65504"] {
                    let expected = Float::<2, 10, R_EVEN>::from_str(exact_s).unwrap().to_bits();
                    assert_eq!(F::from_str(exact_s).unwrap().to_bits(), expected, "exact '{}' RM={}", exact_s, $rm);
                }

                // Exact halfway ties
                assert_eq!(F::from_str("1.00048828125").unwrap().to_bits(), if $pe { one_p1.to_bits() } else { one.to_bits() }, "parse pe RM={}", $rm);
                assert_eq!(F::from_str("1.00146484375").unwrap().to_bits(), if $po { one_p2.to_bits() } else { one_p1.to_bits() }, "parse po RM={}", $rm);
                assert_eq!(F::from_str("-1.00048828125").unwrap().to_bits(), if $ne { (-one_p1).to_bits() } else { (-one).to_bits() }, "parse ne RM={}", $rm);
                assert_eq!(F::from_str("-1.00146484375").unwrap().to_bits(), if $no { (-one_p2).to_bits() } else { (-one_p1).to_bits() }, "parse no RM={}", $rm);

                // Tiny perturbations around power-of-two grid boundary 1.0 (ULP changes across 1.0!)
                let tiny_above = "1.000000000000000000000000000000000001";
                let tiny_below = "0.999999999999999999999999999999999999";
                assert_eq!(F::from_str(tiny_above).unwrap().to_bits(), if $gt1 { one_p1.to_bits() } else { one.to_bits() }, "parse 1+eps RM={}", $rm);
                assert_eq!(F::from_str(tiny_below).unwrap().to_bits(), if $lt1 { one.to_bits() } else { one_m1.to_bits() }, "parse 1-eps RM={}", $rm);
                assert_eq!(F::from_str("-1.000000000000000000000000000000000001").unwrap().to_bits(), if $neg_gt1 { (-one_p1).to_bits() } else { (-one).to_bits() }, "parse -1-eps RM={}", $rm);
                assert_eq!(F::from_str("-0.999999999999999999999999999999999999").unwrap().to_bits(), if $neg_lt1 { (-one).to_bits() } else { (-one_m1).to_bits() }, "parse -1+eps RM={}", $rm);
            }};
        }

        //                     pe     po     ne     no     1+e    1-e   -1-e  -1+e
        check_parse!(R_EVEN,   false, true,  false, true,  false, true,  false, true);
        check_parse!(R_AWAY,   true,  true,  true,  true,  false, true,  false, true);
        check_parse!(R_POS,    true,  true,  false, false, true,  true,  false, false);
        check_parse!(R_NEG,    false, false, true,  true,  false, false, true,  true);
        check_parse!(R_ZERO,   false, false, false, false, false, false, false, false);
        check_parse!(R_AFZ,    true,  true,  true,  true,  true,  true,  true,  true);
        check_parse!(R_ODD,    true,  false, true,  false, true,  false, true,  false);
        check_parse!(R_TPOS,   true,  true,  false, false, false, true,  false, true);
        check_parse!(R_TNEG,   false, false, true,  true,  false, true,  false, true);
        check_parse!(R_TZER,   false, false, false, false, false, true,  false, true);
        check_parse!(R_TODD,   true,  false, true,  false, false, true,  false, true);

        // Exact decimal values under Faithful and Stochastic
        for exact_s in ["1.0", "-1.0", "1.5", "-1.5", "0.25", "2.0"] {
            let expected = Float::<2, 10, R_EVEN>::from_str(exact_s).unwrap().to_bits();
            assert_eq!(Float::<2, 10, R_FAITH>::from_str(exact_s).unwrap().to_bits(), expected);
            assert_eq!(Float::<2, 10, R_STOCH>::from_str(exact_s).unwrap().to_bits(), expected);
        }
    }

    #[test]
    fn all_13_modes_casts_int_and_float() {
        // In binary16 (11 significand bits), integers in [2048, 4096] have ULP = 2:
        //   2049 is exact tie between 2048 (even) and 2050 (odd)
        //   2051 is exact tie between 2050 (odd)  and 2052 (even)
        macro_rules! check_casts {
            ($rm:expr, $pe:expr, $po:expr, $ne:expr, $no:expr) => {{
                type F = Float<2, 10, $rm>;
                let b2048 = F::cast_from(2048_u32).to_bits();
                let b2050 = F::cast_from(2050_u32).to_bits();
                let b2052 = F::cast_from(2052_u32).to_bits();
                let n2048 = F::cast_from(-2048_i32).to_bits();
                let n2050 = F::cast_from(-2050_i32).to_bits();
                let n2052 = F::cast_from(-2052_i32).to_bits();

                assert_eq!(F::cast_from(2049_u32).to_bits(), if $pe { b2050 } else { b2048 }, "u32 2049 RM={}", $rm);
                assert_eq!(F::cast_from(2051_u32).to_bits(), if $po { b2052 } else { b2050 }, "u32 2051 RM={}", $rm);
                assert_eq!(F::cast_from(-2049_i32).to_bits(), if $ne { n2050 } else { n2048 }, "i32 -2049 RM={}", $rm);
                assert_eq!(F::cast_from(-2051_i32).to_bits(), if $no { n2052 } else { n2050 }, "i32 -2051 RM={}", $rm);

                // Big bnum Int<8> / Uint<8>
                assert_eq!(F::cast_from(Uint::<8>::cast_from(2049_u64)).to_bits(), if $pe { b2050 } else { b2048 });
                assert_eq!(F::cast_from(Int::<8>::cast_from(-2049_i64)).to_bits(), if $ne { n2050 } else { n2048 });

                // f64 -> Float<2, 10, RM> normal and subnormal exact ties
                assert_eq!(F::cast_from(2049.0_f64).to_bits(), if $pe { b2050 } else { b2048 });
                assert_eq!(F::cast_from(-2049.0_f64).to_bits(), if $ne { n2050 } else { n2048 });
                // 0.5 * MIN_SUBNORMAL = 2^-25 (even q = 0), 1.5 * MIN_SUBNORMAL = 3 * 2^-25 (odd q = 1)
                let min_sub = Uint::<2>::from_le_bytes([1, 0]);
                let two_sub = Uint::<2>::from_le_bytes([2, 0]);
                assert_eq!(F::cast_from(2.0_f64.powi(-25)).to_bits(), if $pe { min_sub } else { F::ZERO.to_bits() }, "f64 sub tie pe RM={}", $rm);
                assert_eq!(F::cast_from(3.0 * 2.0_f64.powi(-25)).to_bits(), if $po { two_sub } else { min_sub }, "f64 sub tie po RM={}", $rm);
                assert_eq!(F::cast_from(-2.0_f64.powi(-25)).to_bits(), if $ne { (-F::from_bits(min_sub)).to_bits() } else { F::NEG_ZERO.to_bits() }, "f64 sub tie ne RM={}", $rm);
            }};
        }

        check_casts!(R_EVEN, false, true,  false, true);
        check_casts!(R_AWAY, true,  true,  true,  true);
        check_casts!(R_POS,  true,  true,  false, false);
        check_casts!(R_NEG,  false, false, true,  true);
        check_casts!(R_ZERO, false, false, false, false);
        check_casts!(R_AFZ,  true,  true,  true,  true);
        check_casts!(R_ODD,  true,  false, true,  false);
        check_casts!(R_TPOS, true,  true,  false, false);
        check_casts!(R_TNEG, false, false, true,  true);
        check_casts!(R_TZER, false, false, false, false);
        check_casts!(R_TODD, true,  false, true,  false);
    }

    #[test]
    fn to_odd_prevents_double_rounding() {
        // Construct values in F64 that lie just below or just above an F16 midpoint, such that
        // rounding F64 -> F32(TiesToEven) snaps onto the exact F16 midpoint and causes
        // F32 -> F16(TiesToEven) to double-round the WRONG way!
        //
        // F16 midpoint between 1.0 (even) and 1.0 + 2^-10 (odd) is M_even = 1.0 + 2^-11.
        //   x_above = M_even + 2^-40: true F16(TiesToEven) = 1.0 + 2^-10 (odd),
        //     but F32(TiesToEven) rounds x_above to M_even -> F16(TiesToEven) gives 1.0 (WRONG!).
        //     F32(ToOdd) rounds x_above to M_even + 2^-23 (odd!) -> F16(TiesToEven) gives 1.0 + 2^-10 (RIGHT!).
        // F16 midpoint between 1.0 + 2^-10 (odd) and 1.0 + 2*2^-10 (even) is M_odd = 1.0 + 3*2^-11.
        //   x_below = M_odd - 2^-40: true F16(TiesToEven) = 1.0 + 2^-10 (odd),
        //     but F32(TiesToEven) rounds x_below to M_odd -> F16(TiesToEven) gives 1.0 + 2*2^-10 (WRONG!).
        //     F32(ToOdd) rounds x_below to M_odd - 2^-23 (odd!) -> F16(TiesToEven) gives 1.0 + 2^-10 (RIGHT!).
        let m_even = 1.0_f64 + 2.0_f64.powi(-11);
        let m_odd = 1.0_f64 + 3.0 * 2.0_f64.powi(-11);
        let delta = 2.0_f64.powi(-40);

        let double_rounding_traps = [
            m_even + delta,
            -(m_even + delta),
            m_odd - delta,
            -(m_odd - delta),
        ];

        for &val in &double_rounding_traps {
            let x64 = Float::<8, 52, R_EVEN>::cast_from(val);
            let direct_f16 = Float::<2, 10, R_EVEN>::cast_from(x64);

            // Confirm naive F32(TiesToEven) -> F16(TiesToEven) actually suffers double rounding:
            let via_even_f32 = Float::<4, 23, R_EVEN>::cast_from(x64);
            let double_rounded = Float::<2, 10, R_EVEN>::cast_from(via_even_f32);
            assert_ne!(
                double_rounded.to_bits(),
                direct_f16.to_bits(),
                "expected naive TiesToEven chain to trigger double rounding for {}",
                val
            );

            // Confirm F32(ToOdd) -> F16(TiesToEven) eliminates double rounding completely:
            let via_odd_f32 = Float::<4, 23, R_ODD>::cast_from(x64);
            let fixed_f16 = Float::<2, 10, R_EVEN>::cast_from(via_odd_f32);
            assert_eq!(
                fixed_f16.to_bits(),
                direct_f16.to_bits(),
                "ToOdd intermediate failed to prevent double rounding for {}",
                val
            );
        }
    }

    #[test]
    fn all_32_math_functions_rounding_mode_invariants() {
        // Check universal ordering & tightness invariants across ALL 13 rounding modes for
        // EVERY mathematical function on both normal and tiny-argument fast-path inputs.
        macro_rules! check_fn_all_modes {
            ($name:expr, $expect_inexact:expr, $eval:expr) => {{
                let r_even  = $eval(Float::<8, 52, R_EVEN>::ZERO);
                let r_away  = $eval(Float::<8, 52, R_AWAY>::ZERO);
                let r_pos   = $eval(Float::<8, 52, R_POS>::ZERO);
                let r_neg   = $eval(Float::<8, 52, R_NEG>::ZERO);
                let r_zero  = $eval(Float::<8, 52, R_ZERO>::ZERO);
                let r_afz   = $eval(Float::<8, 52, R_AFZ>::ZERO);
                let r_odd   = $eval(Float::<8, 52, R_ODD>::ZERO);
                let r_tpos  = $eval(Float::<8, 52, R_TPOS>::ZERO);
                let r_tneg  = $eval(Float::<8, 52, R_TNEG>::ZERO);
                let r_tzer  = $eval(Float::<8, 52, R_TZER>::ZERO);
                let r_todd  = $eval(Float::<8, 52, R_TODD>::ZERO);
                let r_faith = $eval(Float::<8, 52, R_FAITH>::ZERO);
                let r_stoch = $eval(Float::<8, 52, R_STOCH>::ZERO);

                let span = ulp_diff(r_pos, r_neg);
                assert!(
                    span == 0 || span == 1,
                    "{}: TowardPositive - TowardNegative = {} ULP (expected 0 or 1)",
                    $name,
                    span
                );
                if $expect_inexact {
                    assert_eq!(span, 1, "{}: expected strictly inexact (span == 1 ULP)", $name);
                    assert!(
                        r_odd.to_bits().is_odd(),
                        "{}: ToOdd result must have odd LSB when inexact",
                        $name
                    );
                } else {
                    assert_eq!(span, 0, "{}: expected exact result (span == 0 ULP)", $name);
                }

                // Every mode must lie inside [TowardNegative, TowardPositive]
                let all_bits_ok = [
                    ulp_diff(r_even, r_neg),
                    ulp_diff(r_away, r_neg),
                    ulp_diff(r_zero, r_neg),
                    ulp_diff(r_afz, r_neg),
                    ulp_diff(r_odd, r_neg),
                    ulp_diff(r_tpos, r_neg),
                    ulp_diff(r_tneg, r_neg),
                    ulp_diff(r_tzer, r_neg),
                    ulp_diff(r_todd, r_neg),
                    ulp_diff(r_faith, r_neg),
                    ulp_diff(r_stoch, r_neg),
                ];
                for (idx, d) in all_bits_ok.iter().enumerate() {
                    assert!(
                        *d >= 0 && *d <= span,
                        "{}: mode index {} outside [TowardNegative, TowardPositive] (diff={}, span={})",
                        $name, idx, d, span
                    );
                }

                // TowardZero and AwayFromZero must match TowardNegative/TowardPositive by sign
                if !r_pos.is_zero() && !r_neg.is_zero() {
                    if r_pos.is_sign_negative() {
                        assert_eq!(r_zero.to_bits(), r_pos.to_bits(), "{}: neg TowardZero != TowardPositive", $name);
                        assert_eq!(r_afz.to_bits(), r_neg.to_bits(), "{}: neg AwayFromZero != TowardNegative", $name);
                    } else {
                        assert_eq!(r_zero.to_bits(), r_neg.to_bits(), "{}: pos TowardZero != TowardNegative", $name);
                        assert_eq!(r_afz.to_bits(), r_pos.to_bits(), "{}: pos AwayFromZero != TowardPositive", $name);
                    }
                }
            }};
        }

        // Helper to build a Float<8, 52, RM> from f64 using phantom type inference
        #[inline]
        fn mk<const RM: u8>(_tag: Float<8, 52, RM>, v: f64) -> Float<8, 52, RM> {
            Float::<8, 52, RM>::cast_from(v)
        }

        // Exact results across all 13 modes (span == 0!)
        check_fn_all_modes!("sqrt(4.0)", false, |t| mk(t, 4.0).sqrt());
        check_fn_all_modes!("cbrt(8.0)", false, |t| mk(t, 8.0).cbrt());
        check_fn_all_modes!("cbrt(-27.0)", false, |t| mk(t, -27.0).cbrt());
        check_fn_all_modes!("hypot(3.0, 4.0)", false, |t| mk(t, 3.0).hypot(mk(t, 4.0)));
        check_fn_all_modes!("exp2(3.0)", false, |t| mk(t, 3.0).exp2());
        check_fn_all_modes!("log2(8.0)", false, |t| mk(t, 8.0).log2());
        check_fn_all_modes!("log10(1000.0)", false, |t| mk(t, 1000.0).log10());
        check_fn_all_modes!("log(27.0, 3.0)", false, |t| mk(t, 27.0).log(mk(t, 3.0)));
        check_fn_all_modes!("powi(3.0, 2)", false, |t| mk(t, 3.0).powi(2));
        check_fn_all_modes!("powf(9.0, 0.5)", false, |t| mk(t, 9.0).powf(mk(t, 0.5)));

        // Strictly inexact results across ALL 32 math functions (positive, negative, and fast paths!)
        check_fn_all_modes!("sqrt(2.0)", true, |t| mk(t, 2.0).sqrt());
        check_fn_all_modes!("cbrt(2.0)", true, |t| mk(t, 2.0).cbrt());
        check_fn_all_modes!("cbrt(-2.0)", true, |t| mk(t, -2.0).cbrt());
        check_fn_all_modes!("mul_add(1.3, 2.7, -0.1)", true, |t| mk(t, 1.3).mul_add(mk(t, 2.7), mk(t, -0.1)));
        // div_euclid(1.0, 0.1): 1.0 / 0.1_f64 = 9.99999999999999944...
        assert_eq!(
            Float::<8, 52, R_POS>::cast_from(1.0_f64)
                .div_euclid(Float::<8, 52, R_POS>::cast_from(0.1_f64))
                .to_bits(),
            Float::<8, 52, R_POS>::cast_from(10.0_f64).to_bits()
        );
        assert_eq!(
            Float::<8, 52, R_NEG>::cast_from(1.0_f64)
                .div_euclid(Float::<8, 52, R_NEG>::cast_from(0.1_f64))
                .to_bits(),
            Float::<8, 52, R_NEG>::cast_from(9.0_f64).to_bits()
        );
        assert_eq!(
            Float::<8, 52, R_ZERO>::cast_from(1.0_f64)
                .div_euclid(Float::<8, 52, R_ZERO>::cast_from(0.1_f64))
                .to_bits(),
            Float::<8, 52, R_ZERO>::cast_from(9.0_f64).to_bits()
        );
        check_fn_all_modes!("rem_euclid(-1e-30, 1.0)", true, |t| mk(t, -1e-30).rem_euclid(mk(t, 1.0)));
        check_fn_all_modes!("powi(1.3, 7)", true, |t| mk(t, 1.3).powi(7));
        check_fn_all_modes!("powi(1.3, -5)", true, |t| mk(t, 1.3).powi(-5));
        check_fn_all_modes!("powf(2.5, 1.7)", true, |t| mk(t, 2.5).powf(mk(t, 1.7)));
        check_fn_all_modes!("exp(1.0)", true, |t| mk(t, 1.0).exp());
        check_fn_all_modes!("exp(-1.5)", true, |t| mk(t, -1.5).exp());
        check_fn_all_modes!("exp(tiny +2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).exp());
        check_fn_all_modes!("exp(tiny -2^-80)", true, |t| mk(t, -2.0_f64.powi(-80)).exp());
        check_fn_all_modes!("exp2(0.7)", true, |t| mk(t, 0.7).exp2());
        check_fn_all_modes!("exp_m1(0.3)", true, |t| mk(t, 0.3).exp_m1());
        check_fn_all_modes!("exp_m1(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).exp_m1());
        check_fn_all_modes!("ln(2.0)", true, |t| mk(t, 2.0).ln());
        check_fn_all_modes!("ln(0.3)", true, |t| mk(t, 0.3).ln());
        check_fn_all_modes!("ln_1p(0.3)", true, |t| mk(t, 0.3).ln_1p());
        check_fn_all_modes!("ln_1p(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).ln_1p());
        check_fn_all_modes!("log(5.0, 3.0)", true, |t| mk(t, 5.0).log(mk(t, 3.0)));
        check_fn_all_modes!("log2(3.0)", true, |t| mk(t, 3.0).log2());
        check_fn_all_modes!("log10(2.0)", true, |t| mk(t, 2.0).log10());
        check_fn_all_modes!("hypot(1.5, 2.5)", true, |t| mk(t, 1.5).hypot(mk(t, 2.5)));
        check_fn_all_modes!("hypot(1.0, 2^-80)", true, |t| mk(t, 1.0).hypot(mk(t, 2.0_f64.powi(-80))));
        check_fn_all_modes!("sin(1.0)", true, |t| mk(t, 1.0).sin());
        check_fn_all_modes!("sin(-1.0)", true, |t| mk(t, -1.0).sin());
        check_fn_all_modes!("sin(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).sin());
        check_fn_all_modes!("cos(1.0)", true, |t| mk(t, 1.0).cos());
        check_fn_all_modes!("cos(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).cos());
        check_fn_all_modes!("tan(1.0)", true, |t| mk(t, 1.0).tan());
        check_fn_all_modes!("tan(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).tan());
        check_fn_all_modes!("sin_cos(0.7).0", true, |t| mk(t, 0.7).sin_cos().0);
        check_fn_all_modes!("sin_cos(0.7).1", true, |t| mk(t, 0.7).sin_cos().1);
        check_fn_all_modes!("asin(0.5)", true, |t| mk(t, 0.5).asin());
        check_fn_all_modes!("asin(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).asin());
        check_fn_all_modes!("acos(0.5)", true, |t| mk(t, 0.5).acos());
        check_fn_all_modes!("atan(0.7)", true, |t| mk(t, 0.7).atan());
        check_fn_all_modes!("atan(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).atan());
        check_fn_all_modes!("atan2(1.0, 2.0)", true, |t| mk(t, 1.0).atan2(mk(t, 2.0)));
        check_fn_all_modes!("sinh(1.0)", true, |t| mk(t, 1.0).sinh());
        check_fn_all_modes!("sinh(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).sinh());
        check_fn_all_modes!("cosh(1.0)", true, |t| mk(t, 1.0).cosh());
        check_fn_all_modes!("cosh(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).cosh());
        check_fn_all_modes!("tanh(1.0)", true, |t| mk(t, 1.0).tanh());
        check_fn_all_modes!("tanh(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).tanh());
        check_fn_all_modes!("tanh(huge 100.0)", true, |t| mk(t, 100.0).tanh());
        check_fn_all_modes!("tanh(huge -100.0)", true, |t| mk(t, -100.0).tanh());
        check_fn_all_modes!("asinh(1.5)", true, |t| mk(t, 1.5).asinh());
        check_fn_all_modes!("asinh(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).asinh());
        check_fn_all_modes!("acosh(2.0)", true, |t| mk(t, 2.0).acosh());
        check_fn_all_modes!("atanh(0.5)", true, |t| mk(t, 0.5).atanh());
        check_fn_all_modes!("atanh(tiny 2^-80)", true, |t| mk(t, 2.0_f64.powi(-80)).atanh());
        check_fn_all_modes!("to_degrees(1.0)", true, |t| mk(t, 1.0).to_degrees());
        check_fn_all_modes!("to_radians(45.0)", true, |t| mk(t, 45.0).to_radians());
    }

    #[test]
    fn stochastic_rounding_unbiased_accumulation() {
        // Accumulate 2000 distinct small increments in binary16 (`Float<2, 10, RM>`, eps = 2^-10)
        // where each increment is in [0.1 * eps, 0.4 * eps] (< 0.5 * eps).
        // Under TiesToEven, EVERY addition rounds back to 1.0 (total stagnation!).
        // Under Stochastic, the running sum tracks the true mathematical sum closely!
        type FEven = Float<2, 10, R_EVEN>;
        type FStoch = Float<2, 10, R_STOCH>;

        let mut sum_even = FEven::cast_from(1.0_f64);
        let mut sum_stoch = FStoch::cast_from(1.0_f64);
        let mut exact = 1.0_f64;

        let eps = 2.0_f64.powi(-10);
        for i in 1u32..=2000 {
            // Distinct representable F16 increments in ~[0.08 * eps, 0.42 * eps]
            let frac = 0.08 + 0.34 * ((i.wrapping_mul(2654435761u32) as f64) / (u32::MAX as f64));
            let inc_f16_bits = FEven::cast_from(frac * eps).to_bits();
            let inc_even = FEven::from_bits(inc_f16_bits);
            let inc_stoch = FStoch::from_bits(inc_f16_bits);
            let inc_exact = f64::cast_from(inc_even);

            sum_even = sum_even + inc_even;
            sum_stoch = sum_stoch + inc_stoch;
            exact += inc_exact;
        }

        // TiesToEven completely stagnates at 1.0:
        assert_eq!(f64::cast_from(sum_even), 1.0_f64);

        // Stochastic sum tracks `exact` (~1.49) accurately (error well under 0.05):
        let stoch_val = f64::cast_from(sum_stoch);
        assert!(
            (stoch_val - exact).abs() < 0.05,
            "stochastic sum {} drifted too far from exact {} (even stagnated at {})",
            stoch_val,
            exact,
            f64::cast_from(sum_even)
        );
    }

    #[test]
    fn extreme_formats_with_rounding_modes() {
        // Test EXPONENT_BITS = 2 (`Float<2, 13, RM>`) and EXPONENT_BITS = 124 (`Float<16, 3, RM>`).
        macro_rules! check_format {
            ($W:expr, $MB:expr) => {{
                type FP = Float<$W, $MB, R_POS>;
                type FN = Float<$W, $MB, R_NEG>;
                type FZ = Float<$W, $MB, R_ZERO>;
                type FO = Float<$W, $MB, R_ODD>;
                type FE = Float<$W, $MB, R_EVEN>;

                // 1.0 / 3.0 is strictly inexact in every binary format
                let p_div = FP::cast_from(1.0_f64) / FP::cast_from(3.0_f64);
                let n_div = FN::cast_from(1.0_f64) / FN::cast_from(3.0_f64);
                let z_div = FZ::cast_from(1.0_f64) / FZ::cast_from(3.0_f64);
                let o_div = FO::cast_from(1.0_f64) / FO::cast_from(3.0_f64);
                let e_div = FE::cast_from(1.0_f64) / FE::cast_from(3.0_f64);

                assert_eq!(ulp_diff(p_div, n_div), 1, "div W={} MB={}", $W, $MB);
                assert_eq!(z_div.to_bits(), n_div.to_bits(), "div_z W={} MB={}", $W, $MB);
                assert!(o_div.to_bits().is_odd(), "div_o W={} MB={}", $W, $MB);
                assert!(ulp_diff(e_div, n_div) == 0 || ulp_diff(e_div, n_div) == 1);

                // sqrt(2.0), exp(0.5), ln(2.0), sin(0.5), from_str("0.3")
                let p_sqrt = FP::cast_from(2.0_f64).sqrt();
                let n_sqrt = FN::cast_from(2.0_f64).sqrt();
                assert_eq!(ulp_diff(p_sqrt, n_sqrt), 1, "sqrt W={} MB={}", $W, $MB);

                let p_exp = FP::cast_from(0.5_f64).exp();
                let n_exp = FN::cast_from(0.5_f64).exp();
                assert_eq!(ulp_diff(p_exp, n_exp), 1, "exp W={} MB={}", $W, $MB);

                let p_ln = FP::cast_from(2.0_f64).ln();
                let n_ln = FN::cast_from(2.0_f64).ln();
                assert_eq!(ulp_diff(p_ln, n_ln), 1, "ln W={} MB={}", $W, $MB);

                let p_sin = FP::cast_from(0.5_f64).sin();
                let n_sin = FN::cast_from(0.5_f64).sin();
                assert_eq!(ulp_diff(p_sin, n_sin), 1, "sin W={} MB={}", $W, $MB);

                let p_str = FP::from_str("0.3").unwrap();
                let n_str = FN::from_str("0.3").unwrap();
                assert_eq!(ulp_diff(p_str, n_str), 1, "from_str W={} MB={}", $W, $MB);
            }};
        }

        check_format!(2, 13);
        check_format!(16, 3);
    }
}
