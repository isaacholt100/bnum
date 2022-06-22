macro_rules! test_type {
	($prefix: ident, $suffix: ident, $bits: literal) => {
		paste::paste! {
			#[allow(non_camel_case_types, unused)]
			pub type [<$prefix $suffix>] = [<$prefix $bits>];
	
			#[allow(unused)]
			pub type [<$prefix:upper $suffix:upper>] = crate::test::types::[<$prefix:upper $bits>];

			// TODO: replace all tests with utest/itest
		}
	};
}

macro_rules! big_type {
	($($bits: literal), *) => {
		paste::paste! {
			$(
				#[allow(unused)]
				pub type [<U $bits>] = crate::BUint::<{$bits / (crate::digit::BITS as usize)}>;
				#[allow(unused)]
				pub type [<I $bits>] = crate::BInt::<{$bits / (crate::digit::BITS as usize)}>;
			)*
		}
	};
}

macro_rules! test_types {
	($suffix: ident, $bits: literal) => {
		paste::paste! {
			test_type!(u, $suffix, $bits);
			test_type!(i, $suffix, $bits);
		}

		pub use core::primitive::*;

		big_type!(64, 128);

		#[cfg(feature = "u8_digit")]
		big_type!(8, 16, 32);
	};
}

test_types!(test, 128);