macro_rules! impl_desc {
	() => {
		doc::arithmetic_impl_desc!("Saturating", "saturating", "For each method, if overflow or underflow occurs, the largest or smallest value that can be represented by `Self` is returned instead.")
	};
}

pub(crate) use impl_desc;