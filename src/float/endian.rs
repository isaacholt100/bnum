use super::Float;
use crate::digit::u8 as digit;
use crate::BUintD8;

#[cfg(feature = "nightly")]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_be_bytes()
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_le_bytes()
    }

    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_ne_bytes()
    }

    #[inline]
    pub const fn from_be_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUintD8::from_be_bytes(bytes))
    }

    #[inline]
    pub const fn from_le_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUintD8::from_le_bytes(bytes))
    }

    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
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
