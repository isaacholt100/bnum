use super::BInt;
use crate::doc;
use crate::int::checked::tuple_to_option;
use crate::nightly::{const_fn, const_fns};
use crate::{BUint, ExpType};

macro_rules! checked_log {
    ($method: ident $(, $base: ident: $ty: ty)?) => {
		const_fn! {
			#[doc = doc::checked::$method!(I)]
			#[must_use = doc::must_use_op!()]
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
}

#[doc = doc::checked::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[doc = doc::checked::checked_add!(I)]
	#[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }

    #[doc = doc::checked::checked_add_unsigned!(I)]
	#[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    #[doc = doc::checked::checked_sub!(I)]
	#[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }

    #[doc = doc::checked::checked_sub_unsigned!(I)]
	#[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    const_fns! {
        #[doc = doc::checked::checked_mul!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
            tuple_to_option(self.overflowing_mul(rhs))
        }

        #[doc = doc::checked::checked_div!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_div(self, rhs: Self) -> Option<Self> {
            if rhs.is_zero() {
                None
            } else {
                tuple_to_option(self.overflowing_div(rhs))
            }
        }

        #[doc = doc::checked::checked_div_euclid!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
            if rhs.is_zero() {
                None
            } else {
                tuple_to_option(self.overflowing_div_euclid(rhs))
            }
        }

        #[doc = doc::checked::checked_rem!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
            if rhs.is_zero() {
                None
            } else {
                tuple_to_option(self.overflowing_rem(rhs))
            }
        }

        #[doc = doc::checked::checked_rem_euclid!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
            if rhs.is_zero() {
                None
            } else {
                tuple_to_option(self.overflowing_rem_euclid(rhs))
            }
        }

        #[doc = doc::checked::checked_neg!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_neg(self) -> Option<Self> {
            tuple_to_option(self.overflowing_neg())
        }

        #[doc = doc::checked::checked_shl!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
            tuple_to_option(self.overflowing_shl(rhs))
        }

        #[doc = doc::checked::checked_shr!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
            tuple_to_option(self.overflowing_shr(rhs))
        }

        #[doc = doc::checked::checked_abs!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_abs(self) -> Option<Self> {
            tuple_to_option(self.overflowing_abs())
        }

        #[doc = doc::checked::checked_pow!(I)]
		#[must_use = doc::must_use_op!()]
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

        #[doc = doc::checked::checked_next_multiple_of!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
            if rhs.is_zero() {
                return None;
            }
            let rem = self.wrapping_rem_euclid(rhs);
            if rem.is_zero() {
                return Some(self);
            }
            if rem.is_negative() == rhs.is_negative() {
                self.checked_add(rhs.wrapping_sub(rem))
            } else {
                self.checked_sub(rem)
            }
        }

        #[doc = doc::checked::checked_log!(I)]
		#[must_use = doc::must_use_op!()]
		#[inline]
        pub const fn checked_log(self, base: Self) -> Option<ExpType> {
            if base.is_negative() || self.is_negative() {
                None
            } else {
                self.to_bits().checked_log(base.to_bits())
            }
        }
    }

    checked_log!(checked_log2);
    checked_log!(checked_log10);
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
            (8388600i32 as itest, 68201i32 as itest) // tests the unlikely condition in the division algorithm at step D5
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
        skip: b <= u8::MAX as itest,
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
