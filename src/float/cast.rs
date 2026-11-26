use crate::cast::float::{
    CastFloatFromUintHelper, ConvertFloatParts, FloatCastHelper, FloatMantissa,
};

use super::{Float, FloatExponent, RoundingMode, UnsignedFloatExponent};
use crate::Exponent;
use crate::cast::CastFrom;
use crate::{Int, Integer, Uint};

impl<const N: usize, const B: usize> FloatMantissa for Uint<N, B> {
    #[inline]
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

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

/// Casts an unsigned magnitude with an explicit `sign` to `Float<W, MB, RM>`, rounding
/// according to `RM`.
fn cast_float_from_uint_signed<U, const W: usize, const MB: usize, const RM: u8>(
    sign: bool,
    value: U,
) -> Float<W, MB, RM>
where
    U: CastFloatFromUintHelper + Copy,
    Uint<W>: CastFrom<U>,
    u64: CastFrom<U>,
{
    let bit_width = value.bit_width();
    if bit_width == 0 {
        return Float::ZERO;
    }
    let exp_u = bit_width - 1;
    if Float::<W, MB, RM>::EXPONENT_BITS <= 64
        && (exp_u as u64) > ((1u128 << (Float::<W, MB, RM>::EXPONENT_BITS - 1)) - 1) as u64
    {
        return Float::overflow_value(sign);
    }
    let bias = Float::<W, MB, RM>::exp_bias_uint();
    let exp_max = Float::<W, MB, RM>::exp_max_uint();
    let exp_as_uint = Uint::<W>::cast_from_unsigned_float_exponent(exp_u);
    let mut biased_exp = bias.add(exp_as_uint);

    // Check overflow: if biased_exp >= exp_max (or bias + exp_u overflowed).
    if exp_as_uint.gt(&bias) || biased_exp.ge(&exp_max) {
        return Float::overflow_value(sign);
    }

    let mb_plus_1 = Float::<W, MB, RM>::MANTISSA_DIGITS;
    let mant = if bit_width <= mb_plus_1 {
        Uint::<W>::cast_from(value).shl(mb_plus_1 - bit_width)
    } else {
        let shift = bit_width - mb_plus_1;
        let round_bit = value.bit(shift - 1);
        let sticky = value.trailing_zeros() < shift - 1;
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            if shift <= 64 {
                let lo = u64::cast_from(value);
                if shift == 64 {
                    lo
                } else {
                    (lo & ((1u64 << shift) - 1)) << (64 - shift)
                }
            } else {
                u64::cast_from(value >> (shift - 64))
            }
        } else {
            0
        };
        let mut q = Uint::<W>::cast_from(value >> shift);
        let seed = uint_low_u64(&biased_exp).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ uint_low_u64(&q);
        if RoundingMode::should_round_up::<RM>(sign, q.is_odd(), round_bit, sticky, frac64, seed) {
            q = q.add(Uint::ONE);
            if q.bit(mb_plus_1) {
                q = q.shr(1);
                biased_exp = biased_exp.add(Uint::ONE);
            }
        }
        q
    };

    if biased_exp.ge(&exp_max) {
        return Float::overflow_value(sign);
    }
    Float::from_biased_uint_parts(sign, biased_exp, mant)
}

macro_rules! uint_as_float {
    ($($uint: ident), *) => {
        $(
            impl<const W: usize, const MB: usize, const RM: u8> CastFrom<$uint> for Float<W, MB, RM> {
                #[inline]
                fn cast_from(value: $uint) -> Self {
                    cast_float_from_uint_signed(false, value)
                }
            }
        )*
    };
}

uint_as_float!(u8, u16, u32, u64, u128, usize);

macro_rules! int_as_float {
    ($($int: ty), *) => {
        $(
            impl<const W: usize, const MB: usize, const RM: u8> CastFrom<$int> for Float<W, MB, RM> {
                #[inline]
                fn cast_from(value: $int) -> Self {
                    cast_float_from_uint_signed(value.is_negative(), value.unsigned_abs())
                }
            }
        )*
    };
}

