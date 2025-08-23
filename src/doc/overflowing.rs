macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!("Overflowing", "overflowing", "Each method returns a tuple of type `(Self, bool)` where the first item of the tuple is the result of wrapping variant of the method (`self.wrapped_...`), and the second item is a boolean which indicates whether overflow occurred.")
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    overflowing_add,
    overflowing_div,
    overflowing_div_euclid,
    overflowing_mul,
    overflowing_neg,
    overflowing_pow,
    overflowing_rem,
    overflowing_rem_euclid,
    overflowing_shl,
    overflowing_shr,
    overflowing_sub
);

#[cfg(feature = "signed")]
crate::doc::link_doc_comment_method! {
    overflowing_abs,
    overflowing_add_signed,
    overflowing_add_unsigned,
    overflowing_sub_signed,
    overflowing_sub_unsigned
}
