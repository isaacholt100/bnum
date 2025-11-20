use super::{Uint, Integer};
use crate::Exponent;
use crate::doc;
use crate::digit::Digit;

macro_rules! impl_desc {
    () => {
        "Methods for reading and manipulating the underlying bits of the integer."
    }
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, OM> {
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
    /// assert_eq!(n!(0b101101 U1024).count_ones(), 4);
    /// assert_eq!(U1024::MAX.count_ones(), 1024);
    /// assert_eq!(U1024::MIN.count_ones(), 0);
    /// 
    /// assert_eq!(n!(0b1110111 I1024).count_ones(), 6);
    /// assert_eq!(I1024::MAX.count_ones(), 1023);
    /// assert_eq!(I1024::MIN.count_ones(), 1);
    /// assert_eq!(n!(-1 I1024).count_ones(), 1024);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn count_ones(self) -> Exponent {
        let mut ones = 0;
        let mut i = 0;
        while i < N {
            ones += self.bytes[i].count_ones() as Exponent;
            i += 1;
        }
        ones
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
    /// assert_eq!(n!(-1 I512).count_zeros(), 0);
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
    /// assert_eq!(n!(1 U256).leading_zeros(), 255);
    /// 
    /// assert_eq!(I256::MAX.leading_zeros(), 1);
    /// assert_eq!(I256::MIN.leading_zeros(), 0);
    /// assert_eq!(n!(0 I256).leading_zeros(), 256);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn leading_zeros(self) -> Exponent {
        // don't need to use larger digits, the compiler optimises this code well
        let mut zeros = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.bytes[i];
            zeros += digit.leading_zeros();
            if digit != Digit::MIN {
                break;
            }
        }
        zeros
    }

    // this method breaks early if the threshold is exceeded, which provides a speed up when converting between different width integer types
    pub(crate) const fn leading_zeros_at_least_threshold(&self, threshold: Exponent) -> bool {
        // don't need to use larger digits, the compiler optimises this code well
        let mut zeros = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.bytes[i];
            zeros += digit.leading_zeros();
            if zeros >= threshold {
                return true;
            }
            if digit != Digit::MIN {
                break;
            }
        }
        false
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
    /// assert_eq!(n!(0 U2048).trailing_zeros(), 2048);
    /// assert_eq!(U2048::power_of_two(279).trailing_zeros(), 279);
    /// 
    /// assert_eq!(I2048::MAX.trailing_zeros(), 0);
    /// assert_eq!(I2048::MIN.trailing_zeros(), 2047);
    /// assert_eq!(n!(-16 I2048).trailing_zeros(), 4);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn trailing_zeros(self) -> Exponent {
        let mut zeros = 0;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS - 1 {
                let digit = self.as_wide_digits().get(i);
                let tz = digit.trailing_zeros();
                zeros += tz;
                if tz != u128::BITS {
                    return zeros;
                }
                i += 1;
            }
        }
        // zeros
        let last_tz = self.as_wide_digits().last_padded::<true>().trailing_zeros();
        zeros + last_tz
    }

    // this method breaks early if the threshold is exceeded, which provides a speed up when converting between different width integer types
    pub(crate) const fn leading_ones_at_least_threshold(&self, threshold: Exponent) -> bool {
        // don't need to use larger digits, the compiler optimises this code well
        let mut ones = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.bytes[i];
            ones += digit.leading_ones();
            if ones >= threshold {
                return true;
            }
            if digit != Digit::MAX {
                break;
            }
        }
        false
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
    /// assert_eq!(n!(0 U1024).leading_ones(), 0);
    /// assert_eq!((U1024::MAX << 5u32).leading_ones(), 1019);
    /// 
    /// assert_eq!(I1024::MIN.leading_ones(), 1);
    /// assert_eq!(I1024::MAX.leading_ones(), 0);
    /// assert_eq!((I1024::MIN >> 10u32).leading_ones(), 11);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn leading_ones(self) -> Exponent {
        // don't need to use larger digits, the compiler optimises this code well
        let mut ones = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.bytes[i];
            ones += digit.leading_ones();
            if digit != Digit::MAX {
                break;
            }
        }
        ones
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
    /// assert_eq!(n!(0 U512).trailing_ones(), 0);
    /// assert_eq!((U512::MAX >> 9u32).trailing_ones(), 503);
    /// 
    /// assert_eq!(I512::MIN.trailing_ones(), 0);
    /// assert_eq!(I512::MAX.trailing_ones(), 511);
    /// assert_eq!((I512::MAX >> 6u32).trailing_ones(), 505);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn trailing_ones(self) -> Exponent {
        let mut ones = 0;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let digit = self.as_wide_digits().get(i);
                let to = digit.trailing_ones();
                ones += to;
                if to != u128::BITS {
                    return ones;
                }
                i += 1;
            }
        }
        ones
    }

    #[inline]
    const unsafe fn rotate_digits_left(self, n: usize) -> Self {
        // this is no slower than using pointers with add and copy_from_nonoverlapping
        let mut out = Self::ZERO;
        let mut i = n;
        while i < N {
            out.bytes[i] = self.bytes[i - n];
            i += 1;
        }
        let init_index = N - n;
        let mut i = init_index;
        while i < N {
            out.bytes[i - init_index] = self.bytes[i];
            i += 1;
        }

        out
    }

    #[inline]
    const unsafe fn unchecked_rotate_left(self, rhs: Exponent) -> Self {
        unsafe {
            let digit_shift = (rhs / 8) as usize;
            let bit_shift = rhs % 8;

            let mut out = self.rotate_digits_left(digit_shift);

            if bit_shift != 0 {
                let carry_shift = 128 - bit_shift;
                let mut carry =
                    out.as_wide_digits().last() >> (8 * Self::LAST_DIGIT_BYTES as u32 - bit_shift);

                let mut i = 0;
                while i < Self::U128_DIGITS {
                    let current_digit = out.as_wide_digits().get(i);
                    out.as_wide_digits_mut()
                        .set(i, (current_digit << bit_shift) | carry);
                    carry = current_digit >> carry_shift;
                    i += 1;
                }
            }

            out
        }
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
    pub const fn rotate_left(self, n: Exponent) -> Self {
        unsafe { self.unchecked_rotate_left(n % Self::BITS) }
    }

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
        let n = n % Self::BITS;
        unsafe { self.unchecked_rotate_left(Self::BITS - n) }
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
    /// assert_eq!(n!(1 U2048).unbounded_shl(1), n!(2));
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
    /// assert_eq!(n!(2 U1024).unbounded_shr(1), n!(1));
    /// assert_eq!(U1024::MAX.unbounded_shr(1024), n!(0));
    /// assert_eq!(U1024::MAX.unbounded_shr(1030), n!(0));
    /// 
    /// assert_eq!(I1024::MIN.unbounded_shr(1023), n!(-1));
    /// assert_eq!(n!(-1 I1024).unbounded_shr(1024), n!(-1));
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
        } else if self.is_negative_internal() {
            unsafe { self.unchecked_shr_pad_internal::<true>(rhs) }
        } else {
            unsafe { self.unchecked_shr_pad_internal::<false>(rhs) }
        }
    }

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
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < Self::U128_DIGITS {
            unsafe {
                let d = self.as_wide_digits().get(i);
                out.as_wide_digits_mut().set_be(i, d.swap_bytes());
            }

            i += 1;
        }
        out
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
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < Self::U128_DIGITS {
            unsafe {
                let d = self.as_wide_digits().get(i);
                out.as_wide_digits_mut().set_be(i, d.reverse_bits());
            }

            i += 1;
        }
        out
    }

    #[inline]
    pub(crate) const unsafe fn unchecked_shl_internal(self, rhs: Exponent) -> Self {
        let mut out = Self::ZERO;
        let digit_shift = (rhs / 8) as usize;
        let bit_shift = rhs % 8;

        let mut i = digit_shift;
        while i < N {
            // we start i at digit_shift, not 0, since the compiler can elide bounds checks when i < N
            // this is no slower than using pointers with add and copy_from_nonoverlapping
            out.bytes[i] = self.bytes[i - digit_shift];
            i += 1;
        }

        if bit_shift != 0 {
            let carry_shift = 128 - bit_shift;
            let mut carry = 0;

            let mut i = (rhs / 128) as usize;
            while i < Self::U128_DIGITS {
                let current_digit = unsafe { out.as_wide_digits().get(i) };
                unsafe { out.as_wide_digits_mut().set(i, (current_digit << bit_shift) | carry) };
                carry = current_digit >> carry_shift;
                i += 1;
            }
        }

        out
    }

    #[inline]
    pub(crate) const unsafe fn unchecked_shr_pad_internal<const NEG: bool>(
        self,
        rhs: Exponent,
    ) -> Self {
        let mut out = if NEG { Self::ALL_ONES } else { Self::ZERO };
        let digit_shift = (rhs / 8) as usize;
        let bit_shift = rhs % 8;
        
        unsafe {
            let num_copies = N.unchecked_sub(digit_shift);

            let mut i = digit_shift;
            while i < N {
                // we start i at digit_shift, not 0, since the compiler can elide bounds checks when i < N
                out.bytes[i - digit_shift] = self.bytes[i];
                i += 1;
            }

            if bit_shift != 0 {
                let carry_shift = 128 - bit_shift;
                let mut carry = if NEG {
                    u128::MAX << carry_shift // if negative, then we initialise the carry to have the correct number of sign bits (we can get this expression by looking for the expression for carry shift in the while loop. the previous digit will have been all ones (unless this was the last digit, but then we can view it as having infinite leading ones, as this still represents the same value))
                } else {
                    0
                };

                let mut i = (Self::BITS - rhs).div_ceil(128) as usize; // we could just start at U128_DIGITS, but then we would just be shifting all ones/all zeros for the unaffected digits (at higher indices) which would do nothing
                while i > 0 {
                    i -= 1;

                    let current_digit = out.as_wide_digits().get(i);
                    out.as_wide_digits_mut().set(i, (current_digit >> bit_shift) | carry);
                    carry = current_digit << carry_shift;
                }

                // let mut i = digit_shift;
                // while i + 15 < N {
                //     let offset = N - 16 - i;
                //     let current_digit = out.as_wide_digits().get_at_offset(offset);
                //     out.as_wide_digits_mut()
                //         .set_at_offset(offset, (current_digit >> bit_shift) | carry);
                //     carry = current_digit << carry_shift;
                //     i += 16;
                // }
                // let rem = (N - digit_shift) % 16;
                // if rem != 0 {
                //     let mut bytes = [0; 16];
                //     bytes
                //         .as_mut_ptr()
                //         .add(16 - rem)
                //         .copy_from_nonoverlapping(out.digits.as_ptr(), rem);
                //     let digit = u128::from_le_bytes(bytes);
                //     let shifted = (digit >> bit_shift) | carry;
                //     let bytes = shifted.to_le_bytes();
                //     out.digits
                //         .as_mut_ptr()
                //         .copy_from_nonoverlapping(bytes.as_ptr().add(16 - rem), rem);
                // }

                // if NEG {
                //     out.digits[num_copies - 1] |= Digit::MAX << (8 - bit_shift);
                // }
            }

            out
        }
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
    /// let a = n!(0b1101001101 U24);
    /// for i in [0, 2, 3, 6, 8, 9] {
    ///     assert!(a.bit(i));
    /// }
    /// 
    /// let b = n!(0b0010110010 I24);
    /// for i in [1, 4, 5, 7] {
    ///     assert!(b.bit(i));
    /// }
    /// ```
    #[must_use]
    #[inline]
    pub const fn bit(&self, index: Exponent) -> bool {
        let digit = self.bytes[index as usize / Digit::BITS as usize];
        digit & (1 << (index % Digit::BITS)) != 0
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
    /// let mut a = n!(0b1001011001001 U24);
    /// a.set_bit(2, true);
    /// assert_eq!(a, n!(0b1001011001101));
    /// a.set_bit(1, false); // no change
    /// assert_eq!(a, n!(0b1001011001101));
    /// 
    /// let mut b = n!(0b010010110110100 I24);
    /// b.set_bit(4, false);
    /// assert_eq!(b, n!(0b010010110100100));
    /// b.set_bit(0, false); // no change
    /// assert_eq!(b, n!(0b010010110100100));
    /// ```
    #[inline]
    pub const fn set_bit(&mut self, index: Exponent, value: bool) {
        let digit = &mut self.bytes[index as usize / Digit::BITS as usize];
        let shift = index % Digit::BITS;
        *digit = *digit & !(1 << shift) | ((value as Digit) << shift);
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const OM: u8> Uint<N, OM> {
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
    /// assert_eq!(U256::MAX.bits(), 256);
    /// assert_eq!(n!(0 U256).bits(), 0);
    /// ```
    #[must_use]
    #[inline]
    pub const fn bits(&self) -> Exponent {
        Self::BITS as Exponent - self.leading_zeros()
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::cast::CastFrom;

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

        #[test]
        fn bits() {
            let u = STEST::cast_from(0b1010100101010101u128);
            assert_eq!(u.bits(), 16);

            let u: STEST = STEST::ONE << 7;
            assert_eq!(u.bits(), 8);
        }
    }
}


#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::trailing_zeros(a: utest)
    }
    test_bignum! {
        function: <utest>::trailing_ones(a: utest)
    }
    test_bignum! {
        function: <utest>::rotate_left(a: utest, b: u32)
    }
    test_bignum! {
        function: <utest>::rotate_right(a: utest, b: u32)
    }
    test_bignum! {
        function: <utest>::reverse_bits(a: utest)
    }
    test_bignum! {
        function: <utest>::swap_bytes(a: utest)
    }
}