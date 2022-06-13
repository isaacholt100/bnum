macro_rules! impl_desc {
	() => {
		doc::arithmetic_impl_desc!("Overflowing", "overflowing", "Each method returns a tuple of type `(Self, bool)` where the first item of the tuple is the result of the calculation truncated to the number of bits of `self`, and the second item is a boolean which indicates whether overflow occurred (i.e. if the number of bits of the result of the calculation exceeded the number of bits of `self`).")
	};
}

pub(crate) use impl_desc;