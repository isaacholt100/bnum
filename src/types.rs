//! Type aliases for big signed and unsigned integers.

macro_rules! int_type_doc {
    ($bits: literal, $sign: literal) => {
        concat!($bits, "-bit ", $sign, " integer type.")
    };
}

macro_rules! int_types {
    { $($bits: literal $u: ident $i: ident; ) *}  => {
        $(
            #[doc = int_type_doc!($bits, "unsigned")]
            pub type $u = crate::Uint::<{ usize::div_ceil($bits, 8) }, $bits>;

            #[doc = int_type_doc!($bits, "signed")]
            pub type $i = crate::Int::<{ usize::div_ceil($bits, 8) }, $bits>;
        )*
    };
}

macro_rules! call_types_macro {
    ($name: ident) => {
        $name! {
            128 U128 I128;
            256 U256 I256;
            512 U512 I512;
            1024 U1024 I1024;
            2048 U2048 I2048;
            4096 U4096 I4096;
            8192 U8192 I8192;
        }
    };
}

call_types_macro!(int_types);

// #[cfg(feature = "float")]
// /// 16-bit floating point type with 10 bits of precision, stored as the binary16 (half precision) format defined in IEEE 754-2019.
// pub type F16 = crate::Float<2, 10>;

// #[cfg(feature = "float")]
// /// 32-bit floating point type with 23 bits of precision, stored as the binary32 (single precision) format defined in IEEE 754-2019.
// pub type F32 = crate::Float<4, 23>;

// #[cfg(feature = "float")]
// /// 64-bit floating point type with 52 bits of precision, stored as the binary64 (double precision) format defined in IEEE 754-2019.
// pub type F64 = crate::Float<8, 52>;

// #[cfg(feature = "float")]
// /// 80-bit floating point type with 64 bits of precision.
// pub type F80 = crate::Float<10, 64>;

// #[cfg(feature = "float")]
// /// 128-bit floating point type with 112 bits of precision, stored as the binary128 (quadruple precision) format defined in IEEE 754-2019.
// pub type F128 = crate::Float<16, 112>;

// #[cfg(feature = "float")]
// /// 256-bit floating point type with 236 bits of precision, stored as the binary256 (octuple precision) format defined in IEEE 754-2019.
// pub type F256 = crate::Float<32, 236>;


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_int_bits {
        { $($bits: literal $u: ident $i: ident; ) *} => {
            $(
                assert_eq!($u::BITS, $bits);
                assert_eq!($i::BITS, $bits);
            )*
        }
    }

    #[test]
    fn test_int_bits() {
        call_types_macro!(assert_int_bits);
    }
}
