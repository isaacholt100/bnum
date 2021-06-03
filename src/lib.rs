#![feature(
    const_generics,
    const_evaluatable_checked,
    const_panic,
    const_maybe_uninit_assume_init,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_ptr,
    const_ptr_offset,
    test
)]
#![no_std]

#[macro_use]
extern crate alloc;

#[allow(unused)]
fn test_into_converter<U, T: Into<U>>(x: T) -> U {
    x.into()
}

macro_rules! div_zero {
    () => {
        panic!("attempt to divide by zero")
    };
}

macro_rules! rem_zero {
    () => {
        panic!("attempt to calculate remainder with a divisor of zero")
    };
}

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

macro_rules! all_try_int_impls {
    ($Struct: tt) => {
        try_int_impl!($Struct, u128, to_u128, "BUint is too large to cast to u128");
        try_int_impl!($Struct, u64, to_u64, "BUint is too large to cast to u64");
        try_int_impl!($Struct, usize, to_usize, "BUint is too large to cast to usize");
        try_int_impl!($Struct, u32, to_u32, "BUint is too large to cast to u32");
        try_int_impl!($Struct, u16, to_u16, "BUint is too large to cast to u16");
        try_int_impl!($Struct, u8, to_u8, "BUint is too large to cast to u8");

        try_int_impl!($Struct, i128, to_i128, "BUint is too large to cast to i128");
        try_int_impl!($Struct, i64, to_i64, "BUint is too large to cast to i64");
        try_int_impl!($Struct, isize, to_isize, "BUint is too large to cast to isize");
        try_int_impl!($Struct, i32, to_i32, "BUint is too large to cast to i32");
        try_int_impl!($Struct, i16, to_i16, "BUint is too large to cast to i16");
        try_int_impl!($Struct, i8, to_i8, "BUint is too large to cast to i8");
    }
}

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

macro_rules! expect {
    ($option: expr, $msg: expr) => {
        match $option {
            Some(value) => value,
            None => panic!($msg),
        }
    }
}

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

macro_rules! assign_ref_impl {
    ($tr: tt <$rhs: ty> for $Struct: tt, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for $Struct<N> {
            fn $method(&mut self, rhs: &$rhs) {
                self.$method(*rhs);
            }
        }
    };
}

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

macro_rules! all_shift_impls {
    ($Struct: tt) => {
        use core::convert::TryInto;

        try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left by negative integer", i8, i16, i32, isize, i64, i128);

        try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right by negative integer", i8, i16, i32, isize, i64, i128);

        try_shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", usize, u64, u128);

        try_shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", usize, u64, u128);

        shift_impl!($Struct, Shl, shl, ShlAssign, shl_assign, u8, u16);

        shift_impl!($Struct, Shr, shr, ShrAssign, shr_assign, u8, u16);

        shift_self_impl!($Struct, Shl<BUint>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        shift_self_impl!($Struct, Shr<BUint>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");

        shift_self_impl!($Struct, Shl<BintTest>, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

        shift_self_impl!($Struct, Shr<BintTest>, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");
    }
}

#[allow(unused)]
macro_rules! test {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: $big_type,
            primitive: $primitive,
            test_name: $test_name,
            method: {
                $($method ($($arg), *) ;) *
            },
            converter: crate::test_into_converter
        }
    };
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
        converter: $converter: expr
    } => {
        #[test]
        fn $test_name() {
            $(
                let prim_result = <$primitive>::$method(
                    $($arg), *
                );
                let big_result = <$big_type>::$method(
                    $($arg.into()), *
                );
                assert_eq!(big_result, $converter(prim_result));
            )*
        }
    }
}

mod uint;
mod int;
mod tryops;
mod sign;
mod main;
mod arch;
mod digit;
//mod bound;
mod int_test;
mod benchmarks;
mod error;

pub use sign::Sign;
pub use uint::BUint;
pub use int::Bint;
pub use int_test::BintTest;
pub use error::*;

#[allow(unused)]
type I128 = int::Bint::<{(128 / digit::BITS) - 1}>;

type I128Test = int_test::BintTest::<2>;

#[allow(unused)]
pub type U128 = BUint::<{128 / digit::BITS}>;

pub type U256 = BUint::<{256 / digit::BITS}>;
pub type U512 = BUint::<{512 / digit::BITS}>;
pub type U1024 = BUint::<{1024 / digit::BITS}>;
pub type U2048 = BUint::<{2048 / digit::BITS}>;
pub type U4096 = BUint::<{4096 / digit::BITS}>;
pub type U8192 = BUint::<{8192 / digit::BITS}>;