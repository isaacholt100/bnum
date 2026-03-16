use crate::doc;
use crate::{Integer, Uint};

/// Bigint helper methods: common functions used to implement big integer arithmetic.
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
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
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(1U1024).carrying_add(n!(1), true), (n!(3), false));
    /// assert_eq!(U1024::MAX.carrying_add(n!(1), false), (n!(0), true));
    /// assert_eq!(U1024::MAX.carrying_add(U1024::MAX, true), (U1024::MAX, true));
    ///
    /// assert_eq!(n!(1I1024).carrying_add(n!(1), true), (n!(3), false));
    /// assert_eq!(I1024::MAX.carrying_add(n!(0), true), (I1024::MIN, true));
    /// assert_eq!(I1024::MAX.carrying_add(I1024::MAX, true), (n!(-1), true));
    /// assert_eq!(I1024::MIN.carrying_add(I1024::MIN, true), (n!(1), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_add(rhs);
        if carry {
            let (s2, o2) = s1.overflowing_add(Self::ONE);
            (s2, o1 != o2)
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
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(2U512).borrowing_sub(n!(1), true), (n!(0), false));
    /// assert_eq!(U512::MIN.borrowing_sub(n!(1), false), (U512::MAX, true));
    /// assert_eq!(U512::MAX.borrowing_sub(U512::MAX, true), (U512::MAX, true));
    ///
    /// assert_eq!(n!(1I512).borrowing_sub(n!(1I512), true), (n!(-1I512), false));
    /// assert_eq!(I512::MIN.borrowing_sub(n!(1), false), (I512::MAX, true));
    /// assert_eq!(n!(0).borrowing_sub(I512::MIN, true), (I512::MAX, false));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_sub(rhs);
        if borrow {
            let (s2, o2) = s1.overflowing_sub(Self::ONE);
            (s2, o1 != o2)
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
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(n!(7U256).widening_mul(n!(3)), (n!(21), n!(0)));
    /// assert_eq!(n!(2U256).pow(255).widening_mul(n!(2U256).pow(100)), (n!(0), n!(2U256).pow(99)));
    ///
    /// assert_eq!(n!(-5I256).widening_mul(n!(8)), (n!(-40).cast_unsigned(), n!(-1)));
    /// assert_eq!(I256::MIN.widening_mul(n!(2)), (n!(0), n!(-1)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn widening_mul(self, rhs: Self) -> (Uint<N, B, OM>, Self) {
        if S {
            let (u_lo, u_hi) = self
                .unsigned_abs_internal()
                .widening_mul(rhs.unsigned_abs_internal());
            return if self.is_negative_internal() == rhs.is_negative_internal() {
                (u_lo, u_hi.force_sign())
            } else {
                // treat as a "super bigint" - a bigint with two "digits", where the digits are themselves bigints (Uints). then apply the same logic that a wrapping_neg for signed ints is the same as wrapping_neg for unsigned ints. effectively, we are computing a wrapping_neg of the "super bigint"
                // wrapping_neg = wrapping_not + 1
                let (u_lo, overflow) = u_lo.not().overflowing_add(Uint::ONE); // not overflowing_neg as we want u_lo zero to cause overflow
                let u_hi = if overflow {
                    u_hi.wrapping_neg()
                } else {
                    u_hi.not() // this is wrapping_neg - 1
                };
                (u_lo, u_hi.force_sign())
            };
        }
        let a = self.to_digits::<u128>().grow::<2, 0>();
        let b = rhs.to_digits::<u128>().grow::<2, 0>();
        let m = a.long_mul::<true>(b).0;
        let (mut lo, mut hi): (Uint<N>, Uint<N>) = unsafe { core::mem::transmute_copy(&m) };

        if Self::LAST_BYTE_PAD_BITS != 0 {
            // if NUM_PAD_BITS = n, want to shift hi by n bits and move the most significant n bits of lo to least significant n bits of hi
            hi = hi.shl(Self::LAST_BYTE_PAD_BITS);
            let lo_msb = lo
                .shr(Uint::<N>::BITS - Self::LAST_BYTE_PAD_BITS); // shift by this amount as we are effectively working with a Uint<N, 8 * N>
            hi = hi.bitor(lo_msb);
            lo.set_sign_bits();
        }
        (lo.force(), hi.force())
    }

    /// Computes `self * rhs + carry`, and returns a tuple of the low (wrapping) bits and high (overflow) bits of the result, in that order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(7U2048).carrying_mul(n!(3), n!(5)), (n!(26), n!(0)));
    /// assert_eq!(U2048::MAX.carrying_mul(U2048::MAX, U2048::MAX), (n!(0), U2048::MAX));
    ///
    /// assert_eq!(n!(-5I2048).carrying_mul(n!(8), n!(-6)), (n!(-46).cast_unsigned(), n!(-1)));
    /// assert_eq!(I2048::MIN.carrying_mul(n!(2), n!(3)), (n!(3), n!(-1)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Uint<N, B, OM>, Self) {
        // we pretend that we have a "super bigint" - a big int with two "digits", where the digits are themselves big ints (Uints). then apply the same logic that an wrapping_add for signed ints is the same as wrapping_add for unsigned ints
        // effectively, we are computing a wrapping_add of the "super bigint"
        let (lo, hi) = self.widening_mul(rhs);
        let (lo, overflow) = lo.overflowing_add(carry.force_sign());

        // we interpret the carry as a "super bigint" by extending it to twice its width: since it is negative, it is signed extended with all ones
        // so we want to perform a wrapping_add of hi + (-1). but this is clearly equivalent to a wrapping_sub of hi - 1
        // however, we delay this operation as it may cancel with the overflow increment

        match (carry.is_negative_internal(), overflow) {
            (true, true) => (lo, hi),                          // hi - 1 + 1
            (true, false) => (lo, hi.wrapping_sub(Self::ONE)), // hi - 1
            (false, true) => (lo, hi.wrapping_add(Self::ONE)), // hi + 1
            (false, false) => (lo, hi),                        // hi
        }
    }

    /// Computes `self * rhs + carry + add`, and returns a tuple of the low (wrapping) bits and high (overflow) bits of the result, in that order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(7U2048).carrying_mul_add(n!(3), n!(5), n!(12)), (n!(38), n!(0)));
    /// assert_eq!(U2048::MAX.carrying_mul_add(U2048::MAX, U2048::MAX, U2048::MAX), (U2048::MAX, U2048::MAX));
    ///
    /// assert_eq!(n!(-5I2048).carrying_mul_add(n!(8), n!(-6), n!(-11)), (n!(-57).cast_unsigned(), n!(-1)));
    /// assert_eq!(I2048::MIN.carrying_mul_add(n!(2), n!(3), n!(-2)), (n!(1), n!(-1)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul_add(
        self,
        rhs: Self,
        carry: Self,
        add: Self,
    ) -> (Uint<N, B, OM>, Self) {
        // similarly to carrying_mul
        let (lo, hi) = self.carrying_mul(rhs, carry);
        let (lo, overflow) = lo.overflowing_add(add.force_sign());

        match (add.is_negative_internal(), overflow) {
            (true, true) => (lo, hi),                          // hi - 1 + 1
            (true, false) => (lo, hi.wrapping_sub(Self::ONE)), // hi - 1
            (false, true) => (lo, hi.wrapping_add(Self::ONE)), // hi + 1
            (false, false) => (lo, hi),                        // hi
        }
    }
}

#[cfg(all(test, nightly))] // since bigint_helpers not stable for signed integers yet
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

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::widening_mul(a: utest, b: utest)
    }
}
