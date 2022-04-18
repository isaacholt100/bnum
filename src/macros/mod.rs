macro_rules! div_zero {
    () => {
        panic!("attempt to divide by zero")
    };
}
pub(crate) use div_zero;

macro_rules! rem_zero {
    () => {
        panic!("attempt to calculate remainder with a divisor of zero")
    };
}
pub(crate) use rem_zero;

macro_rules! try_int_impl {
    ($Struct: tt, $int: ty, $method: ident, $err: expr) => {
        impl<const N: usize> TryFrom<$Struct<N>> for $int {
            type Error = crate::TryFromIntError;
        
            #[inline]
            fn try_from(uint: $Struct<N>) -> Result<Self, Self::Error> {
                uint.$method().ok_or(crate::TryFromIntError {
                    from: stringify!($Struct),
                    to: stringify!($int),
                    reason: crate::error::TryFromErrorReason::TooLarge,
                })
            }
        }
    }
}
pub(crate) use try_int_impl;

macro_rules! all_try_int_impls {
    ($Struct: tt) => {
        crate::macros::try_int_impl!($Struct, u128, to_u128, "BUint is too large to cast to u128");
        crate::macros::try_int_impl!($Struct, u64, to_u64, "BUint is too large to cast to u64");
        crate::macros::try_int_impl!($Struct, usize, to_usize, "BUint is too large to cast to usize");
        crate::macros::try_int_impl!($Struct, u32, to_u32, "BUint is too large to cast to u32");
        crate::macros::try_int_impl!($Struct, u16, to_u16, "BUint is too large to cast to u16");
        crate::macros::try_int_impl!($Struct, u8, to_u8, "BUint is too large to cast to u8");

        crate::macros::try_int_impl!($Struct, i128, to_i128, "BUint is too large to cast to i128");
        crate::macros::try_int_impl!($Struct, i64, to_i64, "BUint is too large to cast to i64");
        crate::macros::try_int_impl!($Struct, isize, to_isize, "BUint is too large to cast to isize");
        crate::macros::try_int_impl!($Struct, i32, to_i32, "BUint is too large to cast to i32");
        crate::macros::try_int_impl!($Struct, i16, to_i16, "BUint is too large to cast to i16");
        crate::macros::try_int_impl!($Struct, i8, to_i8, "BUint is too large to cast to i8");
    }
}
pub(crate) use all_try_int_impls;

macro_rules! checked_pow {
    () => {
        #[inline]
        pub const fn checked_pow(self, exp: crate::ExpType) -> Option<Self> {
            if exp == 0 {
                return Some(Self::ONE);
            }
            if self.is_zero() {
                return Some(Self::ZERO);
            }
            let mut y = Self::ONE;
            let mut n = exp;
            let mut x = self;

            macro_rules! checked_mul {
                ($var: ident) => {
                    let prod = x.checked_mul($var);
                    match prod {
                        Some(prod) => {
                            $var = prod;
                        },
                        None => {
                            return None;
                        }
                    };
                }
            }

            while n > 1 {
                if n & 1 == 0 {
                    checked_mul!(x);
                    n >>= 1;
                } else {
                    checked_mul!(y);
                    checked_mul!(x);
                    n -= 1;
                    n >>= 1;
                }
            }
            x.checked_mul(y)
        }
    }
}
pub(crate) use checked_pow;

macro_rules! overflowing_pow {
    () => {
        #[inline]
        pub const fn overflowing_pow(self, exp: crate::ExpType) -> (Self, bool) {
            if exp == 0 {
                return (Self::ONE, false);
            }
            if self.is_zero() {
                return (Self::ZERO, false);
            }
            let mut y = Self::ONE;
            let mut n = exp;
            let mut x = self;
            let mut overflow = false;
    
            macro_rules! overflowing_mul {
                ($var: ident) => {
                    let (prod, o) = x.overflowing_mul($var);
                    $var = prod;
                    overflow |= o;
                }
            }
    
            while n > 1 {
                if n & 1 == 0 {
                    overflowing_mul!(x);
                    n >>= 1;
                } else {
                    overflowing_mul!(y);
                    overflowing_mul!(x);
                    n -= 1;
                    n >>= 1;
                }
            }
            let (prod, o) = x.overflowing_mul(y);
            overflow |= o;
            (prod, overflow)
        }
    }
}
pub(crate) use overflowing_pow;

