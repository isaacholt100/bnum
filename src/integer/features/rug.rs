use rug::Integer as RugInteger;
use rug::integer::Order;
use crate::{Int, Integer, Uint, errors::TryFromIntError};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> From<Integer<S, N, B, OM>> for RugInteger {
    #[inline]
    fn from(value: Integer<S, N, B, OM>) -> Self {
        let bytes = value.unsigned_abs_internal().to_bytes();
        let abs_value = RugInteger::from_digits(&bytes, Order::Lsf);
        if value.is_negative_internal() {
            -abs_value
        } else {
            abs_value
        }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<&RugInteger> for Integer<S, N, B, OM> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: &RugInteger) -> Result<Self, Self::Error> {
        if !S && value.is_negative() {
            return Err(TryFromIntError(()));
        }
        if S && value.signed_bits() > Self::BITS {
            return Err(TryFromIntError(()));
        }
        if !S && value.significant_bits() > Self::BITS {
            return Err(TryFromIntError(()));
        }
        let mut out = Self::ZERO;
        value.write_digits(&mut out.bytes, Order::Lsf);

        if value.is_negative() {
            out.set_sign_bits();
            out = out.wrapping_neg();
        }
        Ok(out)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<RugInteger> for Integer<S, N, B, OM> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(value: RugInteger) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    quickcheck::quickcheck! {
        fn rug_integer_convert_unsigned_invertible(value: UTEST) -> bool {
            let rug_int: RugInteger = value.into();
            let converted_back: UTEST = rug_int.try_into().unwrap();
            
            converted_back == value
        }

        fn rug_integer_convert_signed_invertible(value: ITEST) -> bool {
            let rug_int: RugInteger = value.into();
            let converted_back: ITEST = rug_int.try_into().unwrap();
            
            converted_back == value
        }

        #[cfg(feature = "alloc")]
        fn rug_integer_convert_unsigned(value: UTEST) -> bool {
            use alloc::string::ToString;
            let rug_int: RugInteger = value.into();
            
            rug_int.to_string() == value.to_string()
        }

        #[cfg(feature = "alloc")]
        fn rug_integer_convert_signed(value: ITEST) -> bool {
            use alloc::string::ToString;
            let rug_int: RugInteger = value.into();
            
            rug_int.to_string() == value.to_string()
        }
    }
}