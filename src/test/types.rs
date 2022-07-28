macro_rules! test_type {
    ($prefix: ident, $suffix: ident, $bits: literal, $attr: meta) => {
        paste::paste! {
            #[allow(non_camel_case_types)]
            #[$attr]
            pub type [<$prefix $suffix>] = [<$prefix $bits>];

            /*#[$attr]
            pub type [<$prefix:upper $suffix:upper>] = crate::test::types::[<$prefix:upper $bits>];*/
        }
    };
}

macro_rules! big_type {
	($($bits: literal), *) => {
		paste::paste! {
			$(
				// TODO: change
				pub type [<U $bits>] = crate::BUint::<{$bits / crate::BUint::<0>::DIGIT_BITS}>;

				pub type [<I $bits>] = crate::BInt::<{$bits / crate::BUint::<0>::DIGIT_BITS}>;
			)*
		}
	};
}

macro_rules! test_type_signs {
    ($suffix: ident, $bits: literal, $attr: meta) => {
        test_type!(u, $suffix, $bits, $attr);
        test_type!(i, $suffix, $bits, $attr);
    };
}

macro_rules! test_types_bits {
	($suffix: ident, $($bits: literal), *) => {
		paste::paste! {
			$(
				test_type_signs!($suffix, $bits, cfg(test_int_bits = $bits));
			)*
		}
	};
}

macro_rules! test_types {
    ($suffix: ident) => {
        test_types_bits!($suffix, "8", "16", "32", "64", "128");

        test_type_signs!(
            $suffix,
            "128",
            cfg(not(any(
                test_int_bits = "8",
                test_int_bits = "16",
                test_int_bits = "32",
                test_int_bits = "64",
                test_int_bits = "128"
            )))
        );

        pub use core::primitive::*;

        big_type!(64, 128);

        #[cfg(feature = "u8_digit")]
        big_type!(8, 16, 32);
    };
}

test_types!(test);

pub mod big_types {
    pub mod u8 {
        pub type UTEST = crate::BUintU8D<16>;
        pub type ITEST = crate::BIntU8D<16>;
    }
    pub mod u16 {
        pub type UTEST = crate::BUintU16D<8>;
        pub type ITEST = crate::BIntU16D<8>;
    }
    pub mod u32 {
        pub type UTEST = crate::BUintU32D<4>;
        pub type ITEST = crate::BIntU32D<4>;
    }
    pub mod u64 {
        pub type UTEST = crate::BUint<2>;
        pub type ITEST = crate::BInt<2>;
    }
}
