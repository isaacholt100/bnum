macro_rules! impls {
    ($sign: ident) => {
        #[doc = doc::strict::strict_add!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_add(self, rhs: Self) -> Self {
            crate::errors::option_expect!(
                self.checked_add(rhs),
                crate::errors::err_msg!("attempt to add with overflow")
            )
        }

        #[doc = doc::strict::strict_sub!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_sub(self, rhs: Self) -> Self {
            crate::errors::option_expect!(
                self.checked_sub(rhs),
                crate::errors::err_msg!("attempt to subtract with overflow")
            )
        }

        #[doc = doc::strict::strict_mul!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_mul(self, rhs: Self) -> Self {
            crate::errors::option_expect!(
                self.checked_mul(rhs),
                crate::errors::err_msg!("attempt to multiply with overflow")
            )
        }

        #[doc = doc::strict::strict_div!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_div(self, rhs: Self) -> Self {
            self.div(rhs)
        }

        #[doc = doc::strict::strict_div_euclid!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_div_euclid(self, rhs: Self) -> Self {
            self.div_euclid(rhs)
        }

        #[doc = doc::strict::strict_rem!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_rem(self, rhs: Self) -> Self {
            self.rem(rhs)
        }

        #[doc = doc::strict::strict_rem_euclid!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
            self.rem_euclid(rhs)
        }

        #[doc = doc::strict::strict_neg!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_neg(self) -> Self {
            crate::errors::option_expect!(
                self.checked_neg(),
                crate::errors::err_msg!("attempt to negate with overflow")
            )
        }

        #[doc = doc::strict::strict_shl!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_shl(self, rhs: crate::ExpType) -> Self {
            crate::errors::option_expect!(
                self.checked_shl(rhs),
                crate::errors::err_msg!("attempt to shift left with overflow")
            )
        }

        #[doc = doc::strict::strict_shr!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_shr(self, rhs: crate::ExpType) -> Self {
            crate::errors::option_expect!(
                self.checked_shr(rhs),
                crate::errors::err_msg!("attempt to shift right with overflow")
            )
        }

        #[doc = doc::strict::strict_pow!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const fn strict_pow(self, exp: crate::ExpType) -> Self {
            crate::errors::option_expect!(
                self.checked_pow(exp),
                crate::errors::err_msg!("attempt to calculate power with overflow")
            )
        }
    };
}

pub(crate) use impls;

#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        use crate::test::{test_bignum, types::*};

        test_bignum! {
            function: <$int>::strict_add(a: $int, b: $int),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_sub(a: $int, b: $int),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_mul(a: $int, b: $int),
            skip: a.checked_mul(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_div(a: $int, b: $int),
            skip: a.checked_div(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_div_euclid(a: $int, b: $int),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_rem(a: $int, b: $int),
            skip: a.checked_rem(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_rem_euclid(a: $int, b: $int),
            skip: a.checked_rem_euclid(b).is_none()
        }
        test_bignum! {
            function: <$int>::strict_neg(a: $int),
            skip: a.checked_neg().is_none()
        }
        test_bignum! {
            function: <$int>::strict_shl(a: $int, b: u8),
            skip: a.checked_shl(b as u32).is_none()
        }
        test_bignum! {
            function: <$int>::strict_shr(a: $int, b: u8),
            skip: a.checked_shr(b as u32).is_none()
        }
        test_bignum! {
            function: <$int>::strict_pow(a: $int, b: u8),
            skip: a.checked_pow(b as u32).is_none()
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
