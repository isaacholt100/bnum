use super::Bint;
use crate::uint::BUint;
use crate::cast::{CastFrom, As};
use crate::digit;

macro_rules! bint_as {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> const CastFrom<Bint<N>> for $int {
                #[inline]
                fn cast_from(from: Bint<N>) -> Self {
                    if from.is_negative() {
                        let digits = from.digits();
                        let mut out = !0;
                        let mut i = 0;
                        while i << digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
                            out &= !((!digits[i]) as $int << (i << digit::BIT_SHIFT));
                            i += 1;
                        }
                        out
                    } else {
                        <$int>::cast_from(from.bits)
                    }
                }
            }
        )*
    };
}

bint_as!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> CastFrom<Bint<N>> for f32 {
    #[inline]
    fn cast_from(from: Bint<N>) -> Self {
        let f: f32 = from.unsigned_abs().as_();
        if from.is_negative() {
            -f
        } else {
            f
        }
    }
}

impl<const N: usize> CastFrom<Bint<N>> for f64 {
    #[inline]
    fn cast_from(from: Bint<N>) -> Self {
        let f: f64 = from.unsigned_abs().as_();
        if from.is_negative() {
            -f
        } else {
            f
        }
    }
}

macro_rules! as_bint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> const CastFrom<$ty> for Bint<N> {
                #[inline]
                fn cast_from(from: $ty) -> Self {
                    Self::from_bits(BUint::cast_from(from))
                }
            }
        )*
    }
}

as_bint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char);

impl<const N: usize, const M: usize> const CastFrom<BUint<M>> for Bint<N> {
    #[inline]
    fn cast_from(from: BUint<M>) -> Self {
        Self::from_bits(BUint::cast_from(from))
    }
}

impl<const N: usize, const M: usize> const CastFrom<Bint<M>> for Bint<N> {
    #[inline]
    fn cast_from(from: Bint<M>) -> Self {
        Self::from_bits(BUint::cast_from(from))
    }
}

macro_rules! cast_from_float {
    ($f: ty) => {
        #[inline]
        fn cast_from(from: $f) -> Self {
            if from.is_sign_negative() {
                let u = BUint::<N>::cast_from(-from);
                if u > Self::MIN.to_bits() {
                    Self::MIN
                } else {
                    -Self::from_bits(u)
                }
            } else {
                let u = BUint::<N>::cast_from(from);
                let i = Self::from_bits(u);
                if i.is_negative() {
                    Self::MAX
                } else {
                    i
                }
            }
        }
    };
}

impl<const N: usize> CastFrom<f32> for Bint<N> {
    cast_from_float!(f32);
}

impl<const N: usize> CastFrom<f64> for Bint<N> {
    cast_from_float!(f64);
}

#[cfg(test)]
mod tests {
    use crate::types::{I128, U128, I64, U64};
	use crate::test;
    use crate::cast::{As, CastFrom};
    
    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char] as i64);

    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char] as i128);

    test::test_cast_from!(i64 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, U64, I64, I128, U128]);

    test::test_cast_from!(i128 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, U64, I64, I128, U128]);
}