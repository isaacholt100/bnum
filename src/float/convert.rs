use super::Float;
use crate::BUintD8;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    pub const fn to_bits(self) -> BUintD8<W> {
        self.bits
    }

    #[inline(always)]
    pub const fn from_bits(v: BUintD8<W>) -> Self {
        Self { bits: v }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{F32, F64};
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::to_bits(a: ftest)
    }
    test_bignum! {
        function: <f64>::from_bits(a: u64)
    }
    test_bignum! {
        function: <f32>::from_bits(a: u32)
    }
}
