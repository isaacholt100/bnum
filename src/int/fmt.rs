macro_rules! format_trait {
	($($method: ident), *) => {
		use alloc::string::String;

		pub trait Format {
			$(
				fn $method(&self) -> String;
			)*
		}
	};
}

format_trait!(binary, lower_hex, upper_hex, octal, display, debug, lower_exp, upper_exp);

#[allow(unused)]
macro_rules! impl_format_method {
	{ $($name: ident : $format: literal), * } => {
		$(
			fn $name(&self) -> String {
				format!(concat!("{:", $format, "}"), self)
			}
		)*
	};
}

#[allow(unused)]
pub(crate) use impl_format_method;

#[allow(unused)]
macro_rules! impl_format {
	($ty: ty) => {
		impl Format for $ty {
			crate::int::fmt::impl_format_method! {
				binary: "b",
				lower_hex: "x",
				upper_hex: "X",
				octal: "o",
				display: "",
				debug: "?",
				lower_exp: "e",
				upper_exp: "E"
			}
		}
	};
}

#[allow(unused)]
pub(crate) use impl_format;

#[allow(unused)]
macro_rules! test_formats {
	($ty: ty; $($name: ident), *) => {
		$(
			test_bignum! {
				function: <$ty as Format>::$name(a: ref &$ty)
			}
		)*
	};
}

#[allow(unused)]
pub(crate) use test_formats;

macro_rules! tests {
	($ty: ty) => {
		#[cfg(test)]
		mod tests {
			use crate::int::fmt::{Format, self};
			use crate::test::{test_bignum, types::*};
			use alloc::string::String;

			fmt::impl_format!($ty);
			paste::paste! {
				fmt::impl_format!([<$ty:upper>]);
			}

			fmt::test_formats!($ty; binary, lower_hex, upper_hex, octal, display, debug, lower_exp, upper_exp);
		}
	};
}

pub(crate) use tests;