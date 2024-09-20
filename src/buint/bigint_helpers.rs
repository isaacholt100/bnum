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

        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                // use crate::test::{test_bignum, types::*};
                use crate::test::types::big_types::$Digit::*;

                type U64 = crate::$BUint::<{64 / $Digit::BITS as usize}>;

                crate::int::bigint_helpers::tests!(utest);

                test_bignum! {
                    function: <u64>::widening_mul(a: u64, b: u64),
                    cases: [
                        (u64::MAX, u64::MAX)
                    ]
                }

                test_bignum! {
                    function: <u64>::carrying_mul(a: u64, b: u64, c: u64),
                    cases: [
                        (u64::MAX, u64::MAX, u64::MAX),
                        (u64::MAX, u64::MAX, 1u64)
                    ]
                }
            }
        }
    };
}

crate::macro_impl!(bigint_helpers);
