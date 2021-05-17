use super::Bint;
use std::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $pad: expr, $prefix: expr, $trait: tt) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            if self.signed_digit == 0 {
                $trait::fmt(&self.uint, f)
            } else {
                f.pad_integral(true, $prefix, &format!($format, self.signed_digit, self.uint, pad = $pad))
            }
        }
    }
}

impl<const N: usize> Binary for Bint<N> {
    fmt!("{:b}{:0pad$b}", Self::UINT_BITS, "0b", Binary);
}

impl<const N: usize> Display for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

/*impl<const N: usize> Debug for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Binary::fmt(self, f)
    }
}*/

impl<const N: usize> LowerExp for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<const N: usize> LowerHex for Bint<N> {
    fmt!("{:x}{:0pad$x}", Self::UINT_BITS / 4, "0x", LowerHex);
}

impl<const N: usize> Octal for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(8))
    }
}

impl<const N: usize> UpperExp for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<const N: usize> UpperHex for Bint<N> {
    fmt!("{:X}{:0pad$X}", Self::UINT_BITS / 4, "0x", UpperHex);
}

#[cfg(test)]
mod tests {
    use crate::I128;

    #[test]
    fn test_binary_format() {
        let u = -3459837459374957398457354i128;
        assert_eq!(format!("{:#b}", I128::from(u)), format!("{:#b}", u));
    }
}