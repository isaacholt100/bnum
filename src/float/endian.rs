use super::Float;
use crate::BUintD8;
use crate::doc;

#[cfg(feature = "nightly")]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_be_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; BUintD8::<W>::BYTES_USIZE] {
        self.to_bits().to_be_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_le_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; BUintD8::<W>::BYTES_USIZE] {
        self.to_bits().to_le_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_ne_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; BUintD8::<W>::BYTES_USIZE] {
        self.to_bits().to_ne_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_be_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; BUintD8::<W>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_be_bytes(bytes))
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_le_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; BUintD8::<W>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_le_bytes(bytes))
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_ne_bytes!(F)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; BUintD8::<W>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_ne_bytes(bytes))
    }
}

#[cfg(feature = "nightly")]
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
