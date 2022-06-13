macro_rules! impl_desc {
	($ty: ty) => {
		stringify!("Methods which convert a `", $ty, "` between different types in a given radix (base).")
	};
}

pub(crate) use impl_desc;