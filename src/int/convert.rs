use crate::{cast::CastFrom, errors::TryFromIntError, Uint, ExpType};

pub trait IntConvertHelper {
    const BITS: ExpType;

    fn leading_zeros_at_least_threshold(&self, threshold: ExpType) -> bool;
}

pub trait SignedIntConvertHelper: IntConvertHelper {
    fn is_negative(&self) -> bool;
    #[allow(unused)]
    fn leading_ones_at_least_threshold(&self, threshold: ExpType) -> bool;
}

impl<const N: usize> IntConvertHelper for Uint<N> {
    const BITS: ExpType = Self::BITS;

    #[inline]
    fn leading_zeros_at_least_threshold(&self, threshold: ExpType) -> bool {
        Self::leading_zeros_at_least_threshold(&self, threshold)
    }
}

#[cfg(feature = "signed")]
impl<const N: usize> IntConvertHelper for crate::Int<N> {
    const BITS: ExpType = Self::BITS;

    #[inline]
    fn leading_zeros_at_least_threshold(&self, threshold: ExpType) -> bool {
        Uint::leading_zeros_at_least_threshold(&self.bits, threshold)
    }
}

#[cfg(feature = "signed")]
impl<const N: usize> SignedIntConvertHelper for crate::Int<N> {
    #[inline]
    fn is_negative(&self) -> bool {
        use crate::digit::SignedDigit;

        SignedDigit::is_negative(self.bits.digits[N - 1] as SignedDigit)
    }

    #[inline]
    fn leading_ones_at_least_threshold(&self, threshold: ExpType) -> bool {
        self.bits.leading_ones_at_least_threshold(threshold)
    }
}

macro_rules! impl_int_convert_helper_for_primitive_int {
    ($($int: ty), *) => {
        $(
            impl IntConvertHelper for $int {
                const BITS: ExpType = Self::BITS as ExpType;

                #[inline]
                fn leading_zeros_at_least_threshold(&self, threshold: ExpType) -> bool {
                    self.leading_zeros() >= threshold
                }
            }
        )*
    };
}

impl_int_convert_helper_for_primitive_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! impl_signed_int_convert_helper_for_primitive_int {
    ($($int: ty), *) => {
        $(
            impl SignedIntConvertHelper for $int {
                #[inline]
                fn is_negative(&self) -> bool {
                    Self::is_negative(*self)
                }

                #[inline]
                fn leading_ones_at_least_threshold(&self, threshold: ExpType) -> bool {
                    self.leading_ones() >= threshold
                }
            }
        )*
    };
}

impl_signed_int_convert_helper_for_primitive_int!(i8, i16, i32, i64, i128, isize);

// Conversion functions

#[inline]
pub fn uint_try_from_uint<U, V>(uint: U) -> Result<V, TryFromIntError>
where
    V: CastFrom<U>,
    U: IntConvertHelper,
    V: IntConvertHelper,
{
    if U::BITS <= V::BITS || uint.leading_zeros_at_least_threshold(U::BITS - V::BITS + 1) {
        Ok(V::cast_from(uint))
    } else {
        Err(TryFromIntError(()))
    }
}

#[inline]
pub fn uint_try_from_int<I, U>(int: I) -> Result<U, TryFromIntError>
where
    U: CastFrom<I>,
    I: SignedIntConvertHelper,
    U: IntConvertHelper,
{
    if int.is_negative() {
        return Err(TryFromIntError(()));
    }
    if I::BITS - 1 <= U::BITS || int.leading_zeros_at_least_threshold(I::BITS - U::BITS) {
        Ok(U::cast_from(int))
    } else {
        Err(TryFromIntError(()))
    }
}

#[inline]
pub fn int_try_from_uint<U, I>(uint: U) -> Result<I, TryFromIntError>
where
    I: CastFrom<U>,
    U: IntConvertHelper,
    I: IntConvertHelper,
{
    if U::BITS <= I::BITS - 1 || uint.leading_zeros_at_least_threshold(U::BITS - I::BITS + 1) {
        Ok(I::cast_from(uint))
    } else {
        Err(TryFromIntError(()))
    }
}

#[cfg(feature = "signed")]
#[inline]
pub fn int_try_from_int<I, J>(int: I) -> Result<J, TryFromIntError>
where
    J: CastFrom<I>,
    I: SignedIntConvertHelper,
    J: IntConvertHelper,
{
    if I::BITS <= J::BITS {
        return Ok(J::cast_from(int));
    }
    if int.is_negative() {
        if int.leading_ones_at_least_threshold(I::BITS - J::BITS + 1) {
            Ok(J::cast_from(int))
        } else {
            Err(TryFromIntError(()))
        }
    } else {
        if int.leading_zeros_at_least_threshold(I::BITS - J::BITS + 1) {
            Ok(J::cast_from(int))
        } else {
            Err(TryFromIntError(()))
        }
    }
}
