macro_rules! impl_desc {
	() => {
		doc::arithmetic_impl_desc!("Wrapping", "wrapping", "Each method returns of the calculation truncated to the number of bits of `self`, i.e. they each return the first item in the tuple returned by their overflowing equivalent.")
	};
}

pub(crate) use impl_desc;