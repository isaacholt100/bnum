use crate::{Int, Integer, Uint};

macro_rules! impl_desc {
    () => {
        "Strict arithmetic methods which act on `self`: `self.strict_...`. Each method will always panic if overflow or division by zero occurs (i.e. when the checked equivalent would return `None`), regardless of whether overflow checks are enabled."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize> Integer<S, N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs)
            .expect(crate::errors::err_msg!("attempt to multiply with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_div(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::div_by_zero_message!()));
        }
        self.div_rem_unchecked(rhs).0
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_div_euclid(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::div_by_zero_message!()));
        }
        self.div_rem_euclid_unchecked(rhs).0
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_rem(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to calculate the remainder with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::rem_by_zero_message!()));
        }
        self.div_rem_unchecked(rhs).1
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to calculate the remainder with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::rem_by_zero_message!()));
        }
        self.div_rem_euclid_unchecked(rhs).1
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_neg(self) -> Self {
        self.checked_neg()
            .expect(crate::errors::err_msg!("attempt to negate with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_shl(self, rhs: crate::ExpType) -> Self {
        self.checked_shl(rhs).expect(crate::errors::err_msg!(
            "attempt to shift left with overflow"
        ))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_shr(self, rhs: crate::ExpType) -> Self {
        self.checked_shr(rhs).expect(crate::errors::err_msg!(
            "attempt to shift right with overflow"
        ))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_pow(self, exp: crate::ExpType) -> Self {
        self.checked_pow(exp).expect(crate::errors::err_msg!(
            "attempt to calculate power with overflow"
        ))
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize> Uint<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_signed(self, rhs: Int<N>) -> Self {
        self.checked_add_signed(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub_signed(self, rhs: Int<N>) -> Self {
        self.checked_sub_signed(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_abs(self) -> Self {
        self.checked_abs()
            .expect(crate::errors::err_msg!("attempt to negate with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_unsigned(self, rhs: Uint<N>) -> Self {
        self.checked_add_unsigned(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub_unsigned(self, rhs: Uint<N>) -> Self {
        self.checked_sub_unsigned(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }
}

#[cfg(all(test, feature = "nightly"))] // since strict_overflow_ops are not stable yet
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::strict_add_signed(a: utest, b: itest),
            skip: a.checked_add_signed(b).is_none()
        }
        test_bignum! {
            function: <utest>::strict_sub_signed(a: utest, b: itest),
            skip: a.checked_sub_signed(b).is_none()
        }
    }
    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::strict_abs(a: itest),
            skip: a.checked_abs().is_none()
        }
        test_bignum! {
            function: <itest>::strict_add_unsigned(a: itest, b: utest),
            skip: a.checked_add_unsigned(b).is_none()
        }
        test_bignum! {
            function: <itest>::strict_sub_unsigned(a: itest, b: utest),
            skip: a.checked_sub_unsigned(b).is_none()
        }
    }
    crate::test::test_all! {
        testing integers;
        
        test_bignum! {
            function: <stest>::strict_add(a: stest, b: stest),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_sub(a: stest, b: stest),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_mul(a: stest, b: stest),
            skip: a.checked_mul(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_div(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_div_euclid(a: stest, b: stest),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_rem(a: stest, b: stest),
            skip: a.checked_rem(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_rem_euclid(a: stest, b: stest),
            skip: a.checked_rem_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_neg(a: stest),
            skip: a.checked_neg().is_none()
        }
        test_bignum! {
            function: <stest>::strict_shl(a: stest, b: u8),
            skip: a.checked_shl(b as u32).is_none()
        }
        test_bignum! {
            function: <stest>::strict_shr(a: stest, b: u8),
            skip: a.checked_shr(b as u32).is_none()
        }
        test_bignum! {
            function: <stest>::strict_pow(a: stest, b: u8),
            skip: a.checked_pow(b as u32).is_none()
        }
    }
}

use crate::doc;
