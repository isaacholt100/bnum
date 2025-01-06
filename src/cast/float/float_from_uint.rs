use super::FloatCastHelper;
use crate::cast::CastFrom;
use crate::helpers::{Bits, One};
use crate::ExpType;
use core::ops::{Add, Shr};

pub trait CastFloatFromUintHelper: Bits + Shr<ExpType, Output = Self> {
    fn trailing_zeros(self) -> ExpType;
}

macro_rules! impl_cast_float_from_uint_helper_for_primitive_uint {
    ($($uint: ty), *) => {
        $(
            impl CastFloatFromUintHelper for $uint {
                #[inline]
                fn trailing_zeros(self) -> ExpType {
                    Self::trailing_zeros(self) as ExpType
                }
            }
        )*
    };
}

impl_cast_float_from_uint_helper_for_primitive_uint!(u8, u16, u32, u64, u128, usize);

pub fn cast_float_from_uint<U, F>(value: U) -> F
where
    F: FloatCastHelper,
    F::SignedExp: TryFrom<ExpType> + One + Add<F::SignedExp, Output = F::SignedExp>,
    F::Mantissa: CastFrom<U> + One,
    U: CastFloatFromUintHelper + Copy,
{
    let bit_width = value.bits(); // number of bits needed to specify value = exponent of largest power of two smaller than value. so bit_width will be one less than the exponent of the float
                                  // let mant = if F::M::BITS < U::BITS {

    // }
    if bit_width == 0 {
        // in this case, value is zero
        return F::ZERO;
    }
    let exponent = bit_width - 1; // value lies in range [2^(bit_width - 1), 2^bit_width)

    match F::SignedExp::try_from(exponent) {
        Ok(mut exponent) => {
            if exponent >= F::MAX_EXP {
                // exponent is too large
                return F::INFINITY;
            }
            let mantissa = if bit_width <= F::MANTISSA_DIGITS {
                // in this case, we can safely cast which preserves the value (no truncation)
                F::Mantissa::cast_from(value) << (F::MANTISSA_DIGITS - bit_width)
            } else {
                // note: we know that exponent >= F::MANTISSA_DIGITS, so only way
                // TODO: we could generalise the round_mantissa_exponent code so that this could just be a call of round_mantissa_exponent instead
                let shift = bit_width - F::MANTISSA_DIGITS;
                let gte_half = value.bit(shift - 1); // is the discarded part greater than or equal to a half?
                let mut shifted_mantissa = F::Mantissa::cast_from(value >> shift);
                if gte_half && (shifted_mantissa.bit(0) || value.trailing_zeros() != shift - 1) {
                    // by ties-to-even rule, round up
                    shifted_mantissa = shifted_mantissa + F::Mantissa::ONE;
                    if shifted_mantissa.bit(F::MANTISSA_DIGITS) {
                        // adding one overflowed to greater than mantissa width, so increment exponent and renormalised mantissa
                        shifted_mantissa = shifted_mantissa >> 1;
                        exponent = exponent + F::SignedExp::ONE;
                    }
                }
                shifted_mantissa
            };
            F::from_signed_parts(false, exponent, mantissa)
        }
        _ => F::INFINITY, // in this case, the exponent doesn't even fit into the float signed exponent, so is too big to be stored by F
    }
}
