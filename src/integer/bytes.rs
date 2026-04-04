use super::Integer;
use crate::doc;
use crate::Byte;

/// Methods that convert integers to and from byte arrays and slices.
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Returns the underlying bytes of `self` as an array.
    /// 
    /// This method is equivalent to [`to_le_bytes`](Self::to_le_bytes).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// let a = n!(0xE3BD89U24);
    /// assert_eq!(a.to_bytes(), [0x89, 0xBD, 0xE3]);
    /// 
    /// let b = n!(0x0A412FI24);
    /// assert_eq!(b.to_bytes(), [0x2F, 0x41, 0x0A]);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn to_bytes(self) -> [Byte; N] {
        self.bytes
    }

    /// Returns a reference to underlying bytes of `self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// let a = n!(0xD1FB70U24);
    /// assert_eq!(a.as_bytes(), &[0x70, 0xFB, 0xD1]);
    /// 
    /// let b = n!(0x3F23A1I24);
    /// assert_eq!(b.as_bytes(), &[0xA1, 0x23, 0x3F]);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[Byte; N] {
        &self.bytes
    }

    /// Returns a mutable reference to underlying bytes of `self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// let mut a = n!(0xD1FB70U24);
    /// let bytes = a.as_bytes_mut();
    /// bytes[0] = 0x3D;
    /// bytes[2] = 0xA5;
    /// assert_eq!(a, n!(0xA5FB3D));
    /// 
    /// let mut b = n!(0x1D3C4EI24);
    /// let bytes = b.as_bytes_mut();
    /// bytes[1] = 0xFF;
    /// assert_eq!(b, n!(0x1DFF4E));
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn as_bytes_mut(&mut self) -> &mut [Byte; N] {
        &mut self.bytes
    }

    const ASSERT_IS_VALID: () = {
        assert!(OM <= 2, "invalid overflow mode");
        assert!(Self::BITS.div_ceil(Byte::BITS) == N as u32 && (if usize::BITS > 32 { N < u32::MAX as usize } else { true }), "`N` must be equal to `B.div_ceil(8)`, unless `B` is `0`");
        assert!(N != 0, "bnum types cannot be zero-sized");
        // `Self::BITS` must be at most `u32::MAX` (i.e. 2^32 - 1)
        // so `N` must be at most (2^32 - 1) / 8 = 2^29 - 1/8 = 2^29 - 1 (rounded down)
        assert!(Self::BITS >= 2, "bnum types must be at least 2 bits"); // having 1 bit is having a boolean so pointless
        if usize::BITS > 32 {
            assert!((Self::BITS as usize) < (1 << 32), "bnum types must be less than 2^32 bits");
        }
    };

    /// Creates a new integer from the given array of bytes. The first byte in the array is interpreted as the least significant byte, the last byte in the array is interpreted as the most significant byte.
    /// 
    /// If [`Self::BITS`] is not a multiple of `8`, then the high-order bits are ignored.
    /// 
    /// This method is equivalent to [`from_le_bytes`](Self::from_le_bytes).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// type U15 = t!(U15);
    /// 
    /// let bytes = [0x56, 0x34, 0x12];
    /// assert_eq!(U24::from_bytes(bytes), n!(0x123456));
    /// 
    /// let bytes = [0xFE, 0xDC, 0x7B];
    /// assert_eq!(I24::from_bytes(bytes), n!(0x7BDCFE));
    /// 
    /// assert_eq!(U15::from_bytes([0xFF, 0xFF]), U15::MAX); // = 0x7FFF. the high-order bit is ignored
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn from_bytes(digits: [Byte; N]) -> Self {
        // this is the only method where `Self` is explicitly constructed, all other methods use this one indirectly. thus, we can make all assertions about whether `N` is a valid size here.
        const { Self::ASSERT_IS_VALID };
        let mut out = Self { bytes: digits };
        out.set_sign_bits(); // this is important! as the only way of creating a new Integer, this needs to clean the input bytes
        out
    }

    /// Converts the slice of big endian bytes to an integer. An empty slice is interpreted as zero. If the value represented by the bytes is too large to be stored in `Self`, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let a: U24 = n!(0x005CF1);
    /// 
    /// let b = U24::from_be_slice(&[0x00, 0x5C, 0xF1]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = U24::from_be_slice(&[0x00, 0x00, 0x5C, 0xF1]);
    /// assert_eq!(c, Some(a));
    ///
    /// let d = U24::from_be_slice(&[0x00, 0x00, 0x00, 0x5C, 0xF1]);
    /// assert_eq!(d, Some(a));
    /// 
    /// let e = U24::from_be_slice(&[0x01, 0x00, 0x5C, 0xF1]);
    /// assert_eq!(e, None);
    /// 
    /// let a: I24 = n!(0xFF8A06).cast_signed();
    /// assert!(a.is_negative());
    /// 
    /// let b = I24::from_be_slice(&[0xFF, 0x8A, 0x06]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = I24::from_be_slice(&[0xFF, 0xFF, 0x8A, 0x06]);
    /// assert_eq!(c, Some(a));
    /// 
    /// let d = I24::from_be_slice(&[0xFF, 0xFF, 0xFF, 0x8A, 0x06]);
    /// assert_eq!(d, Some(a));
    /// 
    /// let e = I24::from_be_slice(&[0xFE, 0xFF, 0x8A, 0x06]);
    /// assert_eq!(e, None);
    /// ```
    #[must_use]
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        if slice.is_empty() {
            return Some(Self::ZERO);
        }
        let negative = S && (slice[0] as i8).is_negative();
        if slice.len() > N {
            let mut i = 0;
            while i < slice.len() - N {
                if !negative && slice[i] != 0 {
                    return None; // too large
                }
                if negative && slice[i] != 0xFF {
                    return None; // too small
                }
                i += 1;
            }
        }
        let mut bytes = if negative {
            [u8::MAX; N]
        } else {
            [0; N]
        };
        let mut i = N;
        while i > N.saturating_sub(slice.len()) {
            i -= 1;
            bytes[N - 1 - i] = slice[i + slice.len() - N];
        }
        // not bytes[0] as now the ordering is correct! (little endian)
        if !S && bytes[N - 1].leading_zeros() < Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        if S && !negative && bytes[N - 1].leading_zeros() <= Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        if negative && bytes[N - 1].leading_ones() < Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        Some(Self::from_bytes(bytes))
    }

    /// Converts the slice of little endian bytes to an integer. An empty slice is interpreted as zero. If the value represented by the bytes is too large to be stored in `Self`, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let a: U24 = n!(0x005CF1);
    /// let b = U24::from_le_slice(&[0xF1, 0x5C, 0x00]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = U24::from_le_slice(&[0xF1, 0x5C]);
    /// assert_eq!(c, Some(a));
    ///
    /// let d = U24::from_le_slice(&[0xF1, 0x5C, 0x00, 0x00, 0x00]);
    /// assert_eq!(d, Some(a));
    ///
    /// let e = U24::from_le_slice(&[0xF1, 0x5C, 0x00, 0x01]);
    /// assert_eq!(e, None);
    /// 
    /// let a: I24 = n!(0xFF8A06).cast_signed();
    /// assert!(a.is_negative());
    /// 
    /// let b = I24::from_le_slice(&[0x06, 0x8A, 0xFF]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = I24::from_le_slice(&[0x06, 0x8A]); // 0x8A has a leading one, so the slice represents a negative number
    /// assert_eq!(c, Some(a));
    ///
    /// let d = I24::from_le_slice(&[0x06, 0x8A, 0xFF, 0xFF, 0xFF]);
    /// assert_eq!(d, Some(a));
    ///
    /// let e = I24::from_le_slice(&[0x06, 0x8A, 0xFF, 0xFE]);
    /// assert_eq!(e, None);
    /// ```
    #[must_use]
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        if slice.is_empty() {
            return Some(Self::ZERO);
        }
        let negative = S && (slice[slice.len() - 1] as i8).is_negative();
        if slice.len() > N {
            let mut i = N;
            while i < slice.len() {
                if !negative && slice[i] != 0 {
                    return None; // too large
                }
                if negative && slice[i] != 0xFF {
                    return None; // too small
                }
                i += 1;
            }
        }
        let mut bytes = if negative {
            [u8::MAX; N]
        } else {
            [0; N]
        };
        let mut i = 0;
        while i < slice.len() && i < N {
            bytes[i] = slice[i];
            i += 1;
        }
        if !S && bytes[N - 1].leading_zeros() < Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        if S && !negative && bytes[N - 1].leading_zeros() <= Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        if negative && bytes[N - 1].leading_ones() < Self::LAST_BYTE_PAD_BITS {
            return None;
        }
        Some(Self::from_bytes(bytes))
    }
}

impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, 0, OM> {    
    /// Returns the representation of `self` as a byte array in big-endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let a: U24 = n!(0x3B2C4E);
    /// assert_eq!(a.to_be_bytes(), [0x3B, 0x2C, 0x4E]);
    /// 
    /// let b: I24 = n!(0xFF8A06).cast_signed();
    /// assert_eq!(b.to_be_bytes(), [0xFF, 0x8A, 0x06]);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N] {
        self.swap_bytes().to_le_bytes()
    }

    /// Returns the representation of `self` as a byte array in little-endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let a: U24 = n!(0x6FABD7);
    /// assert_eq!(a.to_le_bytes(), [0xD7, 0xAB, 0x6F]);
    /// 
    /// let b: I24 = n!(0x6F75FF);
    /// assert_eq!(b.to_le_bytes(), [0xFF, 0x75, 0x6F]);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.to_bytes()
    }

    /// Creates an integer value from a byte array in big-endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let bytes = [0x3B, 0x2C, 0x4E];
    /// assert_eq!(U24::from_be_bytes(bytes), n!(0x3B2C4E));
    /// 
    /// let bytes = [0x06, 0x8A, 0xFF];
    /// assert_eq!(I24::from_be_bytes(bytes), n!(0x068AFF));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
        Self::from_le_bytes(bytes).swap_bytes()
    }

    /// Creates an integer value from a byte array in little-endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// type I24 = Int<3>;
    /// 
    /// let bytes = [0xD7, 0xAB, 0x6F];
    /// assert_eq!(U24::from_le_bytes(bytes), n!(0x6FABD7));
    /// 
    /// let bytes = [0xFF, 0x75, 0x6F];
    /// assert_eq!(I24::from_le_bytes(bytes), n!(0x6F75FF));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self::from_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    macro_rules! test_from_endian_slice {
        ($TestType: ty, $endian: ident) => {
            paste::paste! {
                quickcheck::quickcheck! {
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $TestType _from_ $endian _slice>](int: $TestType, pad_length: u8) -> quickcheck::TestResult {
                        type TestType = $TestType;
                        type BaseType = [<$TestType Base>];

                        use crate::test::convert;

                        // pad_length is greater than the size of the integer in bytes
                        if pad_length >= BaseType::BITS as u8 / 8 {
                            return quickcheck::TestResult::discard();
                        }
                        let pad_length = pad_length as usize;

                        let mut pad_bits = if int.is_negative_internal() {
                            u8::MAX // 1111...
                        } else {
                            u8::MIN // 0000...
                        };

                        let mut bytes = int.[<to_ $endian _bytes>](); // random input bytes

                        // first, test that the original bytes as slice is converted back to the same integer
                        let mut passed = convert::test_eq(TestType::[<from_ $endian _slice>](&bytes[..]), Some(int));
                        
                        let bytes_vec = [<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length); // random vector padded with a random amount of bytes
                        // test that the padded bytes are still converted back to the same integer
                        passed &= convert::test_eq(TestType::[<from_ $endian _slice>](&bytes_vec[..]), Some(int));

                        
                        // most significant byte position, range of bytes indices to change to padding bits, range of bytes indices that will result in the same integer without the padding bits
                        let (msb, pad_range, slice_range) = [<$endian _pad>](pad_length, BaseType::BITS);

                        pad_bits = {
                            #[allow(unused_comparisons)]
                            if BaseType::MIN < 0 && (bytes[msb] as i8).is_negative() {
                                u8::MAX
                            } else {
                                u8::MIN
                            }
                        };

                        for item in &mut bytes[pad_range] {
                            *item = pad_bits;
                        }
                        let correct = Some(TestType::[<from_ $endian _bytes>](bytes));
                        // test that a shortened slice of bytes is converted to the same integer as the shortened slice that is padded to be the same number of bytes as the size of the integer
                        passed &= TestType::[<from_ $endian _slice>](&bytes[slice_range]) == correct;

                        let bytes_vec = [<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length);
                        passed &= TestType::[<from_ $endian _slice>](&bytes_vec[..]) == correct;

                        quickcheck::TestResult::from_bool(passed)
                    }
                }
            }
        };
    }

    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    #[cfg(feature = "alloc")]
    use core::ops::{Range, RangeFrom};

    #[cfg(feature = "alloc")]
    /// Pad a slice of bytes with leading pad bits so that the resulting vector of bytes represents the same integer as the original slice
    pub fn be_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
        let mut bytes_vec = vec![pad_bits; pad_length];
        bytes_vec.append(&mut bytes.to_vec());
        bytes_vec
    }

    #[cfg(all(test, feature = "alloc"))]
    pub fn be_pad(pad_length: usize, _bits: u32) -> (usize, Range<usize>, RangeFrom<usize>) {
        (pad_length, 0..pad_length, pad_length..)
    }

    #[cfg(feature = "alloc")]
    /// Pad a slice of bytes with trailing pad bits so that the resulting vector of bytes represents the same integer as the original slice
    pub fn le_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
        let mut bytes_vec = bytes.to_vec();
        bytes_vec.append(&mut vec![pad_bits; pad_length]);
        bytes_vec
    }

    #[cfg(feature = "alloc")]
    pub fn le_pad(pad_length: usize, bits: u32) -> (usize, Range<usize>, Range<usize>) {
        let bytes = bits as usize / 8;
        (
            bytes - 1 - pad_length,
            (bytes - pad_length)..bytes,
            0..(bytes - pad_length),
        )
    }

    use crate::test::test_bignum;

    crate::test::test_all! {
        testing integers;

        #[test]
        fn as_bytes() {
            let a = STest::ZERO;
            let digits = *a.as_bytes();
            assert_eq!(a, STest::from_bytes(digits));
        }

        #[test]
        fn to_bytes() {
            let a = STest::ZERO;
            assert_eq!(a.to_bytes(), [0; _]);
        }

        quickcheck::quickcheck! {
            fn as_bytes_mut(a: STest) -> bool {
                let a = STest::from_bytes(a.to_le_bytes());
                let mut b = a;
                b.as_bytes_mut().reverse();
                
                b == a.swap_bytes()
            }
        }

        test_bignum! {
            function: <STest>::to_be_bytes(a: STest)
        }
        test_bignum! {
            function: <STest>::to_le_bytes(a: STest)
        }
        test_bignum! {
            function: <STest>::from_be_bytes(a: [u8; STest::BYTES as usize])
        }
        test_bignum! {
            function: <STest>::from_le_bytes(a: [u8; STest::BYTES as usize])
        }

        #[cfg(feature = "alloc")]
        test_from_endian_slice!(STest, be);

        #[cfg(feature = "alloc")]
        test_from_endian_slice!(STest, le);

        use crate::n;

        #[test]
        fn cases_from_be_slice() {
            assert_eq!(STest::from_be_slice(&[]), Some(n!(0)));
        }
    }
}