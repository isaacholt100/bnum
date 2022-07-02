use super::BInt;
use crate::{ExpType, BUint};
use crate::doc;
use crate::int::checked::tuple_to_option;

macro_rules! checked_log {
    ($method: ident $(, $base: ident: $ty: ty)?) => {
        #[inline]
        pub const fn $method(self $(, $base: $ty)?) -> Option<ExpType> {
            if self.is_negative() {
                None
            } else {
                self.bits.$method($($base)?)
            }
        }
    }
}

#[doc=doc::checked::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[inline]
    #[doc=doc::checked_add!(I256)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }

    #[inline]
    pub const fn checked_add_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }

    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div(rhs))
        }
    }

    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div_euclid(rhs))
        }
    }

    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem(rhs))
        }
    }

    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem_euclid(rhs))
        }
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        tuple_to_option(self.overflowing_neg())
    }

    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shl(rhs))
    }

    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }

    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }

	#[inline]
	pub const fn checked_pow(self, pow: ExpType) -> Option<Self> {
		match self.unsigned_abs().checked_pow(pow) {
			Some(u) => {
				let out = Self::from_bits(u);
				let neg = self.is_negative();
				if !neg || pow & 1 == 0 {
					if out.is_negative() {
						None
					} else {
						Some(out)
					}
				} else {
					let out = out.wrapping_neg();
					if !out.is_negative() {
						None
					} else {
						Some(out)
					}
				}
			},
			None => None,
		}
	}
    
    checked_log!(checked_log2);
    checked_log!(checked_log10);

	#[inline]
	pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
		match self.checked_rem_euclid(rhs) {
			Some(rem) => {
				if rhs.is_negative() {
					self.checked_sub(rem)
				} else if rem.is_zero() {
					Some(self)
				} else {
					self.checked_add(rhs - rem)
				}
			},
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test::{test_bignum, types::*};

    test_bignum! {
        function: <itest>::checked_add(a: itest, b: itest),
        cases: [
            (itest::MAX, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_add_unsigned(a: itest, b: utest)
    }
    test_bignum! {
        function: <itest>::checked_sub(a: itest, b: itest),
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_sub_unsigned(a: itest, b: utest)
    }
    test_bignum! {
        function: <itest>::checked_mul(a: itest, b: itest),
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_div(a: itest, b: itest),
        cases: [
			(23098403i32 as itest, 0i8),
			(itest::MIN, -1i8),
			(8388600i32 as itest, 68201i32 as itest) // tests the unlikely condition
        ]
    }
    test_bignum! {
        function: <itest>::checked_div_euclid(a: itest, b: itest),
        cases: [
			(itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_rem(a: itest, b: itest),
        cases: [
			(itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_rem_euclid(a: itest, b: itest),
        cases: [
			(itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::checked_neg(a: itest),
        cases: [
            (itest::MIN)
        ]
    }
    test_bignum! {
        function: <itest>::checked_shl(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::checked_shr(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::checked_pow(a: itest, b: u16)
    }
}