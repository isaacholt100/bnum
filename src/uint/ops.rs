use crate::ExpType;
use crate::{Int, Integer, Uint};

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl<const S: bool, const N: usize> BitAnd for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::bitand(self, rhs)
    }
}

impl<const S: bool, const N: usize> BitOr for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const S: bool, const N: usize> BitXor for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::bitxor(self, rhs)
    }
}

impl<const S: bool, const N: usize> Not for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const S: bool, const N: usize> Not for &Integer<S, N> {
    type Output = Integer<S, N>;

    #[inline]
    fn not(self) -> Integer<S, N> {
        (*self).not()
    }
}

impl<const N: usize> Neg for Int<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const N: usize> Neg for &Int<N> {
    type Output = Int<N>;

    #[inline]
    fn neg(self) -> Int<N> {
        Int::neg(*self)
    }
}

impl<const S: bool, const N: usize> Add for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

impl<const S: bool, const N: usize> Sub for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

impl<const S: bool, const N: usize> Mul for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs)
    }
}

impl<const S: bool, const N: usize> Div for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

impl<const N: usize> Div<u64> for Uint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u64) -> Self {
        self.div_rem_u64(rhs).0
    }
}

impl<const S: bool, const N: usize> Rem for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

impl<const N: usize> Rem<u64> for Uint<N> {
    type Output = u64;

    #[inline]
    fn rem(self, rhs: u64) -> u64 {
        self.div_rem_u64(rhs).1
    }
}

impl<const S: bool, const N: usize> Shl<ExpType> for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: ExpType) -> Self {
        Self::shl(self, rhs)
    }
}

impl<const S: bool, const N: usize> Shr<ExpType> for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: ExpType) -> Self {
        Self::shr(self, rhs)
    }
}

macro_rules! full_op_impl {
    (<$(const $C: ident : $CType: ty), +> $OpTrait: ident, $AssignTrait: ident, $rhs: ty, $op: ident, $assign: ident for $int: ty) => {
        impl<$(const $C: $CType), +> $OpTrait<&$rhs> for $int {
            type Output = $int;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                $OpTrait::<$rhs>::$op(self, *rhs)
            }
        }

        impl<$(const $C: $CType), +> $OpTrait<&$rhs> for &$int {
            type Output = $int;

            #[inline]
            fn $op(self, rhs: &$rhs) -> Self::Output {
                $OpTrait::<$rhs>::$op(*self, *rhs)
            }
        }

        impl<$(const $C: $CType), +> $OpTrait<$rhs> for &$int {
            type Output = $int;

            #[inline]
            fn $op(self, rhs: $rhs) -> Self::Output {
                $OpTrait::<$rhs>::$op(*self, rhs)
            }
        }

        impl<$(const $C: $CType), +> $AssignTrait<$rhs> for $int {
            #[inline]
            fn $assign(&mut self, rhs: $rhs) {
                *self = $OpTrait::$op(*self, rhs);
            }
        }

        impl<$(const $C: $CType), +> $AssignTrait<&$rhs> for $int {
            #[inline]
            fn $assign(&mut self, rhs: &$rhs) {
                self.$assign(*rhs);
            }
        }
    }
}

pub(crate) use full_op_impl;

full_op_impl!(<const S: bool, const N: usize> BitAnd, BitAndAssign, Integer<S, N>, bitand, bitand_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> BitOr, BitOrAssign, Integer<S, N>, bitor, bitor_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> BitXor, BitXorAssign, Integer<S, N>, bitxor, bitxor_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> Add, AddAssign, Integer<S, N>, add, add_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> Sub, SubAssign, Integer<S, N>, sub, sub_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> Mul, MulAssign, Integer<S, N>, mul, mul_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> Div, DivAssign, Integer<S, N>, div, div_assign for Integer<S, N>);
full_op_impl!(<const S: bool, const N: usize> Rem, RemAssign, Integer<S, N>, rem, rem_assign for Integer<S, N>);

