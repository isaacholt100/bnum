use super::{BUint, ExpType};
use crate::{BInt, doc, error};
use crate::macros::option_expect;
use crate::int::wrapping::wrapping_method;

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> BUint<N> {
	wrapping_method!(wrapping_add, overflowing_add, Self);

	wrapping_method!(wrapping_add_signed, overflowing_add_signed, BInt<N>);

	wrapping_method!(wrapping_sub, overflowing_sub, Self);

	wrapping_method!(wrapping_mul, overflowing_mul, Self);

    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        option_expect!(self.checked_div(rhs), error::err_msg!("attempt to divide by zero"))
    }

    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        option_expect!(self.checked_rem(rhs), error::err_msg!("attempt to calculate the remainder with a divisor of zero"))
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
	use crate::test::test_bignum;

    test_bignum! {
		function: <u128>::wrapping_add(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::wrapping_sub(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::wrapping_mul(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::wrapping_div(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (908940869048689045869048680405869009u128, 9347539457839475893475959u128),
            (9476485690845684567394573544345543u128, 349587458697u128)
        ]
    }
    test_bignum! {
		function: <u128>::wrapping_div_euclid(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (495769576475698737689374598674899857u128, 856894756457986456u128),
            (13495893475u128, 349583453457458697u128)
        ]
    }
    test_bignum! {
		function: <u128>::wrapping_rem(a: u128, b: u128),
        skip: b == 0
    }
    test_bignum! {
		function: <u128>::wrapping_rem_euclid(a: u128, b: u128),
        skip: b == 0
    }
    test_bignum! {
		function: <u128>::wrapping_neg(a: u128)
    }
    test_bignum! {
		function: <u128>::wrapping_shl(a: u128, b: u16)
    }
    test_bignum! {
		function: <u128>::wrapping_shr(a: u128, b: u16)
    }
    test_bignum! {
		function: <u128>::wrapping_pow(a: u128, b: u16)
    }
}