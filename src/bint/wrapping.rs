use super::BInt;
use crate::{ExpType, BUint, doc};

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[doc=doc::wrapping::wrapping_add!(U)]
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_add(rhs.bits))
    }

	#[doc=doc::wrapping::wrapping_add_unsigned!(U)]
	#[inline]
	pub const fn wrapping_add_unsigned(self, rhs: BUint<N>) -> Self {
		self.overflowing_add_unsigned(rhs).0
	}

    #[doc=doc::wrapping::wrapping_sub!(U)]
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_sub(rhs.bits))
    }

	#[doc=doc::wrapping::wrapping_sub_unsigned!(U)]
	#[inline]
	pub const fn wrapping_sub_unsigned(self, rhs: BUint<N>) -> Self {
		self.overflowing_sub_unsigned(rhs).0
	}

    #[doc=doc::wrapping::wrapping_mul!(U)]
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_mul(rhs.bits))
    }

	#[doc=doc::wrapping::wrapping_div!(U)]
	#[inline]
	pub const fn wrapping_div(self, rhs: Self) -> Self {
		self.overflowing_div(rhs).0
	}

	#[doc=doc::wrapping::wrapping_div_euclid!(U)]
	#[inline]
	pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
		self.overflowing_div_euclid(rhs).0
	}

	#[doc=doc::wrapping::wrapping_rem!(U)]
	#[inline]
	pub const fn wrapping_rem(self, rhs: Self) -> Self {
		self.overflowing_rem(rhs).0
	}

	#[doc=doc::wrapping::wrapping_rem_euclid!(U)]
	#[inline]
	pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
		self.overflowing_rem_euclid(rhs).0
	}

    #[doc=doc::wrapping::wrapping_neg!(U)]
	#[inline]
	pub const fn wrapping_neg(self) -> Self {
		self.overflowing_neg().0
	}

    #[doc=doc::wrapping::wrapping_shl!(U)]
	#[inline]
	pub const fn wrapping_shl(self, rhs: ExpType) -> Self {
		self.overflowing_shl(rhs).0
	}

    #[doc=doc::wrapping::wrapping_shr!(U)]
	#[inline]
	pub const fn wrapping_shr(self, rhs: ExpType) -> Self {
		self.overflowing_shr(rhs).0
	}

    #[doc=doc::wrapping::wrapping_pow!(U)]
	#[inline]
	pub const fn wrapping_pow(self, pow: ExpType) -> Self {
		// as wrapping_mul for signed and unsigned is the same
		Self::from_bits(self.bits.wrapping_pow(pow))
	}

    #[doc=doc::wrapping::wrapping_abs!(U)]
	#[inline]
	pub const fn wrapping_abs(self) -> Self {
		self.overflowing_abs().0
	}
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
            (itest::MIN, -1i8),
			(185892231884832768i64 as itest, 92946115942416385i64 as itest)
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