use crate::digit;
use crate::doc;
use crate::wide_digits::WideDigits;
use crate::{Integer, Uint};

/// Bigint helper methods: common functions used to implement big integer arithmetic.
impl<const S: bool, const N: usize> Integer<S, N> {
    /// Computes `self + rhs + carry`, and returns a tuple of the low (wrapping) bits and the high (carry) bit of the result, in that order.
    ///
    /// If `carry` is false, then this method is equivalent to [`overflowing_add`](Self::overflowing_add).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(U1024::ONE.carrying_add(U1024::ONE, true), (3.as_(), false));
    /// assert_eq!(U1024::MAX.carrying_add(U1024::ONE, false), (0.as_(), true));
    /// assert_eq!(U1024::MAX.carrying_add(U1024::MAX, true), (U1024::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_add(rhs);
        if carry {
            let (s2, o2) = s1.overflowing_add(Self::ONE);
            (s2, o1 ^ o2)
        } else {
            (s1, o1)
        }
    }

    /// Computes `self - rhs - borrow`, and returns a tuple of the low (wrapping) bits of the result and a boolean indicating whether an arithmetic borrow (overflow) occurred.
    ///
    /// If `borrow` is false, then this method is equivalent to [`overflowing_sub`](Self::overflowing_sub).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(2.as_::<U1024>().borrowing_sub(U1024::ONE, true), (0.as_(), false));
    /// assert_eq!(U1024::ZERO.borrowing_sub(U1024::ONE, false), (U1024::MAX, true));
    /// assert_eq!(U1024::MAX.borrowing_sub(U1024::MAX, true), (U1024::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_sub(rhs);
        if borrow {
            let (s2, o2) = s1.overflowing_sub(Self::ONE);
            (s2, o1 ^ o2)
        } else {
            (s1, o1)
        }
    }

    /// Computes `self * rhs`, and returns a tuple of the low (wrapping) bits and high (overflow) bits of the result, in that order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    ///
    /// assert_eq!(7.as_::<U256>().widening_mul(3.as_()), (21.as_(), 0.as_()));
    /// assert_eq!(2.as_::<U256>().pow(255).widening_mul(2.as_().pow(100)), (0.as_(), 2.as_().pow(99)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn widening_mul(self, rhs: Self) -> (Uint<N>, Self) {
        if S {
            let (u_lo, u_hi) = self
                .unsigned_abs_internal()
                .widening_mul(rhs.unsigned_abs_internal());
            return if self.is_negative_internal() == rhs.is_negative_internal() {
                (u_lo, u_hi.force_sign())
            } else {
                // treat as a "super bigint" - a bigint with two "digits", where the digits are themselves bigints (Uints). then apply the same logic that a wrapping_neg for signed ints is the same as wrapping_neg for unsigned ints. effectively, we are computing a wrapping_neg of the "super bigint"
                let (u_lo, overflow) = u_lo.overflowing_neg();
                let hi = if overflow {
                    u_hi.force_sign().wrapping_neg()
                } else {
                    u_hi.force_sign().not() // this is wrapping_neg - 1
                };
                (u_lo, hi)
            };
        }
        // low, high in that order
        #[repr(C)] // so that the arrays are stored in contiguous memory
        struct DoubleInt<const M: usize>(Uint<M>, Uint<M>);

        impl<const M: usize> DoubleInt<M> {
            const ZERO: Self = Self(Uint::ZERO, Uint::ZERO);
            const U128_DIGITS: usize = (2 * M).div_ceil(16);

            #[inline]
            const fn as_wide_digits(&self) -> WideDigits<M, true, false> {
                WideDigits::new(&self.0.bytes)
            }

            #[inline]
            const unsafe fn get_u128_digit(&self, index: usize) -> u128 {
                let mut bytes = [0; 16];
                unsafe {
                    self.0
                        .bytes
                        .as_ptr()
                        .add(index * 16)
                        .copy_to_nonoverlapping(bytes.as_mut_ptr(), 16);
                }
                u128::from_le_bytes(bytes)
            }

            #[inline]
            pub const unsafe fn get_u128_digit_at_offset(&self, offset: usize) -> u128 {
                let mut bytes = [0; 16];
                let c = 2 * M - offset;
                let count = if c > 16 { 16 } else { c };
                unsafe {
                    self.0
                        .bytes
                        .as_ptr()
                        .add(offset)
                        .copy_to_nonoverlapping(bytes.as_mut_ptr(), count);
                }
                u128::from_le_bytes(bytes)
            }

            #[inline]
            pub const unsafe fn get_u128_digit_with_correct_count(&self, index: usize) -> u128 {
                unsafe { self.get_u128_digit_at_offset(index * 16) }
            }

            #[inline]
            const unsafe fn set_u128_digit(&mut self, index: usize, value: u128) {
                let bytes = value.to_le_bytes();
                unsafe {
                    self.0
                        .bytes
                        .as_mut_ptr()
                        .add(index * 16)
                        .copy_from_nonoverlapping(bytes.as_ptr(), 16);
                }
            }

            #[inline]
            const unsafe fn set_u128_digit_at_offset(&mut self, offset: usize, value: u128) {
                let out_bytes = value.to_le_bytes();
                let c = 2 * M - offset;
                let count = if c > 16 { 16 } else { c };
                unsafe {
                    out_bytes
                        .as_ptr()
                        .copy_to_nonoverlapping(self.0.bytes.as_mut_ptr().add(offset), count);
                }
            }

            #[inline]
            const unsafe fn set_u128_digit_with_correct_count(
                &mut self,
                index: usize,
                value: u128,
            ) {
                unsafe {
                    self.set_u128_digit_at_offset(index * 16, value);
                }
            }
        }

        let mut out = DoubleInt::<N>::ZERO;
        let mut carry: u128;

        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                carry = 0;
                let self_digit_i = self.as_wide_digits().get(i);

                let mut j = 0;
                while j < Self::FULL_U128_DIGITS {
                    let index = i + j;
                    let (prod, c) = digit::carrying_mul_u128(
                        self_digit_i,
                        rhs.as_wide_digits().get(j),
                        carry,
                        out.as_wide_digits().get(index),
                    );
                    out.set_u128_digit(index, prod);
                    carry = c;

                    j += 1;
                }
                if Self::U128_DIGIT_REMAINDER == 0 {
                    out.set_u128_digit(i + Self::U128_DIGITS, carry);
                } else {
                    let index = i + Self::FULL_U128_DIGITS;
                    let (prod, c) = digit::carrying_mul_u128(
                        self_digit_i,
                        rhs.as_wide_digits().get(j),
                        carry,
                        out.get_u128_digit_with_correct_count(index),
                    );
                    out.set_u128_digit_with_correct_count(index, prod);
                    carry = c;
                    if i + Self::U128_DIGITS < DoubleInt::<N>::U128_DIGITS {
                        out.set_u128_digit_with_correct_count(i + Self::U128_DIGITS, carry);
                    } else {
                        // carry must be equal to 0, as the last incomplete u128 digits are narrow enough to not overflow
                        debug_assert!(carry == 0);
                    }
                }

                i += 1;
            }
        }

        (out.0, out.1.force_sign())
    }

    /// Computes `self * rhs + carry`, and returns a tuple of the low (wrapping) bits and high (overflow) bits of the result, in that order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    ///
    /// assert_eq!(7.as_::<U2048>().carrying_mul(3.as_(), 5.as_()), (26.as_(), 0.as_()));
    /// assert_eq!(U2048::MAX.carrying_mul(U2048::MAX, U2048::MAX), (0.as_(), U2048::MAX));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Uint<N>, Self) {
        // if S {

        // we pretend that we have a "super bigint" - a big int with two "digits", where the digits are themselves big ints (Uints). then apply the same logic that an wrapping_add for signed ints is the same as wrapping_add for unsigned ints
        // effectively, we are computing a wrapping_add of the "super bigint"
        let (lo, hi) = self.widening_mul(rhs);
        let (lo, overflow) = lo.overflowing_add(carry.force_sign());

        // we interpret the carry as a "super bigint" by extending it to twice its width: since it is negative, it is signed extended with all ones
        // so we want to perform a wrapping_add of hi + (-1). but this is clearly equivalent to a wrapping_sub of hi - 1
        // however, we delay this operation as it may cancel with the overflow increment
        // let hi = if carry.is_negative_internal() {
        //     hi.wrapping_sub(Self::ONE)
        // } else {
        //     hi
        // };
        // return if overflow {
        //     (lo, hi.wrapping_add(Self::ONE))
        // } else {
        //     (lo, hi)
        // };

        match (carry.is_negative_internal(), overflow) {
            (true, true) => (lo, hi),                          // hi - 1 + 1
            (true, false) => (lo, hi.wrapping_sub(Self::ONE)), // hi - 1
            (false, true) => (lo, hi.wrapping_add(Self::ONE)), // hi + 1
            (false, false) => (lo, hi),                        // hi
        }
        // }

        // let (low, high) = self.widening_mul(rhs);
        // let (low, overflow) = low.overflowing_add(carry.force_sign());
        // if overflow {
        //     (low, high.wrapping_add(Self::ONE))
        // } else {
        //     (low, high)
        // }
    }

    /// Computes `self * rhs + carry + add`, and returns a tuple of the low (wrapping) bits and high (overflow) bits of the result, in that order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    ///
    /// assert_eq!(7.as_::<U2048>().carrying_mul_add(3.as_(), 5.as_(), 12.as_()), (38.as_(), 0.as_()));
    /// assert_eq!(U2048::MAX.carrying_mul_add(U2048::MAX, U2048::MAX, U2048::MAX), (U2048::MAX, U2048::MAX));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Uint<N>, Self) {
        // if S {

            // similarly to carrying_mul
            let (lo, hi) = self.carrying_mul(rhs, carry);
            // let extension = if add.is_negative() {
            //     Self::NEG_ONE // all ones
            // } else {
            //     Self::ZERO // all zeros
            // };
            let (lo, overflow) = lo.overflowing_add(add.force_sign());
            // let hi = hi.wrapping_add(extension);
            // return if overflow {
            //     (lo, hi.wrapping_add(Self::ONE))
            // } else {
            //     (lo, hi)
            // };

            match (add.is_negative_internal(), overflow) {
                (true, true) => (lo, hi), // hi - 1 + 1
                (true, false) => (lo, hi.wrapping_sub(Self::ONE)), // hi - 1
                (false, true) => (lo, hi.wrapping_add(Self::ONE)), // hi + 1
                (false, false) => (lo, hi), // hi
            }
        // }

        // let (low, high) = self.carrying_mul(rhs, carry);
        // let (low, overflow) = low.overflowing_add(add);
        // if overflow {
        //     (low, high.wrapping_add(Self::ONE))
        // } else {
        //     (low, high)
        // }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest>::carrying_add(a: stest, b: stest, carry: bool),
            cases: [
                (<stest>::MAX, 1u8, true),
                (<stest>::MAX, 1u8, false)
            ]
        }
        test_bignum! {
            function: <stest>::borrowing_sub(a: stest, b: stest, borrow: bool),
            cases: [
                (<stest>::MIN, 1u8, false),
                (<stest>::MIN, 1u8, true)
            ]
        }
        test_bignum! {
            function: <stest>::widening_mul(a: stest, b: stest),
            cases: [
                (<stest>::MAX, 2u8)
            ]
        }
        test_bignum! {
            function: <stest>::carrying_mul(a: stest, b: stest, c: stest)
        }
        test_bignum! {
            function: <stest>::carrying_mul_add(a: stest, b: stest, c: stest, d: stest)
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::widening_mul(a: utest, b: utest)
    }
}
