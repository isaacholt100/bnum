use super::BUint;
use crate::digit::DIGIT_BITS;
use std::fmt::{Binary, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};
macro_rules! fmt {
    ($format: expr, $format_pad: expr, $pad: expr, $prefix: expr) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let mut format_string = String::new();
            self.digits.iter().rev().for_each(|digit| {
                if format_string.is_empty() {
                    if digit != &0 {
                        format_string.push_str(&format!($format, digit));
                    }
                } else {
                    format_string.push_str(&format!($format_pad, digit, $pad));
                }
            });
            f.pad_integral(true, $prefix, if format_string.is_empty() {
                "0"
            } else {
                &format_string
            })
        }
    };
}

impl<const N: usize> Binary for BUint<N> {
    fmt!("{:b}", "{:01$b}", DIGIT_BITS, "0b");
}

/*impl<const N: usize> Debug for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}*/

impl<const N: usize> Display for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: implement
        write!(f, "Bigint")
    }
}

impl<const N: usize> LowerExp for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

const HEX_PADDING: usize = DIGIT_BITS / 4;

impl<const N: usize> LowerHex for BUint<N> {
    fmt!("{:x}", "{:01$x}", HEX_PADDING, "0x");
}

fn bin_str_to_oct_char(s: &str) -> char {
    match s {
        "000" | "00" | "0" => '0',
        "001" | "01" | "1" => '1',
        "010" | "10" => '2',
        "011" | "11" => '3',
        "100" => '4',
        "101" => '5',
        "110" => '6',
        "111" => '7',
        _ => unreachable!(),
    }
}

impl<const N: usize> Octal for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut bin_string = format!("{:b}", &self);
        let mut bytes = unsafe {
            bin_string.as_bytes_mut()
        };
        bytes.reverse();
        let mut string = String::new();
        bytes.chunks(3).rev().for_each(|buf| unsafe {
            match buf.len() {
                3 => {
                    string.push(bin_str_to_oct_char(std::str::from_utf8_unchecked(&[buf[2], buf[1], buf[0]])))
                },
                2 => {
                    string.push(bin_str_to_oct_char(std::str::from_utf8_unchecked(&[buf[1], buf[0]])))
                },
                _ => {
                    string.push(bin_str_to_oct_char(std::str::from_utf8_unchecked(buf)))
                }
            }
        });
        f.pad_integral(true, "0o", &string)
    }
}

impl<const N: usize> UpperExp for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<const N: usize> UpperHex for BUint<N> {
    fmt!("{:X}", "{:01$X}", HEX_PADDING, "0x");
}

#[cfg(test)]
mod tests {
    use crate::U128;

    macro_rules! test_fmt {
        ($a: expr, $name: ident, $format: expr) => {
            #[test]
            fn $name() {
                let buint = U128::from($a);
                assert_eq!(format!($format, $a), format!($format, buint));
            }
        }
    }

    test_fmt!(30034348u128, test_binary_format, "{:b}");

    test_fmt!(30034348u128, test_binary_verbose_format, "{:#b}");
    //test_fmt!(30034348u128, test_display_format, "{}");
    //test_fmt!(30034348u128, test_debug_format, "{:?}");
    //test_fmt!(30034348u128, test_lower_exp_format, "{:e}");
    test_fmt!(0x34534400000000000000000000434345u128, test_lower_hex_format, "{:x}");
    test_fmt!(0x34534400000000000000000000434345u128, test_lower_hex_verbose_format, "{:#x}");
    test_fmt!(0o30000000000000000000000000000000000001u128, test_octal_format, "{:o}");
    test_fmt!(0o30000000000000000000000000000000000001u128, test_octal_verbose_format, "{:#o}");
    //test_fmt!(30034348u128, test_upper_exp_format, "{:E}");
    test_fmt!(0x18000000000000000000000000001u128, test_upper_hex_format, "{:X}");
    test_fmt!(0x18000000000000000000000000001u128, test_upper_hex_verbose_format, "{:#X}");
}