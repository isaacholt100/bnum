macro_rules! test_types {
    ($bits: literal) => {
        paste::paste! {
            mod types {
                #[allow(non_camel_case_types)]
                pub type utest = [<u $bits>];

                #[allow(non_camel_case_types)]
                pub type itest = [<i $bits>];

                #[allow(non_camel_case_types)]
                pub type UTEST = crate::BUintD8<{ $bits / 8 }>;

                #[allow(non_camel_case_types)]
                pub type ITEST = crate::BIntD8<{ $bits / 8 }>;
            }
        }
    };
}

#[cfg(test_int_bits = "16")]
test_types!(16);

#[cfg(test_int_bits = "32")]
test_types!(32);

#[cfg(test_int_bits = "128")]
test_types!(128);

#[cfg(not(any(test_int_bits = "16", test_int_bits = "32", test_int_bits = "128")))]
test_types!(64);

pub use core::primitive::*;
pub use types::*;

#[cfg(feature = "float")]
#[cfg(not(test_int_bits = "32"))]
pub type FTEST = crate::float::Float<8, 52>;

#[cfg(feature = "float")]
#[cfg(test_int_bits = "32")]
pub type FTEST = crate::float::Float<4, 23>;
