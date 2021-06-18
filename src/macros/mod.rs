#[macro_use]
macro_rules! div_zero {
    () => {
        panic!("attempt to divide by zero")
    };
}
pub(crate) use div_zero;

#[macro_use]
macro_rules! rem_zero {
    () => {
        panic!("attempt to calculate remainder with a divisor of zero")
    };
}
pub(crate) use rem_zero;

#[macro_use]
macro_rules! try_int_impl {
    ($Struct: tt, $int: ty, $method: ident, $err: expr) => {
        impl<const N: usize> TryFrom<$Struct<N>> for $int {
            type Error = crate::TryFromIntError;
        
            fn try_from(uint: $Struct<N>) -> Result<Self, Self::Error> {
                uint.$method().ok_or(crate::TryFromIntError {
                    from: "BUint",
                    to: stringify!($int),
                    reason: crate::error::TryFromErrorReason::TooLarge,
                })
            }
        }
    }
}
pub(crate) use try_int_impl;

#[macro_use]
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

#[macro_use]
macro_rules! checked_pow {
    () => {
        pub const fn checked_pow(self, exp: u32) -> Option<Self> {
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

#[macro_use]
macro_rules! overflowing_pow {
    () => {
        pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
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
                    if o {
                        overflow = o;
                    }
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
            if o {
                overflow = o;
            }
            (prod, overflow)
        }
    }
}
pub(crate) use overflowing_pow;

#[macro_use]
macro_rules! expect {
    ($option: expr, $msg: expr) => {
        match $option {
            Some(value) => value,
            None => panic!($msg),
        }
    }
}
pub(crate) use expect;

#[macro_use]
macro_rules! op_ref_impl {
    ($tr: tt <$rhs: ty> for $Struct: tt, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for $Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<&$rhs> for &$Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<$rhs> for &$Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: $rhs) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    }
}
pub(crate) use op_ref_impl;

#[macro_use]
macro_rules! assign_ref_impl {
    ($tr: tt <$rhs: ty> for $Struct: tt, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for $Struct<N> {
            fn $method(&mut self, rhs: &$rhs) {
                self.$method(*rhs);
            }
        }
    };
}
pub(crate) use assign_ref_impl;

#[macro_use]
macro_rules! shift_impl {
    ($Struct: tt, $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for $Struct<N> {
                type Output = Self;

                fn $method(self, rhs: $rhs) -> Self {
                    self.$method(rhs as u32)
                }
            }

            op_ref_impl!($tr<$rhs> for $Struct, $method);
        )*
    }
}
pub(crate) use shift_impl;

#[macro_use]
macro_rules! try_shift_impl {
    ($Struct: tt, $tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for $Struct<N> {
                type Output = Self;

                fn $method(self, rhs: $rhs) -> Self {
                    let rhs: u32 = expect!(rhs.try_into().ok(), $err);
                    self.$method(rhs)
                }
            }

            op_ref_impl!($tr<$rhs> for $Struct, $method);
        )*
    }
}
pub(crate) use try_shift_impl;

#[macro_use]
macro_rules! shift_self_impl {
    ($Struct: tt, $tr: tt<$rhs: tt>, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr) => {

        impl<const N: usize, const M: usize> $tr<$rhs<M>> for $Struct<N> {
            type Output = Self;
        
            fn $method(self, rhs: $rhs<M>) -> Self {
                let rhs: u32 = expect!(rhs.try_into().ok(), $err);
                self.$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for $Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<&$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: &$rhs<M>) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<$rhs<M>> for &$Struct<N> {
            type Output = $Struct<N>;
        
            fn $method(self, rhs: $rhs<M>) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    }
}
pub(crate) use shift_self_impl;

#[macro_use]
macro_rules! all_shift_impls {
    ($Struct: tt) => {
        use core::convert::TryInto;

        crate::macros::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left by negative integer", i8, i16, i32, isize, i64, i128);

        crate::macros::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right by negative integer", i8, i16, i32, isize, i64, i128);

        crate::macros::try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", usize, u64, u128);

        crate::macros::try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", usize, u64, u128);

        crate::macros::shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, u8, u16);

        crate::macros::shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, u8, u16);

        crate::macros::shift_self_impl!($Struct, Shl<BUint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::macros::shift_self_impl!($Struct, Shr<BUint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");

        crate::macros::shift_self_impl!($Struct, Shl<BIint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        crate::macros::shift_self_impl!($Struct, Shr<BIint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");
    }
}
pub(crate) use all_shift_impls;