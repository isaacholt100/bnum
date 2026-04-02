use super::{Integer, Uint};
use crate::doc;
use crate::{Byte, Exponent};

macro_rules! impl_desc {
    () => {
        "Methods for reading and manipulating the underlying bits of the integer."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(0b101101U1024).count_ones(), 4);
    /// assert_eq!(U1024::MAX.count_ones(), 1024);
    /// assert_eq!(U1024::MIN.count_ones(), 0);
    ///
    /// assert_eq!(n!(0b1110111I1024).count_ones(), 6);
    /// assert_eq!(I1024::MAX.count_ones(), 1023);
    /// assert_eq!(I1024::MIN.count_ones(), 1);
    /// assert_eq!(n!(-1I1024).count_ones(), 1024);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn count_ones(mut self) -> Exponent {
        if S {
            self.set_pad_bits(false); // don't want to count sign-extending pad bits
        }
        self.to_digits::<u32>().count_ones()
    }

    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(U512::MAX.count_zeros(), 0);
    /// assert_eq!(U512::MIN.count_zeros(), 512);
    ///
    /// assert_eq!(I512::MAX.count_zeros(), 1);
    /// assert_eq!(I512::MIN.count_zeros(), 511);
    /// assert_eq!(n!(-1I512).count_zeros(), 0);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn count_zeros(self) -> Exponent {
        Self::BITS - self.count_ones()
    }

    /// Returns the number of leading zeros in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(U256::MAX.leading_zeros(), 0);
    /// assert_eq!(U256::MIN.leading_zeros(), 256);
    /// assert_eq!(n!(1U256).leading_zeros(), 255);
    ///
    /// assert_eq!(I256::MAX.leading_zeros(), 1);
    /// assert_eq!(I256::MIN.leading_zeros(), 0);
    /// assert_eq!(n!(0I256).leading_zeros(), 256);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn leading_zeros(mut self) -> Exponent {
        // benchmarks show no quicker to use &self and as_digits
        self.set_pad_bits(false);
        self.to_digits::<u32>().leading_zeros() - Self::LAST_BYTE_PAD_BITS
    }

    // this method breaks early if the threshold is exceeded, which provides a speed up when converting between different width integer types
    pub(crate) const fn leading_ones_at_least_threshold(&self, threshold: Exponent) -> bool {
        self.leading_ones() >= threshold
    }

    /// Returns the number of leading ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(U1024::MAX.leading_ones(), 1024);
    /// assert_eq!(n!(0U1024).leading_ones(), 0);
    /// assert_eq!((U1024::MAX << 5u32).leading_ones(), 1019);
    ///
    /// assert_eq!(I1024::MIN.leading_ones(), 1);
    /// assert_eq!(I1024::MAX.leading_ones(), 0);
    /// assert_eq!((I1024::MIN >> 10u32).leading_ones(), 11);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn leading_ones(self) -> Exponent {
        self.not().leading_zeros()
    }

    /// Returns the number of trailing ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(U512::MAX.trailing_ones(), 512);
    /// assert_eq!(n!(0U512).trailing_ones(), 0);
    /// assert_eq!((U512::MAX >> 9u32).trailing_ones(), 503);
    ///
    /// assert_eq!(I512::MIN.trailing_ones(), 0);
    /// assert_eq!(I512::MAX.trailing_ones(), 511);
    /// assert_eq!((I512::MAX >> 6u32).trailing_ones(), 505);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn trailing_ones(self) -> Exponent {
        self.not().trailing_zeros()
    }

    // this method breaks early if the threshold is exceeded, which provides a speed up when converting between different width integer types
    pub(crate) const fn leading_zeros_at_least_threshold(&self, threshold: Exponent) -> bool {
        self.leading_zeros() >= threshold
    }

    /// Returns the number of trailing zeros in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(U2048::MAX.trailing_zeros(), 0);
    /// assert_eq!(n!(0U2048).trailing_zeros(), 2048);
    /// assert_eq!(U2048::power_of_two(279).trailing_zeros(), 279);
    ///
    /// assert_eq!(I2048::MAX.trailing_zeros(), 0);
    /// assert_eq!(I2048::MIN.trailing_zeros(), 2047);
    /// assert_eq!(n!(-16I2048).trailing_zeros(), 4);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn trailing_zeros(self) -> Exponent {
        let tz = self.to_digits::<u32>().trailing_zeros();
        if tz >= Self::BITS { Self::BITS } else { tz }
    }

    /// Rotates the bits of `self` to the left by `n` places.
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
    /// let a: U24 = n!(0x3D2A17);
    /// assert_eq!(a.rotate_left(12), n!(0xA173D2));
    ///
    /// let b: I24 = n!(0x7C34AE);
    /// assert_eq!(b.rotate_left(8), n!(0x34AE7C));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rotate_left(mut self, n: Exponent) -> Self {
        if Self::LAST_BYTE_PAD_BITS != 0 {
            self.set_pad_bits(false);
            let wide = Integer::<false, N, 0, OM>::from_bytes(self.bytes);
            let a = wide.wrapping_shl(n % Self::BITS);
            let b = wide.wrapping_shr(Self::BITS - (n % Self::BITS));
            let o = a.bitor(b);
            let mut out = Self { bytes: o.bytes };
            out.set_sign_bits();
            out
        } else {
            // u32 is fastest
            unsafe { self.to_digits::<u32>().unchecked_rotate_left(n % Self::BITS).to_integer() }
        }
    }

    // TODO: maybe add const `cast` function to cast to other integer with different params. would behave differently depending on overflow mode

    /// Rotates the bits of `self` to the right by `n` places.
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
    /// let a: U24 = n!(0x8427AB);
    /// assert_eq!(a.rotate_right(4), 0xB8427A.as_());
    ///
    /// let b: I24 = n!(0x4ACD8A);
    /// assert_eq!(b.rotate_right(16), 0xCD8A4A.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rotate_right(self, n: Exponent) -> Self {
        self.rotate_left(Self::BITS - (n % Self::BITS))
    }

    /// Left-shifts `self` by `rhs` bits. If `rhs` is larger than or equal to `Self::BITS`, the entire value is shifted out and zero is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(1U2048).unbounded_shl(1), n!(2));
    /// assert_eq!(U2048::MAX.unbounded_shl(2048), n!(0));
    /// assert_eq!(U2048::MAX.unbounded_shl(2049), n!(0));
    ///
    /// assert_eq!(n!(1).unbounded_shl(2047), I2048::MIN);
    /// assert_eq!(I2048::MAX.unbounded_shl(2048), n!(0));
    /// assert_eq!(I2048::MIN.unbounded_shl(2049), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn unbounded_shl(self, rhs: Exponent) -> Self {
        if rhs >= Self::BITS {
            Self::ZERO
        } else {
            unsafe { self.unchecked_shl_internal(rhs) }
        }
    }

    /// Right-shifts `self` by `rhs` bits. If `rhs` is larger than or equal to `Self::BITS`, then the entire value is shifted out, and:
    /// - for unsigned integers, `0` is returned.
    /// - for signed integers, `-1` is returned if `self` is negative, and `0` is returned if `self` is non-negative.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(2U1024).unbounded_shr(1), n!(1));
    /// assert_eq!(U1024::MAX.unbounded_shr(1024), n!(0));
    /// assert_eq!(U1024::MAX.unbounded_shr(1030), n!(0));
    ///
    /// assert_eq!(I1024::MIN.unbounded_shr(1023), n!(-1));
    /// assert_eq!(n!(-1I1024).unbounded_shr(1024), n!(-1));
    /// assert_eq!(I1024::MAX.unbounded_shr(1025), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn unbounded_shr(self, rhs: Exponent) -> Self {
        if rhs >= Self::BITS {
            if self.is_negative_internal() {
                Self::ALL_ONES // i.e. -1
            } else {
                Self::ZERO
            }
        } else {
            unsafe { self.unchecked_shr_internal(rhs) }
        }
    }

    /// Reverses the order of the bits of `self`.
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
    /// let a: U24 = 0b10110011_11001010_00011101.as_();
    /// assert_eq!(a.reverse_bits(), 0b10111000_01010011_11001101.as_());
    ///
    /// let b: I24 = 0b01101001_00111100_11100011.as_();
    /// assert_eq!(b.reverse_bits(), 0b11000111_00111100_10010110.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
        if Self::LAST_BYTE_PAD_BITS != 0 {
            let mut out = self.force::<S, 0, OM>().reverse_bits();
            out = out.shr(Self::LAST_BYTE_PAD_BITS); // this will sign-extend, so no need to call set_sign_bits
            return out.force();
        }

        self.to_digits::<u8>().reverse_bits().to_integer() // 1 byte is faster than any other digit size
    }

    #[inline]
    pub(crate) const unsafe fn unchecked_shl_internal(self, rhs: Exponent) -> Self {
        let mut out = unsafe { self.to_digits::<u64>().unchecked_shl(rhs).to_integer() };

        out.set_sign_bits();

        out
    }

    #[inline]
    pub(crate) const unsafe fn unchecked_shr_internal(self, rhs: Exponent) -> Self {
        if Self::LAST_BYTE_PAD_BITS != 0 {
            let out = unsafe {
                self.widen().unchecked_shr_internal(rhs)
            };
            return out.force();
        }
        unsafe { self.to_digits::<u64>().unchecked_shr(rhs, self.is_negative_internal()).to_integer() }
    }

    /// Returns a boolean representing the bit in the given position (`true` if the bit is 1). The least significant bit is at index `0`, the most significant bit is at index `Self::BITS - 1`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// let a = n!(0b1101001101U24);
    /// for i in [0, 2, 3, 6, 8, 9] {
    ///     assert!(a.bit(i));
    /// }
    ///
    /// let b = n!(0b0010110010I24);
    /// for i in [1, 4, 5, 7] {
    ///     assert!(b.bit(i));
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn bit(&self, index: Exponent) -> bool {
        let byte = self.bytes[index as usize / Byte::BITS as usize];
        byte & (1 << (index % Byte::BITS)) != 0
    }

    /// Sets/unsets the bit in the given position (i.e. to `1` if `value` is `true`). The least significant bit is at index `0`, the most significant bit is at index `Self::BITS - 1`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// let mut a = n!(0b1001011001001U24);
    /// a.set_bit(2, true);
    /// assert_eq!(a, n!(0b1001011001101));
    /// a.set_bit(1, false); // no change
    /// assert_eq!(a, n!(0b1001011001101));
    ///
    /// let mut b = n!(0b010010110110100I24);
    /// b.set_bit(4, false);
    /// assert_eq!(b, n!(0b010010110100100));
    /// b.set_bit(0, false); // no change
    /// assert_eq!(b, n!(0b010010110100100));
    /// ```
    #[inline]
    pub const fn set_bit(&mut self, index: Exponent, value: bool) {
        let digit = &mut self.bytes[index as usize / Byte::BITS as usize];
        let shift = index % Byte::BITS;
        *digit = *digit & !(1 << shift) | ((value as Byte) << shift);
    }
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, 0, OM> {
    /// Reverses the order of the bytes of `self`.
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
    /// let a: U24 = n!(0x7C283D);
    /// assert_eq!(a.swap_bytes(), n!(0x3D287C));
    ///
    /// let b: I24 = n!(0x1DC87B);
    /// assert_eq!(b.swap_bytes(), n!(0x7BC81D));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
        // let mut bytes = self.bytes;
        // bytes.reverse(); // this is faster than manually swapping using uN::swap_bytes
        // Self { bytes }
        self.to_digits::<u64>().swap_bytes().to_integer() // u8 is fastest
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    /// Returns the smallest number of bits necessary to represent `self`.
    ///
    /// This is equal to the size of the type in bits minus the leading zeros of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    ///
    /// assert_eq!(U256::MAX.bit_width(), 256);
    /// assert_eq!(n!(0U256).bit_width(), 0);
    /// ```
    #[must_use]
    #[inline]
    pub const fn bit_width(self) -> Exponent {
        self.to_digits::<u32>().bit_width()
    }
}

