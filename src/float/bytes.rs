use super::Float;
use crate::Uint;
use crate::doc;

/// Methods that convert floats to and from byte arrays.
impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
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
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest>::to_be_bytes(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::to_le_bytes(a: FTestBase)
        }
        test_bignum! {
            function: <FTest>::from_be_bytes(a: [u8; FTest::BITS as usize / 8])
        }
        test_bignum! {
            function: <FTest>::from_le_bytes(a: [u8; FTest::BITS as usize / 8])
        }

        #[test]
        fn bytes_preserve_nan_bits() {
            for nan in [
                FTest::NAN,
                FTest::NEG_NAN,
                FTest::from_bits(FTest::NAN.to_bits() | crate::Uint::ONE),
                FTest::from_bits(FTest::NEG_NAN.to_bits() | crate::Uint::ONE),
            ] {
                assert_eq!(FTest::from_be_bytes(nan.to_be_bytes()).to_bits(), nan.to_bits());
                assert_eq!(FTest::from_le_bytes(nan.to_le_bytes()).to_bits(), nan.to_bits());
            }
        }
    }
}
