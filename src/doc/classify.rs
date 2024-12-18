macro_rules! impl_desc {
    () => {
        "Classification methods: used to determine properties about the storage of the number."
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    is_sign_positive,
    is_sign_negative,
    is_finite,
    is_infinite,
    is_nan,
    is_subnormal,
    is_normal,
    classify
);