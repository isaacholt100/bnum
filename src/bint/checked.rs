use super::BInt;
use crate::{ExpType, BUint};
use crate::doc;
use crate::int::checked::tuple_to_option;

macro_rules! checked_log {
    ($method: ident $(, $base: ident: $ty: ty)?) => {
        #[inline]
        pub fn $method(self $(, $base: $ty)?) -> Option<ExpType> {
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
	use crate::test::test_bignum;

    test_bignum! {
        function: <i128>::checked_add(a: i128, b: i128),
        cases: [
            (i128::MAX, 1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_add_unsigned(a: i128, b: u128)
    }
    test_bignum! {
        function: <i128>::checked_sub(a: i128, b: i128),
        cases: [
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_sub_unsigned(a: i128, b: u128)
    }
    test_bignum! {
        function: <i128>::checked_mul(a: i128, b: i128),
        cases: [
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_div(a: i128, b: i128),
        cases: [
            (-34564564564i128, -33219654565456456453434545697i128),
			(i128::MIN, -1i128),
			(8388600, 68201) // tests the unlikely condition
        ]
    }
    test_bignum! {
        function: <i128>::checked_div_euclid(a: i128, b: i128),
        cases: [
			(i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_rem(a: i128, b: i128),
        cases: [
			(i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_rem_euclid(a: i128, b: i128),
        cases: [
			(i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::checked_neg(a: i128),
        cases: [
            (i128::MIN)
        ]
    }
    test_bignum! {
        function: <i128>::checked_shl(a: i128, b: u16),
        cases: [
            (1907304597249567965987i128, 21 as u16),
            (-2023973209458764967589i128, 15 as u16),
            (2845197495679875698546i128, 8457 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::checked_shr(a: i128, b: u16),
        cases: [
            (61354072459679717429576097i128, 120 as u16),
            (-23045692977456978956795i128, 18 as u16),
            (203967947569745986748956i128, 128 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::checked_pow(a: i128, b: u16),
        cases: [
            (-13i128, 22 as u16),
            (7i128, 29 as u16)
        ]
    }
}