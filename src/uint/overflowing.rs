use super::BUint;
use crate::macros::overflowing_pow;
use crate::ExpType;
use crate::digit::Digit;
use crate::Bint;

//const LONG_MUL_THRESHOLD: usize = 32;
//const KARATSUBA_THRESHOLD: usize = 256;

/// Tuple of (product, carry)
impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = false;
        let mut i = 0;
        while i < N {
            let result = self.digits[i].carrying_add(rhs.digits[i], carry);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        (out, carry)
    }

    #[inline]
    pub const fn overflowing_add_signed(self, rhs: Bint<N>) -> (Self, bool) {
        let (out, overflow) = self.overflowing_add(rhs.to_bits());
        (out, overflow ^ rhs.is_negative())
    }

    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = false;
        let mut i = 0;
        while i < N {
            let result = self.digits[i].borrowing_sub(rhs.digits[i], borrow);
            out.digits[i] = result.0;
            borrow = result.1;
            i += 1;
        }
        (out, borrow)
    }

    #[inline]
    const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut carry: Digit;
        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = super::carrying_mul(self.digits[i], rhs.digits[j], carry, out.digits[index]);
                    out.digits[index] = prod;
                    carry = c;
                } else if (self.digits[i] != 0 && rhs.digits[j] != 0) || carry != 0 {
                    overflow = true;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        (out, overflow)
    }

    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        self.long_mul(rhs)
    }
    /*const fn overflowing_mul_digit(self, rhs: Digit) -> (Self, Digit) {
        let mut out = Self::ZERO;
        let mut carry: Digit = 0;
        let mut i = 0;
        while i < N {
            let (prod, c) = arch::mul_carry_unsigned(carry, 0, self.digits[i], rhs);
            out.digits[i] = prod;
            carry = c;
            i += 1;
        }
        (out, carry)
    }*/
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }

    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_rem(rhs), false)
    }

    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }

    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (a, b) = (!self).overflowing_add(Self::ONE);
        (a, !b)
    }

    #[inline]
    pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
        if rhs >= Self::BITS {
            (super::unchecked_shl(self, rhs & Self::BITS_MINUS_1), true)
        } else {
            (super::unchecked_shl(self, rhs), false)
        }
    }

    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        if rhs >= Self::BITS {
            //assert_eq!(rhs & Self::BITS_MINUS_1, 13);
            (super::unchecked_shr(self, rhs & Self::BITS_MINUS_1), true)
        } else {
            (super::unchecked_shr(self, rhs), false)
        }
    }
    
    overflowing_pow!();
}

#[cfg(test)]
mod tests {
    use crate::test::converters;

    test_unsigned! {
        function: overflowing_add(a: u128, b: u128),
        cases: [
            (u128::MAX - 35348957, 34059304859034578959083490834850937458u128),
            (34987358947598374835u128, 340593453454564568u128)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_sub(a: u128, b: u128),
        cases: [
            (34053457987u128, 34059304859034578959083490834850937458u128),
            (34987358947598374835345345345454645645u128, 9856946974958764564564508456849058u128)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_mul(a: u128, b: u128),
        cases: [
            (93875893745946675675675675745687345u128, 394857456456456456434534355645384975u128),
            (103453534455674958789u128, 509u128)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_div(a: u128, b: u128),
        cases: [
            (103573984758937498573594857389345u128, 3453454545345345345987u128),
            (193679457916593485358497389457u128, 684u128)
        ],
        quickcheck_skip: b == 0,
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_div_euclid(a: u128, b: u128),
        cases: [
            (349573947593745898375u128, 349573947593745898375u128),
            (0u128, 3459745734895734957984579u128)
        ],
        quickcheck_skip: b == 0,
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_rem(a: u128, b: u128),
        cases: [
            (2973459793475897343495439857u128, 56u128),
            (1u128 << 64, 2u128)
        ],
        quickcheck_skip: b == 0,
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_rem_euclid(a: u128, b: u128),
        cases: [
            (27943758345638459034898756847983745u128, 37589734758937458973459u128),
            (0u128, 93745934953894u128)
        ],
        quickcheck_skip: b == 0,
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_neg(a: u128),
        cases: [
            (0u128),
            (93498734534534273984577u128)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_shl(a: u128, b: u16),
        cases: [
            (u128::MAX - 3453475, 5 as u16),
            (934987774987u128, 55645 as u16)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_shr(a: u128, b: u16),
        cases: [
            (349573947593475973453348759u128, 10 as u16),
            (972456948567894576895749857u128, 58969 as u16)
        ],
        converter: converters::tuple_converter
    }
    test_unsigned! {
        function: overflowing_pow(a: u128, b: u16),
        cases: [
            (3444334u128, 34345 as u16),
            (23u128, 31 as u16)
        ],
        converter: converters::tuple_converter
    }
}