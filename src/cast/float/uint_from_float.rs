use super::FloatCastHelper;
use super::FloatMantissa;
use crate::Exponent;
use crate::cast::CastFrom;
use crate::helpers::{Bits, One, Zero};
use core::ops::{Neg, Shl};

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

pub enum UintFromFloatError {
    Negative,
    Overflow,
    NaN,
}

pub fn cast_uint_from_float<F, U>(value: F) -> U
where
    F: FloatCastHelper,
    F::Mantissa: Bits,
    Exponent: TryFrom<F::SignedExp>,
    U: CastUintFromFloatHelper + CastFrom<F::Mantissa> + Shl<Exponent, Output = U>,
    F::SignedExp: One + Neg<Output = F::SignedExp>,
{
    match uint_try_from_float::<F, U>(value) {
        Ok(u) => u,
        Err(UintFromFloatError::Negative) => U::MIN,
        Err(UintFromFloatError::Overflow) => U::MAX,
        Err(UintFromFloatError::NaN) => U::ZERO,
    }
}

pub fn uint_try_from_float<F, U>(value: F) -> Result<U, UintFromFloatError>
where
    F: FloatCastHelper,
    F::Mantissa: Bits,
    Exponent: TryFrom<F::SignedExp>,
    U: CastUintFromFloatHelper + CastFrom<F::Mantissa> + Shl<Exponent, Output = U>,
    F::SignedExp: One + Neg<Output = F::SignedExp>,
{
    if value.is_nan() {
        return Err(UintFromFloatError::NaN);
    }
    let is_infinite = value.is_infinite(); // store this value first, as then we check if value is infinite after checking if it is negative, as we don't want to return MAX for negative infinity
    let (sign, exp, mant) = value.into_normalised_signed_parts();
    if mant.is_zero() {
        return Ok(U::ZERO);
    }
    if sign {
        return Err(UintFromFloatError::Negative);
    }
    if is_infinite {
        return Err(UintFromFloatError::Overflow);
    }
    if exp < -F::SignedExp::ONE {
        // in this case, the value is at most a half, so we round (ties to even) to zero
        return Ok(U::ZERO);
    }
    if exp == -F::SignedExp::ONE {
        // exponent is -1, so value is in range [1/2, 1)
        if mant.is_power_of_two() {
            // in this case, the value is exactly 1/2, so we round (ties to even) to zero
            return Ok(U::ZERO);
        }
        return Ok(U::ONE);
    }
    // now we know that the exponent is non-negative so can shift
    // As per Rust's numeric casting semantics (https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast), casting a float to an integer truncates rather than using ties to even

    match Exponent::try_from(exp) {
        Ok(exp) => {
            if exp >= U::BITS {
                return Err(UintFromFloatError::Overflow);
            }
            let mant_bit_width = mant.bit_width();
            if exp <= mant_bit_width - 1 {
                // in this case, we have a fractional part to truncate
                Ok(U::cast_from(mant >> (mant_bit_width - 1 - exp))) // the right shift means the mantissa now has exp + 1 bits, and as we must have exp < U::BITS, the shifted mantissa is no wider than U
            } else {
                Ok(U::cast_from(mant) << (exp - (mant_bit_width - 1)))
            }
        }
        _ => Err(UintFromFloatError::Overflow),
    }
}
