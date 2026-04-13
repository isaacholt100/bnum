use num_bigint::{BigUint, BigInt, Sign};

use crate::digits::Digits;
use crate::{Integer, Int, Uint};
use crate::errors::TryFromIntError;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> From<Integer<S, N, B, OM>> for BigInt {
    #[inline]
    fn from(value: Integer<S, N, B, OM>) -> Self {
        BigInt::from_signed_bytes_le(value.as_bytes())
    }
}

impl<const N: usize, const B: usize, const OM: u8> TryFrom<Int<N, B, OM>> for BigUint {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: Int<N, B, OM>) -> Result<Self, Self::Error> {
        if value.is_negative_internal() {
            return Err(TryFromIntError(()));
        }
        Ok(BigUint::from_bytes_le(value.as_bytes()))
    }
}



impl<const N: usize, const B: usize, const OM: u8> From<Uint<N, B, OM>> for BigUint {
    #[inline]
    fn from(value: Uint<N, B, OM>) -> Self {
        BigUint::from_bytes_le(value.as_bytes())
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
        fn big_uint_convert_unsigned_invertible(value: UTest) -> bool {
            let rug_int: BigUint = value.into();
            let converted_back: UTest = rug_int.try_into().unwrap();
            
            converted_back == value
        }

        fn big_uint_convert_signed_invertible(value: ITest) -> quickcheck::TestResult {
            if value.is_negative() {
                return quickcheck::TestResult::discard();
            }
            let rug_int: BigUint = value.try_into().unwrap();
            let converted_back: ITest = rug_int.try_into().unwrap();
            
            quickcheck::TestResult::from_bool(converted_back == value)
        }

        #[cfg(feature = "alloc")]
        fn big_uint_convert_unsigned(value: UTest) -> bool {
            use alloc::string::ToString;
            let rug_int: BigUint = value.into();
            
            rug_int.to_string() == value.to_string()
        }

        #[cfg(feature = "alloc")]
        fn big_uint_convert_signed(value: ITest) -> quickcheck::TestResult {
            if value.is_negative() {
                return quickcheck::TestResult::discard();
            }
            use alloc::string::ToString;
            let rug_int: BigUint = value.try_into().unwrap();
            
            quickcheck::TestResult::from_bool(rug_int.to_string() == value.to_string())
        }
    }
}