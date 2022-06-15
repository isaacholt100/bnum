use super::{BUint, ExpType};
use crate::{BInt, doc, error};
use crate::macros::{option_expect, wrapping_pow};

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    #[inline]
    pub const fn wrapping_add_signed(self, rhs: BInt<N>) -> Self {
        self.overflowing_add_signed(rhs).0
    }

    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

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

    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    #[inline]
    pub const fn wrapping_shl(self, rhs: ExpType) -> Self {
        self.overflowing_shl(rhs).0
    }

    #[inline]
    pub const fn wrapping_shr(self, rhs: ExpType) -> Self {
        self.overflowing_shr(rhs).0
    }
    
    wrapping_pow!();
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