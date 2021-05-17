use super::BUint;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

macro_rules! op_ref_impl {
    ($tr: tt <$rhs: ty>, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<&$rhs> for &BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<$rhs> for &BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: $rhs) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    }
}

macro_rules! assign_ref_impl {
    ($tr: tt <$rhs: ty>, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for BUint<N> {
            fn $method(&mut self, rhs: &$rhs) {
                self.$method(*rhs);
            }
        }
    };
}

impl<const N: usize> BUint<N> {
    pub const fn add(self, rhs: Self) -> Self {
        expect!(self.checked_add(rhs), "attempt to add with overflow")
    }
}

impl<const N: usize> Add<Self> for BUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

op_ref_impl!(Add<BUint<N>>, add);

impl<const N: usize> AddAssign for BUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

assign_ref_impl!(AddAssign<BUint<N>>, add_assign);

impl<const N: usize> BitAnd for BUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a & b
        })
    }
}

op_ref_impl!(BitAnd<BUint<N>>, bitand);

impl<const N: usize> BitAndAssign for BUint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

assign_ref_impl!(BitAndAssign<BUint<N>>, bitand_assign);

impl<const N: usize> BitOr for BUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a | b
        })
    }
}

op_ref_impl!(BitOr<BUint<N>>, bitor);

impl<const N: usize> BitOrAssign for BUint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

assign_ref_impl!(BitOrAssign<BUint<N>>, bitor_assign);

impl<const N: usize> BitXor for BUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a ^ b
        })
    }
}

op_ref_impl!(BitXor<BUint<N>>, bitxor);

impl<const N: usize> BitXorAssign for BUint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

assign_ref_impl!(BitXorAssign<BUint<N>>, bitxor_assign);

impl<const N: usize> Div for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

op_ref_impl!(Div<BUint<N>>, div);

impl<const N: usize> DivAssign for BUint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

assign_ref_impl!(DivAssign<BUint<N>>, div_assign);

impl<const N: usize> Mul for BUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        expect!(self.checked_mul(rhs), "attempt to multiply with overflow")
    }
}

op_ref_impl!(Mul<BUint<N>>, mul);

impl<const N: usize> MulAssign for BUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

assign_ref_impl!(MulAssign<BUint<N>>, mul_assign);

impl<const N: usize> BUint<N> {
    pub const fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = !self.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> Not for BUint<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Not for &BUint<N> {
    type Output = BUint<N>;

    fn not(self) -> BUint<N> {
        (*self).not()
    }
}

impl<const N: usize> Rem for BUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}

op_ref_impl!(Rem<BUint<N>>, rem);

impl<const N: usize> RemAssign for BUint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

assign_ref_impl!(RemAssign<BUint<N>>, rem_assign);

impl<const N: usize> Shl<u32> for BUint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        expect!(self.checked_shl(rhs), "attempt to shift left with overflow")
    }
}

op_ref_impl!(Shl<u32>, shl);

impl<const N: usize> ShlAssign<u32> for BUint<N> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

assign_ref_impl!(ShlAssign<u32>, shl_assign);

impl<const N: usize> Shr<u32> for BUint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        expect!(self.checked_shr(rhs), "attempt to shift right with overflow")
    }
}

op_ref_impl!(Shr<u32>, shr);

impl<const N: usize> ShrAssign<u32> for BUint<N> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

assign_ref_impl!(ShrAssign<u32>, shr_assign);

macro_rules! shift_impl {
    ($tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for BUint<N> {
                type Output = Self;

                fn $method(self, rhs: $rhs) -> Self {
                    self.$method(rhs as u32)
                }
            }

            impl<const N: usize> $assign_tr<$rhs> for BUint<N> {
                fn $assign_method(&mut self, rhs: $rhs) {
                    *self = self.$method(rhs);
                }
            }

            op_ref_impl!($tr<$rhs>, $method);

            assign_ref_impl!($assign_tr<$rhs>, $assign_method);
        )*
    }
}

shift_impl!(Shl, shl, ShlAssign, shl_assign, u8, u16);

shift_impl!(Shr, shr, ShrAssign, shr_assign, u8, u16);

use std::convert::TryInto;

macro_rules! try_shift_impl {
    ($tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr, $($rhs: ty), *) => {
        $(
            impl<const N: usize> $tr<$rhs> for BUint<N> {
                type Output = Self;

                fn $method(self, rhs: $rhs) -> Self {
                    let rhs: u32 = expect!(rhs.try_into().ok(), $err);
                    self.$method(rhs)
                }
            }

            impl<const N: usize> $assign_tr<$rhs> for BUint<N> {
                fn $assign_method(&mut self, rhs: $rhs) {
                    *self = self.$method(rhs);
                }
            }

            op_ref_impl!($tr<$rhs>, $method);

            assign_ref_impl!($assign_tr<$rhs>, $assign_method);
        )*
    }
}

try_shift_impl!(Shl, shl, ShlAssign, shl_assign, "attempt to shift left by negative integer", i8, i16, i32, isize, i64, i128);

try_shift_impl!(Shr, shr, ShrAssign, shr_assign, "attempt to shift right by negative integer", i8, i16, i32, isize, i64, i128);

try_shift_impl!(Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow", usize, u64, u128);

try_shift_impl!(Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow", usize, u64, u128);

macro_rules! shift_self_impl {
    ($tr: tt, $method: ident, $assign_tr: tt, $assign_method: ident, $err: expr) => {

        impl<const N: usize, const M: usize> $tr<BUint<M>> for BUint<N> {
            type Output = Self;
        
            fn $method(self, rhs: BUint<M>) -> Self {
                let rhs: u32 = expect!(rhs.try_into().ok(), $err);
                self.$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $tr<&BUint<M>> for BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: &BUint<M>) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<&BUint<M>> for &BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: &BUint<M>) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize, const M: usize> $tr<BUint<M>> for &BUint<N> {
            type Output = BUint<N>;
        
            fn $method(self, rhs: BUint<M>) -> Self::Output {
                (*self).$method(rhs)
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<BUint<M>> for BUint<N> {
            fn $assign_method(&mut self, rhs: BUint<M>) {
                *self = self.$method(rhs);
            }
        }

        impl<const N: usize, const M: usize> $assign_tr<&BUint<M>> for BUint<N> {
            fn $assign_method(&mut self, rhs: &BUint<M>) {
                *self = self.$method(*rhs);
            }
        }
    }
}

shift_self_impl!(Shl, shl, ShlAssign, shl_assign, "attempt to shift left with overflow");

shift_self_impl!(Shr, shr, ShrAssign, shr_assign, "attempt to shift right with overflow");

impl<const N: usize> Sub for BUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        expect!(self.checked_sub(rhs), "attempt to subtract with overflow")
    }
}

op_ref_impl!(Sub<BUint<N>>, sub);

impl<const N: usize> SubAssign for BUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

assign_ref_impl!(SubAssign<BUint<N>>, sub_assign);