// #[test]
// fn test_rev_bits() {
//     use crate::n;
//     let a = n!(0b11001110U8).cast_signed();
//     let b = n!(0b10111000U8).cast_signed();

//     dbg!(a == b);
// }

#[cfg(test)]
mod tests {
    use crate::cast::CastFrom;
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing integers;

        #[test]
        fn bit() {
            let u = STEST::cast_from(0b001010100101010101u64);
            assert!(u.bit(0));
            assert!(!u.bit(1));
            // assert!(!u.bit(17));
            // assert!(!u.bit(16));
            assert!(u.bit(15));
        }

        #[test]
        fn set_bit() {
            let mut u = STEST::cast_from(0b001010100101010101u64);
            u.set_bit(1, true);
            assert!(u.bit(1));
            u.set_bit(1, false);
            assert!(!u.bit(1));
            u.set_bit(14, false);
            assert!(!u.bit(14));
            u.set_bit(14, true);
            assert!(u.bit(14));
        }

        test_bignum! {
            function: <stest>::count_ones(a: stest)
        }
        test_bignum! {
            function: <stest>::count_zeros(a: stest)
        }
        test_bignum! {
            function: <stest>::leading_zeros(a: stest)
        }
        test_bignum! {
            function: <stest>::trailing_zeros(a: stest)
        }
        test_bignum! {
            function: <stest>::leading_ones(a: stest)
        }
        test_bignum! {
            function: <stest>::trailing_ones(a: stest)
        }
        test_bignum! {
            function: <stest>::rotate_left(a: stest, b: u8)
        }
        test_bignum! {
            function: <stest>::rotate_right(a: stest, b: u8)
        }
        test_bignum! {
            function: <stest>::unbounded_shl(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::unbounded_shr(a: stest, b: u16),
            cases: [
                (stest::MIN, stest::BITS as u16 - 1)
            ]
        }
        test_bignum! {
            function: <stest>::swap_bytes(a: stest)
        }
        test_bignum! {
            function: <stest>::reverse_bits(a: stest)
        }
    }

