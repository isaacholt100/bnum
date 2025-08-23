use crate::doc;

macro_rules! impl_desc {
    () => {
        doc::arithmetic_impl_desc!(
            "Checked",
            "checked",
            "Each method cannot panic and returns an `Option<Self>`. `None` is returned when overflow would have occurred or there was an attempt to divide by zero or calculate a remainder with a divisor of zero."
        )
    };
}

pub(crate) use impl_desc;

macro_rules! checked_add {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #method.checked_add,
            $sign $bits,
            "Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.",

            "use bnum::prelude::*;\n\n"
            "assert_eq!(1.as_::<" doc::type_str!($sign $bits) ">().checked_add(1.as_()), Some(2.as_());\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::MAX.checked_add(" doc::type_str!($sign $bits) "::ONE), None);"
        }
    };
}

pub(crate) use checked_add;

macro_rules! checked_sub {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #method.checked_add,
            $sign $bits,
            "Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.",

            "use bnum::prelude::*;\n\n"
            "assert_eq!(1.as_::<" doc::type_str!($sign $bits) ">().checked_sub(1.as_()), Some(0.as_());\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::MIN.checked_sub(" doc::type_str!($sign $bits) "::ONE), None);"
        }
    };
}

pub(crate) use checked_sub;

macro_rules! checked_mul {
    ($Type: ident) => {
        doc::doc_string! {
            $Type::checked_add,
            "Checked integer multiplication. Computes `self * rhs`, returning `None` if overflow occurred.",

            "use bnum::prelude::*;)\n\n"
            "assert_eq!(1.as_::<$Type>().checked_mul(1.as_()), Some(1.as_()));)"
            "assert_eq!($Type::MAX.checked_sub($Type::TWO), None);"
        }
    };
}

pub(crate) use checked_mul;

doc::link_doc_comment_method!(
    checked_signed_diff,
    checked_div,
    checked_div_euclid,
    checked_ilog,
    checked_ilog10,
    checked_ilog2,
    checked_neg,
    checked_next_multiple_of,
    checked_pow,
    checked_rem,
    checked_rem_euclid,
    checked_shl,
    checked_shr
);

#[cfg(feature = "signed")]
doc::link_doc_comment_method!(
    checked_abs,
    checked_add_signed,
    checked_add_unsigned,
    checked_sub_signed,
    checked_sub_unsigned
);