int_as_float!(i8, i16, i32, i64, i128, isize);

impl<
    const W: usize,
    const MB: usize,
    const RM: u8,
    const S: bool,
    const N: usize,
    const B: usize,
    const OM: u8,
> CastFrom<Integer<S, N, B, OM>> for Float<W, MB, RM>
{
    fn cast_from(value: Integer<S, N, B, OM>) -> Self {
        if !S {
            return cast_float_from_uint_signed(false, value.force::<false, B, _>());
        }
        cast_float_from_uint_signed(
            value.is_negative_internal(),
            value.unsigned_abs_internal().force::<false, B, _>(),
        )
    }
}

impl<const W: usize, const MB: usize, const RM: u8, const N: usize> CastFrom<Float<W, MB, RM>>
    for Int<N>
{
    fn cast_from(value: Float<W, MB, RM>) -> Self {
        crate::integer::cast::cast_int_from_float!(value)
    }
}

macro_rules! float_as_int {
    ($($int: ty; $uint: ty), *) => {
        $(
            impl<const W: usize, const MB: usize, const RM: u8> CastFrom<Float<W, MB, RM>> for $int {
                #[inline]
                fn cast_from(value: Float<W, MB, RM>) -> Self {
                    if value.is_sign_negative() {
                        let u = <$uint>::cast_from(-value);
                        if u >= Self::MIN as $uint {
                            Self::MIN
                        } else {
                            -(u as $int)
                        }
                    } else {
                        let u = <$uint>::cast_from(value);
                        let i = u as $int;
                        if i.is_negative() {
                            Self::MAX
                        } else {
                            i
                        }
                    }
                }
            }
        )*
    };
}

float_as_int!(i8; u8, i16; u16, i32; u32, i64; u64, i128; u128, isize; usize);

macro_rules! float_as_uint {
    ($($uint: ident $(<$N: ident>)?), *) => {
        $(
            impl<const W: usize, const MB: usize, const RM: u8 $(, const $N: usize)?> CastFrom<Float<W, MB, RM>> for $uint $(<$N>)? {
                #[inline]
                fn cast_from(value: Float<W, MB, RM>) -> Self {
                    crate::cast::float::cast_uint_from_float(value)
                }
            }
        )*
    };
}

float_as_uint!(Uint<N>, u8, u16, u32, u64, u128, usize);

impl<const W: usize, const MB: usize, const RM: u8> ConvertFloatParts for Float<W, MB, RM> {
    type Mantissa = Uint<W>;
    type SignedExp = FloatExponent;
    type UnsignedExp = UnsignedFloatExponent;

