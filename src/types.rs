use crate::{BUint, digit, BInt};

macro_rules! int_type_doc {
	($bits: literal, $sign: literal) => {
		concat!($bits, "-bit ", $sign, " integer type.")
	};
}

macro_rules! int_types {
	{ $($bits: literal $u: ident $i: ident; ) *}  => {
		$(
			#[doc=int_type_doc!($bits, "unsigned")]
			pub type $u = BUint::<{$bits / digit::BITS as usize}>;

			#[doc=int_type_doc!($bits, "signed")]
			pub type $i = BInt::<{$bits / digit::BITS as usize}>;
		)*
	};
}

int_types! {
	256 U256 I256;
	512 U512 I512;
	1024 U1024 I1024;
	2048 U2048 I2048;
	4096 U4096 I4096;
	8192 U8192 I8192;
}

// The below types are for testing purposes only and are not publicly exposed

#[allow(unused)]
pub type U64 = BUint::<{64 / digit::BITS as usize}>;

#[allow(unused)]
pub type U128 = BUint::<{128 / digit::BITS as usize}>;

#[allow(unused)]
pub(crate) type I64 = BInt::<{64 / digit::BITS as usize}>;
#[allow(unused)]
pub(crate) type I128 = BInt::<{128 / digit::BITS as usize}>;