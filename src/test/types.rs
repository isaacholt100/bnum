pub mod big_types {
    macro_rules! big_types_modules {
        ($bits: literal) => {
            pub mod u8 {
                pub type UTEST = crate::BUintD8<{ $bits / 8 }>;
                pub type ITEST = crate::BIntD8<{ $bits / 8 }>;
            }
            pub mod u16 {
                pub type UTEST = crate::BUintD16<{ $bits / 16 }>;
                pub type ITEST = crate::BIntD16<{ $bits / 16 }>;
            }
            pub mod u32 {
                pub type UTEST = crate::BUintD32<{ $bits / 32 }>;
                pub type ITEST = crate::BIntD32<{ $bits / 32 }>;
            }
            pub mod u64 {
                pub type UTEST = crate::BUint<{ $bits / 64 }>;
                pub type ITEST = crate::BInt<{ $bits / 64 }>;
            }
        };
    }

    #[cfg(test_int_bits = "16")]
    big_types_modules!(16);

    #[cfg(test_int_bits = "32")]
    big_types_modules!(32);

    #[cfg(test_int_bits = "128")]
    big_types_modules!(128);

    #[cfg(not(any(test_int_bits = "16", test_int_bits = "32", test_int_bits = "128")))]
    big_types_modules!(64);
}

#[cfg(test_int_bits = "16")]
mod small_types {
    #[allow(non_camel_case_types)]
    pub type utest = u16;

    #[allow(non_camel_case_types)]
    pub type itest = i16;

    // #[cfg(feature = "float")]
    // #[allow(non_camel_case_types)]
    // pub type ftest = f16;
}

#[cfg(test_int_bits = "32")]
mod small_types {
    #[allow(non_camel_case_types)]
    pub type utest = u32;

    #[allow(non_camel_case_types)]
    pub type itest = i32;

    // #[cfg(feature = "float")]
    // #[allow(non_camel_case_types)]
    // pub type ftest = f32;
}

#[cfg(test_int_bits = "128")]
mod small_types {
    #[allow(non_camel_case_types)]
    pub type utest = u128;

    #[allow(non_camel_case_types)]
    pub type itest = i128;

    // #[cfg(feature = "float")]
    // #[allow(non_camel_case_types)]
    // pub type ftest = f128;
}

#[cfg(not(any(test_int_bits = "16", test_int_bits = "32", test_int_bits = "128")))] // default is 64
mod small_types {
    #[allow(non_camel_case_types)]
    pub type utest = u64;

    #[allow(non_camel_case_types)]
    pub type itest = i64;

    // #[cfg(feature = "float")]
    // #[allow(non_camel_case_types)]
    // pub type ftest = f64;
}

pub use core::primitive::*;
pub use small_types::*;

// #[cfg(feature = "float")]
// #[cfg(not(test_int_bits = "32"))]
// pub type FTEST = crate::float::Float<8, 52>;

// #[cfg(feature = "float")]
// #[cfg(test_int_bits = "32")]
// pub type FTEST = crate::float::Float<4, 23>;