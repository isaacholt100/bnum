use super::BUint;
use std::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $format_pad: expr, $pad: expr) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let mut format_string = String::new();
            self.digits.iter().rev().for_each(|digit| {
                if digit != &0 {
                    if format_string.is_empty() {
                        format_string.push_str(&format!($format, digit));
                    } else {
                        format_string.push_str(&format!($format_pad, digit, $pad));
                    }
                }
            });
            write!(f, "{:0<1}", format_string)
        }
    };
}

impl<const N: usize> Binary for BUint<N> {
    fmt!("{:b}", "{:01$b}", 64);
}

impl<const N: usize> Debug for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> Display for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: implement
        write!(f, "Bigint")
    }
}

impl<const N: usize> LowerExp for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: implement
        write!(f, "Bigint")
    }
}

impl<const N: usize> LowerHex for BUint<N> {
    fmt!("{:x}", "{:01$x}", 16);
}

impl<const N: usize> Octal for BUint<N> {
    fmt!("{:o}", "{:01$o}", 23);
}

impl<const N: usize> UpperExp for BUint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO: implement
        write!(f, "Bigint")
    }
}

impl<const N: usize> UpperHex for BUint<N> {
    fmt!("{:X}", "{:01$X}", 16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_format() {
        let a = 30034348u128;
        let buint = BUint::<2>::from(a);
        assert_eq!(format!("{:b}", a), format!("{:b}", buint));
    }

    // Test display fmt
    // Test lower exp fmt

    #[test]
    fn test_lower_hex_format() {
        let a = 0x34534400000000000000000000434345u128;
        let buint = BUint::<2>::from(a);
        assert_eq!(format!("{:x}", a), format!("{:x}", buint));
    }

    #[test]
    fn test_octal_format() {
        let a = 0o1000000000000000000000000000001u128;
        let buint = BUint::<2>::from(a);
        assert_eq!(format!("{:x}", a), format!("{:x}", buint));
    }

    // Test upper exp fmt

    #[test]
    fn test_upper_hex_format() {
        let a = 0x10000000000000000000000000000001u128;
        let buint = BUint::<2>::from(a);
        assert_eq!(format!("{:x}", a), format!("{:x}", buint));
    }
}