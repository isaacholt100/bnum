use num_bigint::{BigUint, BigInt, Sign};

use crate::digits::Digits;
use crate::Integer;
use crate::errors::TryFromIntError;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> From<Integer<S, N, B, OM>> for BigInt {
    #[inline]
    fn from(value: Integer<S, N, B, OM>) -> Self {
        BigInt::from_signed_bytes_le(value.as_bytes())
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<Integer<S, N, B, OM>> for BigUint {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: Integer<S, N, B, OM>) -> Result<Self, Self::Error> {
        if value.is_negative_internal() {
            return Err(TryFromIntError(()));
        }
        Ok(BigUint::from_bytes_le(value.as_bytes()))
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<&BigUint> for Integer<S, N, B, OM> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: &BigUint) -> Result<Self, Self::Error> {
        if !S && value.bits() > Self::BITS as u64 {
            return Err(TryFromIntError(()));
        }
        if S && value.bits() > Self::BITS as u64 - 1 {
            return Err(TryFromIntError(()));
        }
        // now guaranteed to succeed
        let mut digits = Digits::<u64, N>::ALL_ZEROS;
        for (i, u64_digit) in value.iter_u64_digits().enumerate() {
            digits.set(i, u64_digit);
        }
        Ok(digits.to_integer())
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<BigUint> for Integer<S, N, B, OM> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: BigUint) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

// impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<&BigInt> for Integer<S, N, B, OM> {
//     type Error = TryFromIntError;

//     #[inline]
//     fn try_from(value: &BigInt) -> Result<Self, Self::Error> {
//         if !S && value.sign() == Sign::Minus {
//             return Err(TryFromIntError(()));
//         }
        

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;


    quickcheck::quickcheck! {
        
    }
}