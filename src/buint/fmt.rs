use super::BUint;
use crate::digit;
use core::fmt::{Binary, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};
use alloc::string::String;
use core::fmt::Write;

macro_rules! fmt_method {
    ($format: expr, $format_pad: expr, $pad: expr, $prefix: expr) => {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let mut format_string = String::new();
            for digit in self.digits.iter().rev() {
                if format_string.is_empty() {
                    if digit != &0 {
						write!(format_string, $format, digit)?;
                    }
                } else {
					write!(format_string, $format_pad, digit, $pad)?;
                }
            };
            f.pad_integral(true, $prefix, if format_string.is_empty() {
                "0"
            } else {
                &format_string
            })
        }
    };
}

impl<const N: usize> Binary for BUint<N> {
    fmt_method!("{:b}", "{:01$b}", digit::BITS as usize, "0b");
}

impl<const N: usize> core::fmt::Debug for BUint<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> Display for BUint<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

macro_rules! exp_fmt {
    ($e: expr) => {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let decimal_str = format!("{}", self);
            if decimal_str == "0" {
                return write!(f, "{}{}0", 0, $e);
            }
            let exp = decimal_str.len() - 1;
            let decimal_str = decimal_str.trim_end_matches('0');
            if decimal_str.len() == 1 {
                write!(f, "{}{}{}", &decimal_str[0..1], $e, exp)
            } else {
                write!(f, "{}.{}{}{}", &decimal_str[0..1], &decimal_str[1..], $e, exp)
            }
        }
    };
}

impl<const N: usize> LowerExp for BUint<N> {
    exp_fmt!("e");
}

const HEX_PADDING: usize = digit::BITS as usize / 4;

impl<const N: usize> LowerHex for BUint<N> {
    fmt_method!("{:x}", "{:01$x}", HEX_PADDING, "0x");
}

impl<const N: usize> Octal for BUint<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = self.to_str_radix(8);
        f.pad_integral(true, "0o", &string)
    }
}

impl<const N: usize> UpperExp for BUint<N> {
    exp_fmt!("E");
}

impl<const N: usize> UpperHex for BUint<N> {
    fmt_method!("{:X}", "{:01$X}", HEX_PADDING, "0x");
}

#[cfg(test)]
mod tests {
    use crate::types::U128;
    use crate::test::test_fmt;

    test_fmt! {
        int: U128,
        name: binary,
        format: "b",
        numbers: {
            30034348u128,
            0b100110100101010101011101010101u128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: lower_hex,
        format: "x",
        numbers: {
            0x45435345u128,
            0x4979457693459874abcdefu128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: octal,
        format: "o",
        numbers: {
            0o30000000000000000000000000000000000001u128,
            39457394759u128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: upper_hex,
        format: "X",
        numbers: {
            0x49867ABBBB34754975CC454u128,
            0x9649567ABCEEDu128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: display,
        format: "",
        numbers: {
            349578347589374664564568395748345u128,
            93847934758734u128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: debug,
        format: "?",
        numbers: {
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: lower_exp,
        format: "e",
        numbers: {
            831647298645678945768947500000u128,
            10u128,
            5u128,
            20u128,
            0u128,
            1u128
        }
    }
    test_fmt! {
        int: U128,
        name: upper_exp,
        format: "E",
        numbers: {
            982736597459678457689674000000u128,
            100400u128,
            50050u128,
            0u128,
            1u128
        }
    }
}