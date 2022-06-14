use super::Bint;
use crate::{ExpType, BUint, doc};

#[doc=doc::wrapping::impl_desc!()]
impl<const N: usize> Bint<N> {
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_add(rhs.bits))
    }

    #[inline]
    pub const fn wrapping_add_unsigned(self, rhs: BUint<N>) -> Self {
        self.overflowing_add_unsigned(rhs).0
    }

    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_sub(rhs.bits))
    }

    #[inline]
    pub const fn wrapping_sub_unsigned(self, rhs: BUint<N>) -> Self {
        self.overflowing_sub_unsigned(rhs).0
    }

    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self::from_bits(self.bits.wrapping_mul(rhs.bits))
    }

    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
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

    crate::macros::wrapping_pow!();

    #[inline]
    pub const fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }
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