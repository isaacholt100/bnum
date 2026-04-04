use crate::{Int, Integer};
use crate::OverflowMode;

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> BitAnd for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::bitand(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> BitOr for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> BitXor for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::bitxor(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Not for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Not for &Integer<S, N, B, OM> {
    type Output = Integer<S, N, B, OM>;

    #[inline]
    fn not(self) -> Integer<S, N, B, OM> {
        (*self).not()
    }
}

impl<const N: usize, const B: usize, const OM: u8> Neg for Int<N, B, OM> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const N: usize, const B: usize, const OM: u8> Neg for &Int<N, B, OM> {
    type Output = Int<N, B, OM>;

    #[inline]
    fn neg(self) -> Int<N, B, OM> {
        Int::neg(*self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Add for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Sub for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Mul for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Div for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

// impl<const N: usize> Div<u64> for Uint<N> {
//     type Output = Self;

//     #[inline]
//     fn div(self, rhs: u64) -> Self {
//         self.div_rem_u64(rhs).0
//     }
// }

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Rem for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

// impl<const N: usize> Rem<u64> for Uint<N> {
//     type Output = u64;

//     #[inline]
//     fn rem(self, rhs: u64) -> u64 {
//         self.div_rem_u64(rhs).1
//     }
// }

use crate::helpers::full_op_impl;

full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> BitAnd, BitAndAssign, Integer<S, N, B, OM>, bitand, bitand_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> BitOr, BitOrAssign, Integer<S, N, B, OM>, bitor, bitor_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> BitXor, BitXorAssign, Integer<S, N, B, OM>, bitxor, bitxor_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Add, AddAssign, Integer<S, N, B, OM>, add, add_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Sub, SubAssign, Integer<S, N, B, OM>, sub, sub_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Mul, MulAssign, Integer<S, N, B, OM>, mul, mul_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Div, DivAssign, Integer<S, N, B, OM>, div, div_assign for Integer<S, N, B, OM>);
full_op_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Rem, RemAssign, Integer<S, N, B, OM>, rem, rem_assign for Integer<S, N, B, OM>);

trait ConstOverflowMode {
    const OVERFLOW_MODE: OverflowMode;
}

macro_rules! shift_impl {
    (<$(const $C: ident : $CType: ty), *> $int: ty, $Trait: ident, $AssignTrait: ident, $method: ident, $assign: ident, $err: expr, shift_by: $rhs: ty) => {
        impl<$(const $C: $CType), *> $Trait<$rhs> for $int {
            type Output = $int;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self {
                use crate::Exponent;
                use crate::cast::As;

                if <$rhs>::BITS <= Exponent::BITS {
                    return Self::$method(self, rhs.as_::<Exponent>());
                }

                let rhs: Exponent = match Self::OVERFLOW_MODE {
                    OverflowMode::Wrap => {
                        let bits: $rhs = Self::BITS.as_(); // need to cast to rhs as Self::BITS may not be power of two, so need to perform modulo in $rhs type
                        let rhs = rhs.rem_euclid(bits); // Euclidean remainder as must always be positive (since we could be using signed types)

                        rhs.as_() // now rhs is positive and is less than Self::BITS, so fits in Exponent, so safe to cast
                    },
                    OverflowMode::Panic => Exponent::try_from(rhs).expect(crate::errors::err_msg!($err)),
                    OverflowMode::Saturate => Exponent::try_from(rhs).unwrap_or(Exponent::MAX), // this is fine since we have the assertion in [`Self::from_bytes`] that N < 2^29, so any rhs >= Exponent::MAX will saturate
                };
                
                Self::$method(self, rhs)
            }
        }

        full_op_impl!(<$(const $C: $CType), *> $Trait, $AssignTrait, $rhs, $method, $assign for $int);
    }
}

macro_rules! all_shift_impls {
    ($($int: ty), *) => {
        $(
            impl ConstOverflowMode for $int {
                const OVERFLOW_MODE: OverflowMode = crate::OverflowMode::DEFAULT;
            } // we have to have this so then we can use Self::OVERFLOW_MODE in the same macro for primitive ints and Integer

            shift_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM>, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: $int);

            shift_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM>, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: $int);

            shift_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> $int, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: Integer<S, N, B, OM>);
            shift_impl!(<const S: bool, const N: usize, const B: usize, const OM: u8> $int, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: Integer<S, N, B, OM>);
        )*
    }
}

