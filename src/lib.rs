#![feature(
    const_generics,
    const_evaluatable_checked,
    const_raw_ptr_deref,
    //#![feature(const_trait_impl,
    const_panic,
    const_fn,
    const_option,
    const_maybe_uninit_assume_init,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_ptr,
    const_ptr_offset,
    test
)]

#[allow(unused)]
fn test_into_converter<U, T: Into<U>>(x: T) -> U {
    x.into()
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

            impl<const N: usize> $assign_tr<$rhs> for $Struct<N> {
                fn $assign_method(&mut self, rhs: $rhs) {
                    *self = self.$method(rhs);
                }
            }

            op_ref_impl!($tr<$rhs> for $Struct, $method);

            assign_ref_impl!($assign_tr<$rhs> for $Struct, $assign_method);
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

            impl<const N: usize> $assign_tr<$rhs> for $Struct<N> {
                fn $assign_method(&mut self, rhs: $rhs) {
                    *self = self.$method(rhs);
                }
            }

            op_ref_impl!($tr<$rhs> for $Struct, $method);

            assign_ref_impl!($assign_tr<$rhs> for $Struct, $assign_method);
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

        impl<const N: usize, const M: usize> $assign_tr<$rhs<M>> for $Struct<N> {
            fn $assign_method(&mut self, rhs: $rhs<M>) {
                *self = self.$method(rhs);
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<&$rhs<M>> for $Struct<N> {
            fn $assign_method(&mut self, rhs: &$rhs<M>) {
                *self = self.$method(*rhs);
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
        big: $big_type: tt,
        primitive: $primitive: tt,
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
        big: $big_type: tt,
        primitive: $primitive: tt,
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
                    $($arg.into()),*
                );
                let big_result = <$big_type>::$method(
                    $($arg.into()), *
                );
                assert_eq!(big_result, $converter(prim_result));
            )*
        }
    }
}

pub mod uint;
mod int;
mod iint;
mod tryops;
mod sign;
mod main;
mod arch;
mod digit;
//mod bound;
mod int_test;
mod benchmarks;

pub use iint::BIint;
pub use sign::Sign;
pub use uint::BUint;
pub use int::Bint;
pub use int_test::BintTest;

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

pub type ParseIntError = &'static str;
pub type TryFromIntError = &'static str;
pub type OperationError = &'static str;