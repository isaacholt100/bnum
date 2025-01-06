use super::BUintD8;
use crate::{digit, Digit};
use alloc::string::String;
use core::fmt::Write;
use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};

macro_rules! fmt_method {
    ($format: expr, $format_pad: expr, $pad: expr, $prefix: expr) => {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
            let mut format_string = String::new();
            for digit in self.digits.iter().rev() {
                if format_string.is_empty() {
                    if digit != &0 {
                        write!(format_string, $format, digit)?;
                    }
                } else {
                    write!(format_string, $format_pad, digit, $pad)?;
                }
            }
            f.pad_integral(
                true,
                $prefix,
                if format_string.is_empty() {
                    "0"
                } else {
                    &format_string
                },
            )
        }
    };
}

impl<const N: usize> Binary for BUintD8<N> {
    /*#[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(true, "0b", &self.to_str_radix(2))
    }*/
    fmt_method!("{:b}", "{:01$b}", digit::BITS as usize, "0b");
}

impl<const N: usize> Debug for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> Display for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(true, "", &self.to_str_radix(10))
    }
}

macro_rules! exp_fmt {
    ($e: expr) => {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
            let decimal_str = self.to_str_radix(10);
            let buf = if decimal_str == "0" {
                format!("{}{}0", 0, $e)
            } else {
                let exp = decimal_str.len() - 1;
                let decimal_str = decimal_str.trim_end_matches('0');
                if decimal_str.len() == 1 {
                    format!("{}{}{}", &decimal_str[0..1], $e, exp)
                } else {
                    format!("{}.{}{}{}", &decimal_str[0..1], &decimal_str[1..], $e, exp)
                }
            };
            f.pad_integral(true, "", &buf)
        }
    };
}

impl<const N: usize> LowerExp for BUintD8<N> {
    exp_fmt!("e");
}

impl<const N: usize> LowerHex for BUintD8<N> {
    fmt_method!("{:x}", "{:01$x}", digit::HEX_PADDING, "0x");
}

impl<const N: usize> Octal for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let string = self.to_str_radix(8);
        f.pad_integral(true, "0o", &string)
    }
}

impl<const N: usize> UpperExp for BUintD8<N> {
    exp_fmt!("E");
}

impl<const N: usize> UpperHex for BUintD8<N> {
    fmt_method!("{:X}", "{:01$X}", digit::HEX_PADDING, "0x");
}

#[cfg(test)]
mod tests {
    crate::int::fmt::tests!(utest);
}
