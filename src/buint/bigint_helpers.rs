use crate::digit;
use crate::doc;

macro_rules! bigint_helpers {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::bigint_helpers::impl_desc!()]
        impl<const N: usize> $BUint<N> {
            crate::int::bigint_helpers::impls!(U);

            #[doc = doc::bigint_helpers::widening_mul!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
                let mut low = Self::ZERO;
                let mut high = Self::ZERO;
                let mut carry: $Digit;

                let mut i = 0;
                while i < N {
                    carry = 0;
                    let mut j = 0;
                    while j < N - i {
                        let index = i + j;
                        let d = low.digits[index];
                        let (new_digit, new_carry) =
                            digit::$Digit::carrying_mul(self.digits[i], rhs.digits[j], carry, d);
                        carry = new_carry;
                        low.digits[index] = new_digit;
                        j += 1;
                    }
                    while j < N {
                        let index = i + j - N;
                        let d = high.digits[index];
                        let (new_digit, new_carry) =
                            digit::$Digit::carrying_mul(self.digits[i], rhs.digits[j], carry, d);
                        carry = new_carry;
                        high.digits[index] = new_digit;
                        j += 1;
                    }
                    high.digits[i] = carry;
                    i += 1;
                }

                (low, high)
            }

            #[doc = doc::bigint_helpers::carrying_mul!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
                let (low, high) = self.widening_mul(rhs);
                let (low, overflow) = low.overflowing_add(carry);
                if overflow {
                    (low, high.wrapping_add(Self::ONE))
                } else {
                    (low, high)
                }
            }
        }
    };
}

#[cfg(all(test, feature = "nightly"))] // as bigint_helpers not stabilised yet
crate::test::all_digit_tests! {
    crate::int::bigint_helpers::tests!(utest);

    #[cfg(test_int_bits = "64")]
    test_bignum! {
        function: <utest>::widening_mul(a: utest, b: utest),
        cases: [
            (utest::MAX, utest::MAX)
        ]
    }

    #[cfg(test_int_bits = "64")]
    test_bignum! {
        function: <utest>::carrying_mul(a: utest, b: utest, c: utest),
        cases: [
            (utest::MAX, utest::MAX, utest::MAX),
            (utest::MAX, utest::MAX, 1 as utest)
        ]
    }
}

crate::macro_impl!(bigint_helpers);
