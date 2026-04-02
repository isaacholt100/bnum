use crate::Float;
use rug::Float as RugFloat;
use rug::Integer as RugInteger;
use rug::ops::{PowAssign, NegAssign};

// similar to the BitInt test type, TestFloat is a type that we will use to test bnum's Float type against, for non-standard sizes and mantissa bit widths.
// it uses rug for operations, which is well-tested and used, so is reliable
// we can have confidence that these tests against TestFloat are valid, and that conversions between bnum::Float and TestFloat are correct by also testing for f64 and f32 parameters, i.e. 64 bits with 53 mantissa bits, and 32 bits with 24 mantissa bits.

// TODO: could also use astro-float instead
pub struct TestFloat<const N: usize, const MB: usize> {
    inner: Float<N, MB>,
}

impl<const N: usize, const MB: usize> From<Float<N, MB>> for RugFloat {
    fn from(value: Float<N, MB>) -> Self {
        let (sign, exponent, mantissa) = value.into_integral_signed_parts();
        let mantissa = RugInteger::from(mantissa);
        let mut out = RugFloat::with_val(Float::<N, MB>::MANTISSA_DIGITS, mantissa);
        out.pow_assign(exponent);
        if sign {
            out.neg_assign();
        }
        out
    }
}