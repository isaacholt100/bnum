macro_rules! op_ref_impl {
    ($tr: ident <$rhs: ty> for $Struct: ident <$($C: ident),+>, $method: ident) => {
        impl<$(const $C: usize),+> $tr<&$rhs> for $Struct <$($C),+> {
            type Output = $Struct <$($C),+>;

            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $tr::<$rhs>::$method(self, *rhs)
            }
        }

        impl<$(const $C: usize),+> $tr<&$rhs> for &$Struct <$($C),+> {
            type Output = $Struct <$($C),+>;

            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                $tr::<$rhs>::$method(*self, *rhs)
            }
        }

        impl<$(const $C: usize),+> $tr<$rhs> for &$Struct <$($C),+> {
            type Output = $Struct <$($C),+>;

            #[inline]
            fn $method(self, rhs: $rhs) -> Self::Output {
                $tr::<$rhs>::$method(*self, rhs)
            }
        }
    }
}
pub(crate) use op_ref_impl;

macro_rules! assign_op_impl {
    ($OpTrait: ident, $AssignTrait: ident<$rhs: ty> for $Struct: ident, $assign: ident, $op: ident) => {
        impl<const N: usize> $AssignTrait<$rhs> for $Struct<N> {
            #[inline]
            fn $assign(&mut self, rhs: $rhs) {
                *self = $OpTrait::$op(*self, rhs);
            }
        }

        impl<const N: usize> $AssignTrait<&$rhs> for $Struct<N> {
            #[inline]
            fn $assign(&mut self, rhs: &$rhs) {
                self.$assign(*rhs);
            }
        }

        crate::int::ops::op_ref_impl!($OpTrait<$rhs> for $Struct<N>, $op);
    }
}
pub(crate) use assign_op_impl;

macro_rules! shift_impl {
    ($Struct: ident, $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for $Struct<N> {
                type Output = Self;

                #[inline]
                fn $method(self, rhs: $rhs) -> Self {
                    self.$method(rhs as crate::ExpType)
                }
            }
        )*
    }
}
pub(crate) use shift_impl;

macro_rules! try_shift_impl {
    ($Struct: ident; $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for $Struct<N> {
                type Output = Self;

                #[inline]
                fn $method(self, rhs: $rhs) -> Self {
                    use crate::ExpType;
                    #[cfg(debug_assertions)]
                    let rhs: ExpType = crate::errors::result_expect!(ExpType::try_from(rhs), crate::errors::err_msg!($err));

                    #[cfg(not(debug_assertions))]
                    let rhs = rhs as ExpType;
                    self.$method(rhs)
                }
            }
        )*
    }
}
pub(crate) use try_shift_impl;

