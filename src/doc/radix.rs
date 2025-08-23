macro_rules! impl_desc {
    ($ty: ty) => {
        concat!(
            "Methods which convert a `",
            stringify!($ty),
            "` from and to strings and lists of digits in a given radix (base)."
        )
    };
}

pub(crate) use impl_desc;
