macro_rules! op_ref_impl {
    ($tr: ident <$rhs: ty> for $Struct: ident <$($C: ident),+>, $method: ident) => {
        impl<$(const $C: usize),+> $tr<&$rhs> for $Struct <$($C),+> {
            type Output = $Struct <$($C),+>;
        
            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<$(const $C: usize),+> $tr<&$rhs> for &$Struct <$($C),+> {
            type Output = $Struct <$($C),+>;
        
            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<$(const $C: usize),+> $tr<$rhs> for &$Struct <$($C),+> {
            type Output = $Struct <$($C),+>;
        
            #[inline]
            fn $method(self, rhs: $rhs) -> Self::Output {
                (*self).$method(rhs)
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
                *self = (*self).$op(rhs);
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
    ($Struct: tt, $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $($rhs: ty), *) => {
        $(
            impl<const N: usize> const $tr<$rhs> for $Struct<N> {
                type Output = Self;

                #[inline]
                fn $method(self, rhs: $rhs) -> Self {
                    use crate::ExpType;
                    self.$method(rhs as ExpType)
                }
            }
        )*
    }
}
pub(crate) use shift_impl;

macro_rules! try_shift_impl {
    ($Struct: tt, $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for $Struct<N> {
                type Output = Self;

                #[inline]
                fn $method(self, rhs: $rhs) -> Self {
                    use crate::ExpType;
                    #[cfg(debug_assertions)]
                    let rhs: ExpType = crate::macros::option_expect!(rhs.try_into().ok(), crate::error::err_msg!($err));

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
    ($Struct: tt, $tr: tt<$rhs: tt>, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr) => {
        impl<const N: usize, const M: usize> $tr<$rhs<M>> for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn $method(self, rhs: $rhs<M>) -> Self {
                use crate::ExpType;
                let rhs: ExpType = crate::macros::option_expect!(rhs.try_into().ok(), crate::error::err_msg!($err));
                self.$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for $Struct<N> {
            type Output = $Struct<N>;
        
            #[inline]
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;
        
            #[inline]
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;
        
            #[inline]
            fn $method(self, rhs: $rhs<M>) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<$rhs<M>> for $Struct<N> {
            #[inline]
            fn $assign_method(&mut self, rhs: $rhs<M>) {
                *self = (*self).$method(rhs);
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<&$rhs<M>> for $Struct<N> {
            #[inline]
            fn $assign_method(&mut self, rhs: &$rhs<M>) {
                (*self).$assign_method(*rhs);
            }
        }
    }
}
pub(crate) use shift_self_impl;

macro_rules! all_shift_impls {
    ($Struct: tt) => {
        use core::convert::TryInto;
        use crate::prelude::*;

        crate::int::ops::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", i8, i16, i32, isize, i64, i128);

        crate::int::ops::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", i8, i16, i32, isize, i64, i128);

        #[cfg(feature="usize_exptype")]
        crate::int::ops::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", u32, u64, u128);

        #[cfg(feature="usize_exptype")]
        crate::int::ops::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", u32, u64, u128);
        
        crate::int::ops::shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, u8, u16);

        crate::int::ops::shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, u8, u16);

        #[cfg(not(feature="usize_exptype"))]
        crate::int::ops::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", usize, u64, u128);

        #[cfg(not(feature="usize_exptype"))]
        crate::int::ops::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", usize, u64, u128);

        crate::int::ops::shift_self_impl!($Struct, Shl<BUint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::int::ops::shift_self_impl!($Struct, Shr<BUint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");

        crate::int::ops::shift_self_impl!($Struct, Shl<BInt>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::int::ops::shift_self_impl!($Struct, Shr<BInt>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");
    }
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

macro_rules! impls {
    ($Struct: ident) => {
        impl<const N: usize> const Add<Self> for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn add(self, rhs: Self) -> Self {
                #[cfg(debug_assertions)]
                return crate::macros::option_expect!(self.checked_add(rhs), "attempt to add with overflow");
        
                #[cfg(not(debug_assertions))]
                self.wrapping_add(rhs)
            }
        }
        
        impl<const N: usize> const Mul for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                #[cfg(debug_assertions)]
                return crate::macros::option_expect!(self.checked_mul(rhs), "attempt to multiply with overflow");
        
                #[cfg(not(debug_assertions))]
                self.wrapping_mul(rhs)
            }
        }
        
        impl<const N: usize> const Not for &$Struct<N> {
            type Output = $Struct<N>;
        
            #[inline]
            fn not(self) -> $Struct<N> {
                !(*self)
            }
        }
        
        impl<const N: usize> const Shl<ExpType> for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn shl(self, rhs: ExpType) -> Self {
                #[cfg(debug_assertions)]
                return crate::macros::option_expect!(self.checked_shl(rhs), "attempt to shift left with overflow");
        
                #[cfg(not(debug_assertions))]
                self.wrapping_shl(rhs)
            }
        }
        
        impl<const N: usize> const Shr<ExpType> for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn shr(self, rhs: ExpType) -> Self {
                #[cfg(debug_assertions)]
                return crate::macros::option_expect!(self.checked_shr(rhs), "attempt to shift left with overflow");
        
                #[cfg(not(debug_assertions))]
                self.wrapping_shr(rhs)
            }
        }
        
        crate::int::ops::all_shift_impls!($Struct);
        
        impl<const N: usize> const Sub for $Struct<N> {
            type Output = Self;
        
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                #[cfg(debug_assertions)]
                return crate::macros::option_expect!(self.checked_sub(rhs), "attempt to subtract with overflow");
        
                #[cfg(not(debug_assertions))]
                self.wrapping_sub(rhs)
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

#[allow(unused)]
macro_rules! tests {
	($int: ty) => {
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

#[allow(unused)]
pub(crate) use tests;