macro_rules! shift_self_impl {
    ($Struct: ident; $tr: tt<$rhs: tt>, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr) => {
        impl<const N: usize, const M: usize> $tr<$rhs<M>> for $Struct<N> {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: $rhs<M>) -> Self {
                use crate::ExpType;
                let rhs: ExpType = crate::errors::result_expect!(
                    ExpType::try_from(rhs),
                    crate::errors::err_msg!($err)
                );
                self.$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for $Struct<N> {
            type Output = $Struct<N>;

            #[inline]
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                $tr::<$rhs<M>>::$method(self, *rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;

            #[inline]
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                $tr::<$rhs<M>>::$method(*self, *rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;

            #[inline]
            fn $method(self, rhs: $rhs<M>) -> Self::Output {
                $tr::<$rhs<M>>::$method(*self, rhs)
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<$rhs<M>> for $Struct<N> {
            #[inline]
            fn $assign_method(&mut self, rhs: $rhs<M>) {
                *self = $tr::<$rhs<M>>::$method(*self, rhs);
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<&$rhs<M>> for $Struct<N> {
            #[inline]
            fn $assign_method(&mut self, rhs: &$rhs<M>) {
                (*self).$assign_method(*rhs);
            }
        }
    };
}
pub(crate) use shift_self_impl;

macro_rules! all_shift_impls {
    ($Struct: ident) => {
        crate::int::ops::try_shift_impl!(
            $Struct;
            Shl,
            shl,
            ShlAssign,
            shl_assign,
            "attempt to shift left with overflow",
            i8,
            i16,
            i32,
            isize,
            i64,
            i128
        );

        crate::int::ops::try_shift_impl!(
            $Struct;
            Shr,
            shr,
            ShrAssign,
            shr_assign,
            "attempt to shift right with overflow",
            i8,
            i16,
            i32,
            isize,
            i64,
            i128
        );

        crate::int::ops::shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, u8, u16);

        crate::int::ops::shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, u8, u16);

        crate::int::ops::try_shift_impl!(
            $Struct;
            Shl,
            shl,
            ShlAssign,
            shl_assign,
            "attempt to shift left with overflow",
            usize,
            u64,
            u128
        );

        crate::int::ops::try_shift_impl!(
            $Struct;
            Shr,
            shr,
            ShrAssign,
            shr_assign,
            "attempt to shift right with overflow",
            usize,
            u64,
            u128
        );

        crate::int::ops::shift_self_impl!(
            $Struct;
            Shl<BUintD8>,
            shl,
            ShlAssign,
            shl_assign,
            "attempt to shift left with overflow"
        );

        crate::int::ops::shift_self_impl!(
            $Struct;
            Shr<BUintD8>,
            shr,
            ShrAssign,
            shr_assign,
            "attempt to shift right with overflow"
        );

        crate::int::ops::shift_self_impl!(
            $Struct;
            Shl<BIntD8>,
            shl,
            ShlAssign,
            shl_assign,
            "attempt to shift left with overflow"
        );

        crate::int::ops::shift_self_impl!(
            $Struct;
            Shr<BIntD8>,
            shr,
            ShrAssign,
            shr_assign,
            "attempt to shift right with overflow"
        );
    };
}

pub(crate) use all_shift_impls;

macro_rules! shift_assign_ops {
    ($OpTrait: ident, $AssignTrait: ident <$($rhs: ty), *> for $Struct: ident, $assign: ident, $op: ident) => {
        $(
            crate::int::ops::assign_op_impl!($OpTrait, $AssignTrait<$rhs> for $Struct, $assign, $op);
        )*
    };
}
pub(crate) use shift_assign_ops;

macro_rules! trait_fillers {
    () => {
        #[inline]
        pub const fn add(self, rhs: Self) -> Self {
            #[cfg(debug_assertions)]
            return self.strict_add(rhs);

            #[cfg(not(debug_assertions))]
            self.wrapping_add(rhs)
        }

        #[inline]
        pub const fn mul(self, rhs: Self) -> Self {
            #[cfg(debug_assertions)]
            return self.strict_mul(rhs);

            #[cfg(not(debug_assertions))]
            self.wrapping_mul(rhs)
        }

        #[inline]
        pub const fn shl(self, rhs: ExpType) -> Self {
            #[cfg(debug_assertions)]
            return self.strict_shl(rhs);

            #[cfg(not(debug_assertions))]
            self.wrapping_shl(rhs)
        }

        #[inline]
        pub const fn shr(self, rhs: ExpType) -> Self {
            #[cfg(debug_assertions)]
            return self.strict_shr(rhs);

            #[cfg(not(debug_assertions))]
            self.wrapping_shr(rhs)
        }

        #[inline]
        pub const fn sub(self, rhs: Self) -> Self {
            #[cfg(debug_assertions)]
            return self.strict_sub(rhs);

            #[cfg(not(debug_assertions))]
            self.wrapping_sub(rhs)
        }
    };
}

pub(crate) use trait_fillers;

macro_rules! impls {
    ($Struct: ident) => {
        impl<const N: usize> Add<Self> for $Struct<N> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self::add(self, rhs)
            }
        }

        impl<const N: usize> Mul for $Struct<N> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self::mul(self, rhs)
            }
        }

        impl<const N: usize> Not for &$Struct<N> {
            type Output = $Struct<N>;

            #[inline]
            fn not(self) -> $Struct<N> {
                (*self).not() // TODO: maybe use separate impl for this as well
            }
        }

        impl<const N: usize> Shl<ExpType> for $Struct<N> {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: ExpType) -> Self {
                Self::shl(self, rhs)
            }
        }

        impl<const N: usize> Shr<ExpType> for $Struct<N> {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: ExpType) -> Self {
                Self::shr(self, rhs)
            }
        }

        crate::int::ops::all_shift_impls!($Struct);

        impl<const N: usize> Sub for $Struct<N> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self::sub(self, rhs)
            }
        }

        crate::int::ops::assign_op_impl!(Add, AddAssign<$Struct<N>> for $Struct, add_assign, add);
        crate::int::ops::assign_op_impl!(BitAnd, BitAndAssign<$Struct<N>> for $Struct, bitand_assign, bitand);
        crate::int::ops::assign_op_impl!(BitOr, BitOrAssign<$Struct<N>> for $Struct, bitor_assign, bitor);
        crate::int::ops::assign_op_impl!(BitXor, BitXorAssign<$Struct<N>> for $Struct, bitxor_assign, bitxor);
        crate::int::ops::assign_op_impl!(Div, DivAssign<$Struct<N>> for $Struct, div_assign, div);
        crate::int::ops::assign_op_impl!(Mul, MulAssign<$Struct<N>> for $Struct, mul_assign, mul);
        crate::int::ops::assign_op_impl!(Rem, RemAssign<$Struct<N>> for $Struct, rem_assign, rem);

        crate::int::ops::shift_assign_ops!(Shl, ShlAssign<u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize> for $Struct, shl_assign, shl);

        crate::int::ops::shift_assign_ops!(Shr, ShrAssign<u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize> for $Struct, shr_assign, shr);

        crate::int::ops::assign_op_impl!(Sub, SubAssign<$Struct<N>> for $Struct, sub_assign, sub);
    };
}
pub(crate) use impls;

#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        #[allow(unused_imports)]
        use super::*;

        test_bignum! {
            function: <$int as Add>::add(a: $int, b: $int),
            skip: a.checked_add(b).is_none()
        }

        test_bignum! {
            function: <$int as BitAnd>::bitand(a: $int, b: $int)
        }

        test_bignum! {
            function: <$int as BitOr>::bitor(a: $int, b: $int)
        }

        test_bignum! {
            function: <$int as BitXor>::bitxor(a: $int, b: $int)
        }

        test_bignum! {
            function: <$int as Div>::div(a: $int, b: $int),
            skip: a.checked_div(b).is_none()
        }

        test_bignum! {
            function: <$int as Rem>::rem(a: $int, b: $int),
            skip: a.checked_rem(b).is_none()
        }

        test_bignum! {
            function: <$int as Not>::not(a: $int)
        }

        test_bignum! {
            function: <$int as Sub>::sub(a: $int, b: $int),
            skip: a.checked_sub(b).is_none()
        }

        test_bignum! {
            function: <$int as Mul>::mul(a: $int, b: $int),
            skip: a.checked_mul(b).is_none()
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
