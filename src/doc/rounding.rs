macro_rules! impl_desc {
    () => {
        "Rounding methods."
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    round,
    ceil,
    floor,
    trunc,
    fract
);