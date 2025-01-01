macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!("Strict", "strict", "Each method will always panic if overflow/underflow occurs (i.e. when the checked equivalent would return `None`), regardless of whether overflow checks are enabled.")
    };
}

pub(crate) use impl_desc;

crate::doc::link_doc_comment_method!(
    strict_abs,
    strict_add,
    strict_add_signed,
    strict_add_unsigned,
    strict_div,
    strict_div_euclid,
    strict_mul,
    strict_neg,
    strict_pow,
    strict_rem,
    strict_rem_euclid,
    strict_shl,
    strict_shr,
    strict_sub,
    strict_sub_unsigned
);