all_shift_impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

shift_impl!(<const S: bool, const N: usize, const B: usize, const R: bool, const M: usize, const A: usize, const OM: u8> Integer<S, N, B, OM>, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: Integer<R, M, A, OM>);
shift_impl!(<const S: bool, const N: usize, const B: usize, const R: bool, const M: usize, const A: usize, const OM: u8> Integer<S, N, B, OM>, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: Integer<R, M, A, OM>);

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;
    use core::ops::{BitAnd, BitOr, BitXor, Not};

    test_bignum! {
        function: <UTest as BitAnd>::bitand(a: UTest, b: UTest)
    }
    test_bignum! {
        function: <UTest as BitOr>::bitor(a: UTest, b: UTest)
    }
    test_bignum! {
        function: <UTest as BitXor>::bitxor(a: UTest, b: UTest)
    }
    test_bignum! {
        function: <UTest as Not>::not(a: UTest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Exponent;
    use crate::test::test_bignum;

    macro_rules! test_shifts {
        ($($rhs: ty), *) => {
            $(
                crate::test::test_bignum! {
                    function: <STest as Shl<$rhs> >::shl(a: STest, b: $rhs),
                    skip: crate::overflow::GLOBAL_OVERFLOW_CHECKS && match Exponent::try_from(b) {
                        Ok(b) => b >= STest::BITS,
                        Err(_) => true,
                    }
                }
                crate::test::test_bignum! {
                    function: <STest as Shr<$rhs> >::shr(a: STest, b: $rhs),
                    skip: crate::overflow::GLOBAL_OVERFLOW_CHECKS && match Exponent::try_from(b) {
                        Ok(b) => b >= <STest>::BITS,
                        Err(_) => true,
                    }
                }
            )*
        }
    }

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <STest as Add>::add(a: STest, b: STest),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: <STest as BitAnd>::bitand(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as BitOr>::bitor(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as BitXor>::bitxor(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Div>::div(a: STest, b: STest),
            skip: a.checked_div(b).is_none()
        }
        test_bignum! {
            function: <STest as Rem>::rem(a: STest, b: STest),
            skip: a.checked_rem(b).is_none()
        }
        test_bignum! {
            function: <STest as Not>::not(a: STest)
        }
        test_bignum! {
            function: <STest as Sub>::sub(a: STest, b: STest),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: <STest as Mul>::mul(a: STest, b: STest),
            skip: a.checked_mul(b).is_none()
        }

        test_shifts!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    }

    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <ITest>::neg(a: ITest),
            skip: crate::overflow::GLOBAL_OVERFLOW_CHECKS && a == ITest::MIN
        }
    }

    crate::test::test_all! {
        testing signed s;

        test_bignum! {
            function: <ITest>::neg(a: ITest)
        }
    }

    crate::test::test_all! {
        testing signed w;

        test_bignum! {
            function: <ITest>::neg(a: ITest)
        }
    }

    crate::test::test_all! {
        testing integers w; // saturating

        test_bignum! {
            function: <STest as Add>::add(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Sub>::sub(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Mul>::mul(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Div>::div(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest as Rem>::rem(a: STest, b: STest),
            skip: b.is_zero()
        }
        // TODO: test shifts
    }

    crate::test::test_all! {
        testing integers s; // saturating

        test_bignum! {
            function: <STest as Add>::add(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Sub>::sub(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Mul>::mul(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Div>::div(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest as Rem>::rem(a: STest, b: STest),
            skip: b.is_zero()
        }
        // TODO: test shifts
    }
}