    #[inline]
    fn into_raw_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_raw_parts(self)
    }

    #[inline]
    fn into_biased_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_biased_parts(self)
    }

    #[inline]
    fn into_signed_biased_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_biased_parts(self)
    }

    #[inline]
    fn into_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_parts(self)
    }

    #[inline]
    fn into_normalised_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_normalised_signed_parts(self)
    }

    #[inline]
    fn from_raw_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_raw_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_biased_parts(
        sign: bool,
        exponent: Self::UnsignedExp,
        mantissa: Self::Mantissa,
    ) -> Self {
        Self::from_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_biased_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self {
        Self::from_signed_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_signed_parts(sign, exponent, mantissa)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> FloatCastHelper for Float<W, MB, RM> {
    const MANTISSA_DIGITS: Exponent = Self::MANTISSA_DIGITS as Exponent;
    const MAX_EXP: FloatExponent = Self::MAX_EXP;
    const INFINITY: Self = Self::INFINITY;
    const ZERO: Self = Self::ZERO;

    #[inline]
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }
}

/// Converts `Float<W2, MB2, RM2>` into `Float<W1, MB1, RM1>` with a single rounding using `RM1`
/// (avoiding double rounding on subnormals and supporting arbitrary exponent widths).
fn cast_float_to_float_rm<
    const W1: usize,
    const MB1: usize,
    const RM1: u8,
    const W2: usize,
    const MB2: usize,
    const RM2: u8,
>(
    value: Float<W2, MB2, RM2>,
) -> Float<W1, MB1, RM1> {
    use core::num::FpCategory;
    match value.classify() {
        FpCategory::Nan => return Float::NAN,
        FpCategory::Infinite => {
            return if value.is_sign_negative() {
                Float::NEG_INFINITY
            } else {
                Float::INFINITY
            };
        }
        FpCategory::Zero => {
            return if value.is_sign_negative() {
                Float::NEG_ZERO
            } else {
                Float::ZERO
            };
        }
        _ => {}
    }

    let (sign, src_be, src_m) = value.into_biased_uint_parts();
    let src_bw = src_m.bit_width();
    let src_sub_shift = (MB2 as Exponent + 1) - src_bw;
    let src_norm = if src_sub_shift != 0 {
        src_m.shl(src_sub_shift)
    } else {
        src_m
    };

    let src_bias = Float::<W2, MB2, RM2>::exp_bias_uint().cast_signed();
    let src_exp: Int<W2> = src_be
        .cast_signed()
        .sub(src_bias)
        .sub(Uint::cast_from_unsigned_float_exponent(src_sub_shift).cast_signed());

    let dst_bias = Float::<W1, MB1, RM1>::exp_bias_uint().cast_signed();
    let dst_exp_max = Float::<W1, MB1, RM1>::exp_max_uint().cast_signed();

    // Check whether src_exp is out of dst's finite range before casting exponent to Int<W1>.
    let exp_neg = src_exp.is_negative();
    let exp_mag_src: Uint<W2> = if exp_neg {
        src_exp.wrapping_neg().cast_unsigned()
    } else {
        src_exp.cast_unsigned()
    };
    let dst_max_pos_mag = dst_bias.cast_unsigned(); // max finite unbiased exp == dst_bias
    // Half of min positive subnormal has unbiased exp == -(dst_bias + MB1).
    let dst_max_neg_mag = dst_bias
        .cast_unsigned()
        .add(Uint::cast_from_unsigned_float_exponent(MB1 as Exponent));

    // Compare exp_mag_src against dst bounds cleanly via bit_width / byte comparison.
    let src_mag_bw = exp_mag_src.bit_width();
    if !exp_neg {
        let lim_bw = dst_max_pos_mag.bit_width();
        if src_mag_bw > lim_bw
            || (src_mag_bw == lim_bw && Uint::<W1>::cast_from(exp_mag_src).gt(&dst_max_pos_mag))
        {
            return Float::overflow_value(sign);
        }
    } else {
        let lim_bw = dst_max_neg_mag.bit_width();
        if src_mag_bw > lim_bw
            || (src_mag_bw == lim_bw && Uint::<W1>::cast_from(exp_mag_src).gt(&dst_max_neg_mag))
        {
            return Float::underflow_nonzero(sign);
        }
    }

    // Now src_exp fits cleanly in Int<W1>.
    let e_dst_mag = Uint::<W1>::cast_from(exp_mag_src);
    let e_dst: Int<W1> = if exp_neg {
        e_dst_mag.cast_signed().wrapping_neg()
    } else {
        e_dst_mag.cast_signed()
    };
    let be = e_dst.add(dst_bias);

    let (mut out_be, sub_drop): (Int<W1>, Exponent) = if be.ge(&Int::ONE) {
        (be, 0)
    } else {
        let drop = Int::<W1>::ONE
            .sub(be)
            .cast_unsigned()
            .cast_to_unsigned_float_exponent();
        (Int::ONE, drop)
    };

    let src_digits = MB2 as Exponent + 1;
    let dst_digits = MB1 as Exponent + 1;

    // Special subnormal halfway boundary: sub_drop == dst_digits (value in [0.5, 1.0) of min subnormal)
    if sub_drop >= dst_digits {
        let sticky = src_norm.trailing_zeros() < src_digits - 1;
        let frac64 = if RM1 == RoundingMode::Stochastic as u8 {
            uint_frac64(&src_norm, src_digits)
        } else {
            1u64 << 63
        };
        let seed = uint_low_u64(&src_norm);
        return if RoundingMode::should_round_up::<RM1>(sign, false, true, sticky, frac64, seed) {
            Float::from_biased_uint_parts(sign, Uint::ONE, Uint::ONE)
        } else if sign {
            Float::NEG_ZERO
        } else {
            Float::ZERO
        };
    }

    let keep_bits = dst_digits - sub_drop;
    let mut q: Uint<W1> = if keep_bits >= src_digits {
        Uint::<W1>::cast_from(src_norm).shl(keep_bits - src_digits)
    } else {
        let shift = src_digits - keep_bits;
        let round_bit = src_norm.bit(shift - 1);
        let sticky = src_norm.trailing_zeros() < shift - 1;
        let frac64 = if RM1 == RoundingMode::Stochastic as u8 {
            uint_frac64(&src_norm, shift)
        } else {
            0
        };
        let mut q0 = Uint::<W1>::cast_from(src_norm.shr(shift));
        let seed = uint_low_u64(&out_be.cast_unsigned()).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&q0);
        if RoundingMode::should_round_up::<RM1>(sign, q0.is_odd(), round_bit, sticky, frac64, seed)
        {
            q0 = q0.add(Uint::ONE);
            if q0.bit(dst_digits) {
                q0 = q0.shr(1);
                out_be = out_be.add(Int::ONE);
            }
        }
        q0
    };

    if out_be.ge(&dst_exp_max) {
        return Float::overflow_value(sign);
    }
    let _ = &mut q;
    Float::from_biased_uint_parts(sign, out_be.cast_unsigned(), q)
}

