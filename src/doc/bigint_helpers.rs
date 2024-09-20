use crate::doc;

macro_rules! impl_desc {
    () => {
        "Bigint helper methods: common functions used to implement big integer arithmetic."
    };
}

pub(crate) use impl_desc;

doc::link_doc_comment!(carrying_add, borrowing_sub, widening_mul, carrying_mul);
