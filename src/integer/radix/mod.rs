mod from_radix;
#[cfg(feature = "alloc")]
mod to_radix;

macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(
            $radix >= 2 && $radix <= $max,
            crate::errors::err_msg!(concat!(
                "Radix must be in range [2, ",
                stringify!($max),
                "]"
            ))
        )
    };
}

pub(crate) use assert_range;

/// Returns the maximum power of `radix` that fits in a `u64`, together with the associated exponent
#[cfg(feature = "alloc")]
#[inline]
const fn max_radix_power(radix: u32) -> (u64, u32) {
    let mut power: u64 = radix as u64;
    let mut exponent = 1;
    loop {
        match power.checked_mul(radix as u64) {
            Some(n) => {
                power = n;
                exponent += 1;
            }
            None => return (power, exponent),
        }
    }
}

// we index using the radix itself
// creating a compile time constant will boost performance
#[cfg(feature = "alloc")]
const MAX_RADIX_POWERS: [(u64, u32); 257] = {
    let mut arr = [(0, 0); 257];
    let mut i = 2;
    while i <= 256 {
        arr[i] = max_radix_power(i as u32);
        i += 1;
    }
    arr
};

#[cfg(test)]
mod tests {
    use super::*;

    quickcheck::quickcheck! {
        fn quickcheck_max_radix_power(radix: u8) -> quickcheck::TestResult {
            if radix < 2 {
                return quickcheck::TestResult::discard();
            }
            let (power, exponent) = max_radix_power(radix as u32);
            let correct_exponent = u64::MAX.ilog(radix as u64);
            let correct_power = (radix as u64).pow(correct_exponent);
            quickcheck::TestResult::from_bool(power == correct_power && exponent == correct_exponent)
        }
    }
}