macro_rules! shift_impl {
    (<$(const $C: ident : $CType: ty), *> $int: ty, $Trait: ident, $AssignTrait: ident, $method: ident, $assign: ident, $err: expr, shift_by: $rhs: ty) => {
        impl<$(const $C: $CType), *> $Trait<$rhs> for $int {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self {
                use crate::ExpType;
                use crate::cast::As;

                if <$rhs>::BITS <= ExpType::BITS {
                    return Self::$method(self, rhs.as_::<ExpType>());
                }

                if crate::OVERFLOW_CHECKS {
                    let rhs: ExpType = ExpType::try_from(rhs).expect(crate::errors::err_msg!($err));
                    return Self::$method(self, rhs);
                }

                let bits: $rhs = Self::BITS.as_(); // need to cast to rhs as Self::BITS may not be power of two, so need to perform modulo in $rhs type
                let rhs = rhs.rem_euclid(bits); // Euclidean remainder as must always be positive (since we could be using signed types)

                let rhs: ExpType = rhs.as_(); // now rhs is positive and is less than Self::BITS, so fits in ExpType, so safe to cast

                Self::$method(self, rhs)
            }
        }

        full_op_impl!(<$(const $C: $CType), *> $Trait, $AssignTrait, $rhs, $method, $assign for $int);
    }
}

macro_rules! all_shift_impls {
    ($($int: ty), *) => {
        $(
            shift_impl!(<const S: bool, const N: usize> Integer<S, N>, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: $int);

            shift_impl!(<const S: bool, const N: usize> Integer<S, N>, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: $int);

            shift_impl!(<const S: bool, const N: usize> $int, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: Integer<S, N>);

            shift_impl!(<const S: bool, const N: usize> $int, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: Integer<S, N>);
        )*
    }
}

all_shift_impls!(u8, u16, u64, u128, usize, i8, i16, i32, i64, i128, isize);

shift_impl!(<const S: bool, const N: usize, const R: bool, const M: usize> Integer<S, N>, Shl, ShlAssign, shl, shl_assign, "attempt to shift left with overflow", shift_by: Integer<R, M>);
shift_impl!(<const S: bool, const N: usize, const R: bool, const M: usize> Integer<S, N>, Shr, ShrAssign, shr, shr_assign, "attempt to shift right with overflow", shift_by: Integer<R, M>);

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;
    use core::ops::{BitAnd, BitOr, BitXor, Not};

    test_bignum! {
        function: <utest as BitAnd>::bitand(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as BitOr>::bitor(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as BitXor>::bitxor(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as Not>::not(a: utest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{test_bignum, debug_skip};

    macro_rules! test_shifts {
        ($($rhs: ty), *) => {
            $(
                crate::test::test_bignum! {
                    function: <stest as Shl<$rhs> >::shl(a: stest, b: $rhs),
                    skip: debug_skip!(match ExpType::try_from(b) {
                        Ok(b) => b >= stest::BITS,
                        Err(_) => true,
                    })
                }
                crate::test::test_bignum! {
                    function: <stest as Shr<$rhs> >::shr(a: stest, b: $rhs),
                    skip: debug_skip!(match ExpType::try_from(b) {
                        Ok(b) => b >= <stest>::BITS,
                        Err(_) => true,
                    })
                }
            )*
        }
    }

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest as Add>::add(a: stest, b: stest),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: <stest as BitAnd>::bitand(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as BitOr>::bitor(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as BitXor>::bitxor(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as Div>::div(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
        test_bignum! {
            function: <stest as Rem>::rem(a: stest, b: stest),
            skip: a.checked_rem(b).is_none()
        }
        test_bignum! {
            function: <stest as Not>::not(a: stest)
        }
        test_bignum! {
            function: <stest as Sub>::sub(a: stest, b: stest),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: <stest as Mul>::mul(a: stest, b: stest),
            skip: a.checked_mul(b).is_none()
        }

        test_shifts!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    }

    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::neg(a: itest),
            skip: debug_skip!(a == itest::MIN)
        }
    }
}
