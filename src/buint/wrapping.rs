use super::{BUint, ExpType};
use crate::{BInt, doc, errors};
use crate::errors::option_expect;
use crate::int::wrapping::wrapping_method;

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> BUint<N> {
	wrapping_method!(wrapping_add, overflowing_add, Self);

	wrapping_method!(wrapping_add_signed, overflowing_add_signed, BInt<N>);

	wrapping_method!(wrapping_sub, overflowing_sub, Self);

	wrapping_method!(wrapping_mul, overflowing_mul, Self);

    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        option_expect!(self.checked_div(rhs), errors::err_msg!("attempt to divide by zero"))
    }

    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        option_expect!(self.checked_rem(rhs), errors::err_msg!("attempt to calculate the remainder with a divisor of zero"))
    }

    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }

	wrapping_method!(wrapping_neg, overflowing_neg);

	wrapping_method!(wrapping_shl, overflowing_shl, ExpType);

	wrapping_method!(wrapping_shr, overflowing_shr, ExpType);

	pub const fn wrapping_pow(mut self, mut pow: ExpType) -> Self {
		// https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method
		if pow == 0 {
			return Self::ONE;
		}
		let mut y = Self::ONE;
		while pow > 1 {
			if pow & 1 == 1 {
				y = self.wrapping_mul(y);
			}
			self = self.wrapping_mul(self);
			pow >>= 1;
		}
		self.wrapping_mul(y)
	}
}

#[cfg(test)]
mod tests {
	use crate::test::{test_bignum, types::utest};

    test_bignum! {
		function: <utest>::wrapping_add(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::wrapping_sub(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::wrapping_mul(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::wrapping_div(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
		function: <utest>::wrapping_div_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
		function: <utest>::wrapping_rem(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
		function: <utest>::wrapping_rem_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
		function: <utest>::wrapping_neg(a: utest)
    }
    test_bignum! {
		function: <utest>::wrapping_shl(a: utest, b: u16)
    }
    test_bignum! {
		function: <utest>::wrapping_shr(a: utest, b: u16)
    }
    test_bignum! {
		function: <utest>::wrapping_pow(a: utest, b: u16)
    }
}