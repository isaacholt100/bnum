macro_rules! impls {
    ($sign: ident) => {
        #[doc = doc::bigint_helpers::carrying_add!($sign)]
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

        #[doc = doc::bigint_helpers::borrowing_sub!($sign)]
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
    };
}

pub(crate) use impls;

#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        use crate::test::{test_bignum, types::*};

        test_bignum! {
            function: <$int>::carrying_add(a: $int, b: $int, carry: bool),
            cases: [
                (<$int>::MAX, 1u8, true),
                (<$int>::MAX, 1u8, false)
            ]
        }
        test_bignum! {
            function: <$int>::borrowing_sub(a: $int, b: $int, borrow: bool),
            cases: [
                (<$int>::MIN, 1u8, false),
                (<$int>::MIN, 1u8, true)
            ]
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
