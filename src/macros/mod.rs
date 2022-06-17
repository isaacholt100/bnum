macro_rules! div_zero {
    () => {
        panic!(crate::error::err_msg!("attempt to divide by zero"))
    };
}
pub(crate) use div_zero;

macro_rules! rem_zero {
    () => {
        panic!(crate::error::err_msg!("attempt to calculate remainder with a divisor of zero"))
    };
}
pub(crate) use rem_zero;

macro_rules! option_expect {
    ($option: expr, $msg: expr) => {
        match $option {
            Some(value) => value,
            None => panic!($msg),
        }
    }
}
pub(crate) use option_expect;

macro_rules! assert_radix_range {
    ($radix: expr, $max: expr) => {
        assert!((2..=$max).contains(&$radix), crate::error::err_msg!("Radix must be in range [2, {}]"), $max)
    }
}

pub(crate) use assert_radix_range;