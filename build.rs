// Build script that creates type aliases for use in testing. `utest`, `UTEST`, `itest`, `ITEST` (upper case because of the paste crate's `$var:upper` conversion tool) are type aliases which can be controlled by an environment variable. This allows testing of different sized integers against the primitives. The core primitives are re-exported for convenience.

use std::fs::File;
use std::io::prelude::*;
use std::io;

const TEST_TYPE_SUFFIX: &str = "test";
const FILE_PATH: &str = "src/test/types.rs";
const FILE_TEMPLATE: &str = "macro_rules! test_type {
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

		#[cfg(feature = \"u8_digit\")]
		big_type!(8, 16, 32);
	};
}

";

#[inline]
fn test_type_bits() -> &'static str {
	option_env!("BNUM_TEST_INT_BITS").unwrap_or("128")
}

#[inline]
fn create_test_types() -> io::Result<()> {
	let mut file = File::create(FILE_PATH)?;
	write!(file, "{}test_types!({}, {});", FILE_TEMPLATE, TEST_TYPE_SUFFIX, test_type_bits())
}

fn main() -> io::Result<()> {
	create_test_types()
}