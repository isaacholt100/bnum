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
#[inline]
const fn max_radix_power(radix: u32) -> (u64, usize) {
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
const MAX_RADIX_POWERS: [(u64, usize); 257] = {
    let mut arr = [(0, 0); 257];
    let mut i = 2;
    while i <= 256 {
        arr[i] = max_radix_power(i as u32);
        i += 1;
    }
    arr
};

