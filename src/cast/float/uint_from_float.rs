use core::ops::{Neg, Shl};
use crate::cast::CastFrom;
use crate::ExpType;
use super::FloatMantissa;
use super::FloatCastHelper;
use crate::helpers::{Bits, One, Zero};

pub trait CastUintFromFloatHelper: Zero + One + Bits {
    const MAX: Self;
    const MIN: Self;
}

macro_rules! impl_cast_uint_from_float_helper_for_primitive_uint {
        ($($uint: ident), *) => {
            $(
                impl CastUintFromFloatHelper for $uint {
                    const MAX: Self = Self::MAX;
                    const MIN: Self = Self::MIN;
                }
            )*
        };
    }

impl_cast_uint_from_float_helper_for_primitive_uint!(u8, u16, u32, u64, u128, usize);

pub fn cast_uint_from_float<F, U>(value: F) -> U
where
    F: FloatCastHelper + core::fmt::Debug,
    F::Mantissa: Bits,
    ExpType: TryFrom<F::SignedExp>,
    U: CastUintFromFloatHelper
        + CastFrom<F::Mantissa>
        + Shl<ExpType, Output = U>
        + core::fmt::Debug,
    F::SignedExp: One + Neg<Output = F::SignedExp>,
{
    if value.is_nan() {
        return U::ZERO;
    }
    let is_infinite = value.is_infinite(); // store this value first, as then we check if value is infinite after checking if it is negative, as we don't want to return MAX for negative infinity
    let (sign, exp, mant) = value.into_normalised_signed_parts();
    if sign {
        return U::MIN;
    }
    if is_infinite {
        return U::MAX;
    }
    if mant == F::Mantissa::ZERO {
        return U::ZERO;
    }
    if exp < -F::SignedExp::ONE {
        // in this case, the value is at most a half, so we round (ties to even) to zero
        return U::ZERO;
    }
    if exp == -F::SignedExp::ONE {
        // exponent is -1, so value is in range [1/2, 1)
        if mant.is_power_of_two() {
            // in this case, the value is exactly 1/2, so we round (ties to even) to zero
            return U::ZERO;
        }
        return U::ONE;
    }
    // now we know that the exponent is non-negative so can shift
    // As per Rust's numeric casting semantics (https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast), casting a float to an integer truncates rather than using ties to even

    match ExpType::try_from(exp) {
        Ok(exp) => {
            if exp >= U::BITS {
                return U::MAX;
            }
            let mant_bit_width = mant.bits();
            if exp <= mant_bit_width - 1 {
                // in this case, we have a fractional part to truncate
                U::cast_from(mant >> (mant_bit_width - 1 - exp)) // the right shift means the mantissa now has exp + 1 bits, and as we must have exp < U::BITS, the shifted mantissa is no wider than U
            } else {
                U::cast_from(mant) << (exp - (mant_bit_width - 1))
            }
        }
        _ => U::MAX,
    }
}
