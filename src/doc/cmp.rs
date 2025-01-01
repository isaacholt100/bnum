macro_rules! impl_desc {
    () => {
        "Comparison methods."
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    max,
    min,
    maximum,
    minimum,
    clamp,
    total_cmp
);