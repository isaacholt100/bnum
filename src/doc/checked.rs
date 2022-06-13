macro_rules! impl_desc {
	() => {
		doc::arithmetic_impl_desc!("Checked", "checked", "Each method cannot panic and returns an `Option<Self>`.")
	};
}

pub(crate) use impl_desc;