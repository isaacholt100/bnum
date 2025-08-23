macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!("Unchecked", "unchecked", "Each method results in undefined behavior if overflow occurs (i.e. when the checked equivalent would return `None`). These methods should therefore only be used when it can be guaranteed that overflow will not occur. If you want to avoid panicking in arithmetic in debug mode, use the wrapping equivalents instead.")
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    unchecked_add,
    unchecked_mul,
    unchecked_shl,
    unchecked_shr,
    unchecked_sub
);

#[cfg(feature = "signed")]
crate::doc::link_doc_comment_method!(
    unchecked_neg
);
