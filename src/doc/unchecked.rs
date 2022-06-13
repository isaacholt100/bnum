macro_rules! impl_desc {
	() => {
		doc::arithmetic_impl_desc!("Unchecked", "unchecked", "Each method results in undefined behavior if overflow/underflow occurs, i.e. when the checked equivalent would return `None`.")
	};
}

pub(crate) use impl_desc;