impl<
    const W1: usize,
    const MB1: usize,
    const RM1: u8,
    const W2: usize,
    const MB2: usize,
    const RM2: u8,
> CastFrom<Float<W2, MB2, RM2>> for Float<W1, MB1, RM1>
{
    #[inline]
    fn cast_from(value: Float<W2, MB2, RM2>) -> Self {
        cast_float_to_float_rm(value)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> CastFrom<f32> for Float<W, MB, RM> {
    #[inline]
    fn cast_from(value: f32) -> Self {
        let f = Float::<4, 23, RM>::from_bits(Uint::from_le_bytes(value.to_bits().to_le_bytes()));
        cast_float_to_float_rm(f)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> CastFrom<f64> for Float<W, MB, RM> {
    #[inline]
    fn cast_from(value: f64) -> Self {
        let f = Float::<8, 52, RM>::from_bits(Uint::from_le_bytes(value.to_bits().to_le_bytes()));
        cast_float_to_float_rm(f)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> CastFrom<Float<W, MB, RM>> for f32 {
    #[inline]
    fn cast_from(value: Float<W, MB, RM>) -> Self {
        let f: Float<4, 23, { RoundingMode::TiesToEven as u8 }> = cast_float_to_float_rm(value);
        f32::from_bits(u32::from_le_bytes(f.to_bits().bytes))
    }
}

impl<const W: usize, const MB: usize, const RM: u8> CastFrom<Float<W, MB, RM>> for f64 {
    #[inline]
    fn cast_from(value: Float<W, MB, RM>) -> Self {
        let f: Float<8, 52, { RoundingMode::TiesToEven as u8 }> = cast_float_to_float_rm(value);
        f64::from_bits(u64::from_le_bytes(f.to_bits().bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::cast::{CastFrom, CastTo};
    use crate::test::cast_types::*;
    use crate::test::{test_from, test_into};

    crate::test::test_all! {
        testing floats;

        test_from! {
            function: <FTest as CastFrom>::cast_from,
            from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, UTest, ITest, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }

        test_into! {
            function: <FTest as CastTo>::cast_to,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, UTest, ITest, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }
    }
}
