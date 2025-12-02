use crate::Uint;

// similar to the BitInt test type, TestFloat is a type that we will use to test bnum's Float type against, for non-standard sizes and mantissa bit widths.
// it uses rug for operations, which is well-tested and used, so is reliable
// we can have confidence that these tests against TestFloat are valid, and that conversions between bnum::Float and TestFloat are correct by also testing for f64 and f32 parameters, i.e. 64 bits with 53 mantissa bits, and 32 bits with 24 mantissa bits.
pub struct TestFloat<const N: usize, const MB: usize> {
    bits: Uint<N>,
}

