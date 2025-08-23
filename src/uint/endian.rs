use super::Uint;
use crate::doc;

#[doc = doc::endian::impl_desc!(Uint)]
impl<const N: usize> Uint<N> {
    /// Converts `x` from big endian to the target platform's endianness.
    /// 
    /// If the target endianness is big endian, this is a no-op. If the target endianness is little endian, this swaps the bytes of `x`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let x: U24 = 0x3B_2C_4E.as_();
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(U24::from_be(x), x);
    /// } else {
    ///     assert_eq!(U24::from_be(x), 0x4E_2C_3B.as_());
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        return x;
        #[cfg(not(target_endian = "big"))]
        x.swap_bytes()
    }

    /// Converts `x` from little endian to the target platform's endianness.
    /// 
    /// If the target endianness is little endian, this is a no-op. If the target endianness is big endian, this swaps the bytes of `x`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let x: U24 = 0x6F_AB_D7.as_();
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(U24::from_le(x), x);
    /// } else {
    ///     assert_eq!(U24::from_le(x), 0xD7_AB_6F.as_());
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        return x;
        #[cfg(not(target_endian = "little"))]
        x.swap_bytes()
    }

    /// Convert `self` to big endian from the target platform's endianness.
    /// 
    /// If the target endianness is big endian, this is a no-op. If the target endianness is little endian, this swaps the bytes of `self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let x: U24 = 0x3B_2C_4E.as_();
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(x.to_be(), x);
    /// } else {
    ///     assert_eq!(x.to_be(), 0x4E_2C_3B.as_());
    /// }
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    /// Convert `self` to little endian from the target platform's endianness.
    ///
    /// If the target endianness is little endian, this is a no-op. If the target endianness is big endian, this swaps the bytes of `self`.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let x: U24 = 0x6F_AB_D7.as_();
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(x.to_le(), x);
    /// } else {
    ///     assert_eq!(x.to_le(), 0xD7_AB_6F.as_());
    /// }
    /// ```
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
    /// type U24 = Uint<3>;
    /// 
    /// let a: U24 = 0x00_5C_F1.as_();
    /// let b = U24::from_be_slice(&[0x5C, 0xF1, 0x00]);
    /// assert_eq!(b, Some(a));
    /// 
    /// let c = U24::from_be_slice(&[0x5C, 0xF1]);
    /// assert_eq!(c, Some(a));
    ///
    /// let d = U24::from_be_slice(&[0x5C, 0xF1, 0x00, 0x00, 0x00]);
    /// assert_eq!(d, Some(a));
    /// 
    /// let e = U24::from_be_slice(&[0x5C, 0xF1, 0x00, 0x01]);
    /// assert_eq!(e, None);
    /// ```
    #[must_use]
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let mut out = Self::ZERO;
        if slice.len() > N {
            let mut i = 0;
            while i < slice.len() - N {
                if slice[i] != 0 {
                    return None; // too large
                }
                i += 1;
            }
        }
        let mut i = N;
        while i > N.saturating_sub(slice.len()) {
            i -= 1;
            out.digits[N - 1 - i] = slice[i + slice.len() - N];
        }
        Some(out)
    }

    /// Converts the slice of little endian bytes to an integer. An empty slice is interpreted as zero. If the value represented by the bytes is too large to be stored in `Self`, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let a: U24 = 0x00_5C_F1.as_();
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
    /// ```
    #[must_use]
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        if slice.len() > N {
            let mut i = N;
            while i < slice.len() {
                if slice[i] != 0 {
                    return None; // too large
                }
                i += 1;
            }
        }
        let mut out = Self::ZERO;

        let mut i = 0;
        while i < slice.len() && i < N {
            out.digits[i] = slice[i];
            i += 1;
        }
        Some(out)
    }

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
    /// 
    /// let x: U24 = 0x3B_2C_4E.as_();
    /// assert_eq!(x.to_be_bytes(), [0x3B, 0x2C, 0x4E]);
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
    /// 
    /// let x: U24 = 0x6F_AB_D7.as_();
    /// assert_eq!(x.to_le_bytes(), [0xD7, 0xAB, 0x6F]);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.digits
    }

    /// Returns the representation of `self` as a byte array in native endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let x: U24 = 0x76_C4_3B.as_();
    /// if cfg!(target_endian = "big") {
    ///    assert_eq!(x.to_ne_bytes(), [0x3B, 0xC4, 0x76]);
    /// } else {
    ///    assert_eq!(x.to_ne_bytes(), [0x76, 0xC4, 0x3B]);
    /// }
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
        #[cfg(target_endian = "big")]
        return self.to_be_bytes();

        #[cfg(not(target_endian = "big"))]
        self.to_le_bytes()
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
    /// 
    /// let bytes = [0x3B, 0x2C, 0x4E];
    /// assert_eq!(U24::from_be_bytes(bytes), 0x3B_2C_4E.as_());
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
    /// 
    /// let bytes = [0xD7, 0xAB, 0x6F];
    /// assert_eq!(U24::from_le_bytes(bytes), 0x6F_AB_D7.as_());
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self::from_digits(bytes)
    }

    /// Creates an integer value from a byte array in native endian order.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// type U24 = Uint<3>;
    /// 
    /// let bytes = [0x76, 0xC4, 0x3B];
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(U24::from_ne_bytes(bytes), 0x3B_C4_76.as_());
    /// } else {
    ///     assert_eq!(U24::from_ne_bytes(bytes), 0x76_C4_3B.as_());
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(bytes);

        #[cfg(not(target_endian = "big"))]
        Self::from_le_bytes(bytes)
    }
}

#[test]
fn test_endian() {
    let a = crate::Int::<2>::from_le_slice(&[128, 0, 0]);
    // println!("{:016b}", a.unwrap());
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    crate::ints::endian::tests!(utest);
}
