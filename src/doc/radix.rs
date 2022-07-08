macro_rules! impl_desc {
    ($ty: ty) => {
        concat!(
            "Methods which convert a `",
            stringify!($ty),
            "` between different types in a given radix (base)."
        )
    };
}

pub(crate) use impl_desc;
