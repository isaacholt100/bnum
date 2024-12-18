macro_rules! impl_desc {
    () => {
        "Bigint helper methods: common functions used to implement big integer arithmetic."
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    abs,
    sqrt,
    div_euclid,
    rem_euclid,
    powi
);