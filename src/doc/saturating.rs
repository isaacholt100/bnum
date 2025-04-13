macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!("Saturating", "saturating", "For each method, if overflow or underflow occurs, the largest or smallest value that can be represented by `Self` is returned instead.")
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    saturating_add,
    saturating_div,
    saturating_mul,
    saturating_pow,
    saturating_sub
);

#[cfg(feature = "signed")]
crate::doc::link_doc_comment_method! {
    saturating_abs,
    saturating_add_signed,
    saturating_add_unsigned,
    saturating_neg,
    saturating_sub_unsigned
}
