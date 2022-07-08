macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(
            (2..=$max).contains(&$radix),
            crate::errors::err_msg!("Radix must be in range [2, {}]"),
            $max
        )
    };
}

pub(crate) use assert_range;
