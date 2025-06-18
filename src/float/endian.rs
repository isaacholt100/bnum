use super::Float;
use crate::doc;
use crate::Uint;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::endian::to_be_bytes!(F)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; W] {
        self.to_bits().to_be_bytes()
    }

    #[doc = doc::endian::to_le_bytes!(F)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; W] {
        self.to_bits().to_le_bytes()
    }

    #[doc = doc::endian::to_ne_bytes!(F)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; W] {
        self.to_bits().to_ne_bytes()
    }

    #[doc = doc::endian::from_be_bytes!(F)]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; W]) -> Self {
        Self::from_bits(Uint::from_be_bytes(bytes))
    }

    #[doc = doc::endian::from_le_bytes!(F)]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; W]) -> Self {
        Self::from_bits(Uint::from_le_bytes(bytes))
    }

    #[doc = doc::endian::from_ne_bytes!(F)]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; W]) -> Self {
        Self::from_bits(Uint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};
    use crate::test::U8ArrayWrapper;

    test_bignum! {
        function: <ftest>::to_be_bytes(a: ftest)
    }
    test_bignum! {
        function: <ftest>::to_le_bytes(a: ftest)
    }
    test_bignum! {
        function: <ftest>::to_ne_bytes(a: ftest)
    }
    test_bignum! {
        function: <ftest>::from_be_bytes(a: U8ArrayWrapper<{FTEST::BITS as usize / 8}>)
    }
    test_bignum! {
        function: <ftest>::from_le_bytes(a: U8ArrayWrapper<{FTEST::BITS as usize / 8}>)
    }
    test_bignum! {
        function: <ftest>::from_ne_bytes(a: U8ArrayWrapper<{FTEST::BITS as usize / 8}>)
    }
}
