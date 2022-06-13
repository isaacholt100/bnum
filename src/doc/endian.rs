macro_rules! impl_desc {
	($ty: ty) => {
		concat!("Methods which convert a `", stringify!($ty), "` to and from data stored in different endianness.")
	};
}

pub(crate) use impl_desc;