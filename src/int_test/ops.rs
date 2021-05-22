use super::BintTest;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

macro_rules! op_ref_impl {
    ($tr: tt <$rhs: ty>, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for BintTest<N> {
            type Output = BintTest<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                self.$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<&$rhs> for &BintTest<N> {
            type Output = BintTest<N>;
        
            fn $method(self, rhs: &$rhs) -> Self::Output {
                (*self).$method(*rhs)
            }
        }
        
        impl<const N: usize> $tr<$rhs> for &BintTest<N> {
            type Output = BintTest<N>;
        
            fn $method(self, rhs: $rhs) -> Self::Output {
                (*self).$method(rhs)
            }
        }
    }
}

macro_rules! assign_ref_impl {
    ($tr: tt <$rhs: ty>, $method: ident) => {
        impl<const N: usize> $tr<&$rhs> for BintTest<N> {
            fn $method(&mut self, rhs: &$rhs) {
                *self = *self + *rhs;
            }
        }
    };
}

impl<const N: usize> BintTest<N> {
    pub const fn add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(int) => int,
            None => panic!("attempt to add with overflow"),
        }
    }
}

impl<const N: usize> Add<Self> for BintTest<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

op_ref_impl!(Add<BintTest<N>>, add);

impl<const N: usize> AddAssign for BintTest<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

assign_ref_impl!(AddAssign<BintTest<N>>, add_assign);

impl<const N: usize> BitAnd for BintTest<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            uint: self.uint & rhs.uint,
        }
    }
}

op_ref_impl!(BitAnd<BintTest<N>>, bitand);

impl<const N: usize> BitAndAssign for BintTest<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

assign_ref_impl!(BitAndAssign<BintTest<N>>, bitand_assign);

impl<const N: usize> BitOr for BintTest<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            uint: self.uint | rhs.uint,
        }
    }
}

op_ref_impl!(BitOr<BintTest<N>>, bitor);

impl<const N: usize> BitOrAssign for BintTest<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

assign_ref_impl!(BitOrAssign<BintTest<N>>, bitor_assign);

impl<const N: usize> BitXor for BintTest<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            uint: self.uint ^ rhs.uint,
        }
    }
}

op_ref_impl!(BitXor<BintTest<N>>, bitxor);

impl<const N: usize> BitXorAssign for BintTest<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

assign_ref_impl!(BitXorAssign<BintTest<N>>, bitxor_assign);

impl<const N: usize> Div for BintTest<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.checked_div(rhs).unwrap()
    }
}

op_ref_impl!(Div<BintTest<N>>, div);

impl<const N: usize> DivAssign for BintTest<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

assign_ref_impl!(DivAssign<BintTest<N>>, div_assign);

impl<const N: usize> Mul for BintTest<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).unwrap()
    }
}

op_ref_impl!(Mul<BintTest<N>>, mul);

impl<const N: usize> MulAssign for BintTest<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

assign_ref_impl!(MulAssign<BintTest<N>>, mul_assign);

impl<const N: usize> BintTest<N> {
    pub const fn not(self) -> Self {
        Self {
            uint: self.uint.not(),
        }
    }
}

impl<const N: usize> Not for BintTest<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Not for &BintTest<N> {
    type Output = BintTest<N>;

    fn not(self) -> BintTest<N> {
        (*self).not()
    }
}

impl<const N: usize> Neg for BintTest<N> {
    type Output = Self;

    fn neg(self) -> Self {
        self.checked_neg().expect("attempt to negative with overflow")
    }
}

impl<const N: usize> Neg for &BintTest<N> {
    type Output = BintTest<N>;

    fn neg(self) -> BintTest<N> {
        (*self).not()
    }
}

impl<const N: usize> Rem for BintTest<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.checked_rem(rhs).unwrap()
    }
}

op_ref_impl!(Rem<BintTest<N>>, rem);

impl<const N: usize> RemAssign for BintTest<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

assign_ref_impl!(RemAssign<BintTest<N>>, rem_assign);

impl<const N: usize> Shl<u32> for BintTest<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        self.checked_shl(rhs).unwrap()
    }
}

impl<const N: usize> ShlAssign<u32> for BintTest<N> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

impl<const N: usize> Shr<u32> for BintTest<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        self.checked_shr(rhs).unwrap()
    }
}

// TODO: implement all shr and shl

/*impl<const N: usize> Shr<u128> for BintTest<N> {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self {
        if rhs > (N << 6) as u128 {
            panic!("Underflow");
        }
        let shift_index = rhs >> 6;
        let small_shift = rhs & (u64::MAX as u128);
        
        self.try_shr(rhs).unwrap()
    }
}*/

impl<const N: usize> ShrAssign<u32> for BintTest<N> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

impl<const N: usize> Sub for BintTest<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).expect("attempt to subtract with overflow")
    }
}

op_ref_impl!(Sub<BintTest<N>>, sub);

impl<const N: usize> SubAssign for BintTest<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

assign_ref_impl!(SubAssign<BintTest<N>>, sub_assign);