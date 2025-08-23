use super::Int;
use crate::Uint;

use crate::doc;

#[doc = doc::endian::impl_desc!(Int)]
impl<const N: usize> Int<N> {
    #[doc = doc::endian::from_be!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        Self::from_bits(Uint::from_be(x.bits))
    }

    #[doc = doc::endian::from_le!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        Self::from_bits(Uint::from_le(x.bits))
    }

    #[doc = doc::endian::to_be!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc = doc::endian::to_le!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
    }

    /// Converts the slice of big endian bytes to an integer. An empty slice is interpreted as zero. If the value represented by the bytes is too large to be stored in `Self`, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type I24 = Int<3>;
    /// 
    /// let a: I24 = 0x00_5C_F1.as_();
    /// assert!(a.is_positive());
    /// 
    /// let b = I24::from_be_slice(&[0x00, 0x5C, 0xF1]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = I24::from_be_slice(&[0x5C, 0xF1]);
    /// assert_eq!(c, Some(a));
    ///
    /// let d = I24::from_be_slice(&[0x00, 0x00, 0x00, 0x5C, 0xF1]);
    /// assert_eq!(d, Some(a));
    ///
    /// let e = I24::from_be_slice(&[0x01, 0x00, 0x5C, 0xF1]);
    /// assert_eq!(e, None);
    /// 
    /// let a: I24 = 0xFF_8A_06.as_();
    /// assert!(a.is_negative());
    /// 
    /// let b = I24::from_be_slice(&[0xFF, 0x8A, 0x06]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = I24::from_be_slice(&[0x8A, 0x06]); // 0x8A has a leading one, so the slice represents a negative number
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
        let negative = (slice[0] as i8).is_negative();
        if slice.len() > N {
            let mut i = 0;
            while i < slice.len() - N {
                if slice[i] != 0 && !negative {
                    return None; // too large
                }
                if slice[i] != 0xFF && negative {
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
        Some(Uint::from_digits(bytes).cast_signed())
    }

    /// Converts the slice of little endian bytes to an integer. An empty slice is interpreted as zero. If the value represented by the bytes is too large to be stored in `Self`, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type I24 = Int<3>;
    /// 
    /// let a: I24 = 0x00_5C_F1.as_();
    /// assert!(a.is_positive());
    /// 
    /// let b = I24::from_le_slice(&[0xF1, 0x5C, 0x00]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = I24::from_le_slice(&[0xF1, 0x5C]);
    /// assert_eq!(c, Some(a));
    ///
    /// let d = I24::from_le_slice(&[0xF1, 0x5C, 0x00, 0x00, 0x00]);
    /// assert_eq!(d, Some(a));
    ///
    /// let e = I24::from_le_slice(&[0xF1, 0x5C, 0x00, 0x01]);
    /// assert_eq!(e, None);
    /// 
    /// let a: I24 = 0xFF_8A_06.as_();
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
        let negative = (slice[slice.len() - 1] as i8).is_negative();
        if slice.len() > N {
            let mut i = N;
            while i < slice.len() {
                if slice[i] != 0 && !negative {
                    return None; // too large
                }
                if slice[i] != 0xFF && negative {
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
        Some(Uint::from_digits(bytes).cast_signed())
    }

    #[doc = doc::endian::to_be_bytes!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N] {
        self.bits.to_be_bytes()
    }

    #[doc = doc::endian::to_le_bytes!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.bits.to_le_bytes()
    }

    #[doc = doc::endian::to_ne_bytes!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
        self.bits.to_ne_bytes()
    }

    #[doc = doc::endian::from_be_bytes!(I)]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
        Self::from_bits(Uint::from_be_bytes(bytes))
    }

    #[doc = doc::endian::from_le_bytes!(I)]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self::from_bits(Uint::from_le_bytes(bytes))
    }

    #[doc = doc::endian::from_ne_bytes!(I)]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
        Self::from_bits(Uint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    crate::ints::endian::tests!(itest);
}