    crate::test::test_all! {
        testing unsigned;

        #[cfg(nightly)]
        test_bignum! {
            function: <stest>::bit_width(a: stest)
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::trailing_zeros(a: utest)
    }
    test_bignum! {
        function: <itest>::trailing_zeros(a: itest)
    }
    test_bignum! {
        function: <utest>::trailing_ones(a: utest)
    }
    test_bignum! {
        function: <itest>::trailing_ones(a: itest)
    }
    test_bignum! {
        function: <utest>::leading_zeros(a: utest)
    }
    test_bignum! {
        function: <itest>::leading_zeros(a: itest)
    }
    test_bignum! {
        function: <utest>::leading_ones(a: utest)
    }
    test_bignum! {
        function: <itest>::leading_ones(a: itest)
    }
    test_bignum! {
        function: <utest>::count_ones(a: utest)
    }
    test_bignum! {
        function: <itest>::count_ones(a: itest)
    }
    test_bignum! {
        function: <utest>::count_zeros(a: utest)
    }
    test_bignum! {
        function: <itest>::count_zeros(a: itest)
    }
    test_bignum! {
        function: <utest>::rotate_left(a: utest, b: u32)
    }
    test_bignum! {
        function: <itest>::rotate_left(a: itest, b: u32)
    }
    test_bignum! {
        function: <utest>::rotate_right(a: utest, b: u32)
    }
    test_bignum! {
        function: <itest>::rotate_right(a: itest, b: u32)
    }
    test_bignum! {
        function: <utest>::reverse_bits(a: utest)
    }
    test_bignum! {
        function: <itest>::reverse_bits(a: itest)
    }
}

#[cfg(test)]
mod swap_bytes_tests {
    macro_rules! test_swap_bytes {
        ($($bits: literal), *) => {
            paste::paste! {
                $(
                    quickcheck::quickcheck! {
                        fn [<quickcheck_swap_bytes_u $bits>](a: crate::Uint<{$bits / 8}>) -> bool {
                            let b = a.swap_bytes();
                            let mut bytes = a.to_bytes();
                            bytes.reverse();

                            b.to_bytes() == bytes
                        }

                        fn [<quickcheck_swap_bytes_i $bits>](a: crate::Int<{$bits / 8}>) -> bool {
                            let b = a.swap_bytes();
                            let mut bytes = a.to_bytes();
                            bytes.reverse();

                            b.to_bytes() == bytes
                        }
                    }
                )*
            }
        };
    }
    test_swap_bytes!(8, 16, 32, 64, 128, 256, 512, 56, 144, 160, 488);
}
