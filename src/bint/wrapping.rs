use super::BInt;
use crate::{ExpType, BUint, doc};
use crate::int::wrapping::wrapping_method;

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_add(rhs.bits))
    }

	wrapping_method!(wrapping_add_unsigned, overflowing_add_unsigned, BUint<N>);

    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_sub(rhs.bits))
    }

	wrapping_method!(wrapping_sub_unsigned, overflowing_sub_unsigned, BUint<N>);

    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_mul(rhs.bits))
    }

	wrapping_method!(wrapping_div, overflowing_div, Self);

	wrapping_method!(wrapping_div_euclid, overflowing_div_euclid, Self);

	wrapping_method!(wrapping_rem, overflowing_rem, Self);

	wrapping_method!(wrapping_rem_euclid, overflowing_rem_euclid, Self);

	wrapping_method!(wrapping_neg, overflowing_neg);

	wrapping_method!(wrapping_shl, overflowing_shl, ExpType);

	wrapping_method!(wrapping_shr, overflowing_shr, ExpType);

	#[inline]
	pub const fn wrapping_pow(self, pow: ExpType) -> Self {
		// as wrapping_mul for signed and unsigned is the same
		Self::from_bits(self.bits.wrapping_pow(pow))
	}

	wrapping_method!(wrapping_abs, overflowing_abs);
}

#[cfg(test)]
mod tests {
	use crate::test::{test_bignum, types::itest};
	
    test_bignum! {
        function: <itest>::wrapping_add(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::wrapping_sub(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::wrapping_mul(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::wrapping_div(a: itest, b: itest),
        skip: b == 0
    }
    test_bignum! {
        function: <itest>::wrapping_div_euclid(a: itest, b: itest),
        skip: b == 0
    }
    test_bignum! {
        function: <itest>::wrapping_rem(a: itest, b: itest),
        skip: b == 0,
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::wrapping_rem_euclid(a: itest, b: itest),
        skip: b == 0
    }
    test_bignum! {
        function: <itest>::wrapping_neg(a: itest),
        cases: [
            (itest::MIN)
        ]
    }
    test_bignum! {
        function: <itest>::wrapping_shl(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::wrapping_shr(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::wrapping_pow(a: itest, b: u16)
    }
}