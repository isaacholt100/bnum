use super::Float;
use crate::Uint;
use crate::doc;

/// Methods that convert floats to and from byte arrays.
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; W] {
        self.to_bits().to_be_bytes()
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; W] {
        self.to_bits().to_le_bytes()
    }

    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; W]) -> Self {
        Self::from_bits(Uint::from_be_bytes(bytes))
    }

    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; W]) -> Self {
        Self::from_bits(Uint::from_le_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::U8ArrayWrapper;
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <ftest>::to_be_bytes(a: ftest)
        }
        test_bignum! {
            function: <ftest>::to_le_bytes(a: ftest)
        }
        test_bignum! {
            function: <ftest>::from_be_bytes(a: U8ArrayWrapper<{FTEST::BITS as usize / 8}>)
        }
        test_bignum! {
            function: <ftest>::from_le_bytes(a: U8ArrayWrapper<{FTEST::BITS as usize / 8}>)
        }
    }
}