macro_rules! wrapping_pow {
    () => {
        #[inline]
        pub const fn wrapping_pow(self, exp: crate::ExpType) -> Self {
            if exp == 0 {
                return Self::ONE;
            }
            if self.is_zero() {
                return Self::ZERO;
            }
            let mut y = Self::ONE;
            let mut n = exp;
            let mut x = self;
    
            while n > 1 {
                if n & 1 == 0 {
                    x = x.wrapping_mul(x);
                    n >>= 1;
                } else {
                    y = x.wrapping_mul(y);
                    x = x.wrapping_mul(x);
                    n -= 1;
                    n >>= 1;
                }
            }
            x.wrapping_mul(y)
        }
    }
}
pub(crate) use wrapping_pow;

macro_rules! option_expect {
    ($option: expr, $msg: expr) => {
        match $option {
            Some(value) => value,
            None => panic!($msg),
        }
    }
}
pub(crate) use option_expect;

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

        crate::macros::op_ref_impl!($OpTrait<$rhs> for $Struct<N>, $op);
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

            //op_ref_impl!($tr<$rhs> for $Struct<N>, $method);
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
                    let rhs: ExpType = crate::macros::option_expect!(rhs.try_into().ok(), $err);
                    #[cfg(not(debug_assertions))]
                    let rhs = rhs as ExpType;
                    self.$method(rhs)
                }
            }

            //op_ref_impl!($tr<$rhs> for $Struct<N>, $method);
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
                let rhs: ExpType = crate::macros::option_expect!(rhs.try_into().ok(), $err);
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

        crate::macros::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", i8, i16, i32, isize, i64, i128);

        crate::macros::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", i8, i16, i32, isize, i64, i128);

        #[cfg(feature="usize_exptype")]
        crate::macros::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", u32, u64, u128);

        #[cfg(feature="usize_exptype")]
        crate::macros::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", u32, u64, u128);
        
        crate::macros::shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, u8, u16);

        crate::macros::shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, u8, u16);

        #[cfg(not(feature="usize_exptype"))]
        crate::macros::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", usize, u64, u128);

        #[cfg(not(feature="usize_exptype"))]
        crate::macros::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", usize, u64, u128);

        crate::macros::shift_self_impl!($Struct, Shl<BUint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::macros::shift_self_impl!($Struct, Shr<BUint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");

        crate::macros::shift_self_impl!($Struct, Shl<Bint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::macros::shift_self_impl!($Struct, Shr<Bint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");
    }
}

pub(crate) use all_shift_impls;

macro_rules! shift_assign_ops {
    ($OpTrait: ident, $AssignTrait: ident <$($rhs: ty), *> for $Struct: ident, $assign: ident, $op: ident) => {
        $(
            crate::macros::assign_op_impl!($OpTrait, $AssignTrait<$rhs> for $Struct, $assign, $op);
        )*
    };
}
pub(crate) use shift_assign_ops;

macro_rules! impl_ops {
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
        
        crate::macros::all_shift_impls!($Struct);
        
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

        crate::macros::assign_op_impl!(Add, AddAssign<$Struct<N>> for $Struct, add_assign, add);
        crate::macros::assign_op_impl!(BitAnd, BitAndAssign<$Struct<N>> for $Struct, bitand_assign, bitand);
        crate::macros::assign_op_impl!(BitOr, BitOrAssign<$Struct<N>> for $Struct, bitor_assign, bitor);
        crate::macros::assign_op_impl!(BitXor, BitXorAssign<$Struct<N>> for $Struct, bitxor_assign, bitxor);
        crate::macros::assign_op_impl!(Div, DivAssign<$Struct<N>> for $Struct, div_assign, div);
        crate::macros::assign_op_impl!(Mul, MulAssign<$Struct<N>> for $Struct, mul_assign, mul);
        crate::macros::assign_op_impl!(Rem, RemAssign<$Struct<N>> for $Struct, rem_assign, rem);

        crate::macros::shift_assign_ops!(Shl, ShlAssign<u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize> for $Struct, shl_assign, shl);

        crate::macros::shift_assign_ops!(Shr, ShrAssign<u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize> for $Struct, shr_assign, shr);

        crate::macros::assign_op_impl!(Sub, SubAssign<$Struct<N>> for $Struct, sub_assign, sub);
    };
}
pub(crate) use impl_ops;