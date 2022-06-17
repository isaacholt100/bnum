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
	use crate::test::test_bignum;
	
    test_bignum! {
        function: <i128>::wrapping_add(a: i128, b: i128),
        cases: [
            (i128::MAX, 1i128),
            (i128::MIN, i128::MIN),
            (i128::MIN, i128::MAX)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_sub(a: i128, b: i128),
        cases: [
            (i128::MIN, i128::MAX),
            (i128::MAX, i128::MIN),
            (87297694759687456797345867i128, -204958769457689745896i128)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_mul(a: i128, b: i128),
        cases: [
            (i128::MIN, -1i128),
            (i128::MIN, i128::MIN),
            (i128::MAX, i128::MAX),
            (i128::MIN, i128::MAX)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_div(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (2459797939576437967897567567i128, -9739457689456456456456i128),
            (-247596802947563975967497564596456i128, -29475697294564566i128),
            (-274957645i128, -12074956872947569874895376456465456i128)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_div_euclid(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (2745345697293475693745979367987567i128, 8956274596748957689745867456456i128),
            (-72957698456456573567576i128, 9874i128)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_rem(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (-234645634564356456i128, 234645634564356456 * 247459769457645i128),
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_rem_euclid(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (-823476907495675i128, 823476907495675i128 * 74859607789455i128)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_neg(a: i128),
        cases: [
            (-27946793759876567567567i128),
            (947697985762756972498576874856i128),
            (i128::MIN)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_shl(a: i128, b: u16),
        cases: [
            (792470986798345769745645685645i128, 2165 as u16),
            (-72459867475967498576849565i128, 1523 as u16),
            (i128::MIN, 8457 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_shr(a: i128, b: u16),
        cases: [
            (-1i128, 101 as u16),
            (-72985769795768973458967984576i128, 1658 as u16),
            (1797465927984576982745896799i128, 15128 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::wrapping_pow(a: i128, b: u16),
        cases: [
            (-79576456i128, 14500 as u16),
            (-14i128, 17 as u16)
        ]
    }
}