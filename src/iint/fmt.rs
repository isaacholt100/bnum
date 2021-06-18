use super::BIint;
use core::fmt::{Binary, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $pad: expr, $prefix: expr, $trait: tt) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            $trait::fmt(&self.uint, f)
        }
    }
}

impl<const N: usize> Binary for BIint<N> {
    fmt!("{:b}{:0pad$b}", Self::BITS, "0b", Binary);
}

impl<const N: usize> Display for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad_integral(!self.is_negative(), "", &format!("{}", self.unsigned_abs()))
    }
}

/*impl<const N: usize> Debug for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Binary::fmt(self, f)
    }
}*/

impl<const N: usize> LowerExp for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:e}", uint))
    }
}

impl<const N: usize> LowerHex for BIint<N> {
    fmt!("{:x}{:0pad$x}", Self::BITS / 4, "0x", LowerHex);
}

impl<const N: usize> Octal for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(8))
    }
}

impl<const N: usize> UpperExp for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:E}", uint))
    }
}

impl<const N: usize> UpperHex for BIint<N> {
    fmt!("{:X}{:0pad$X}", Self::BITS / 4, "0x", UpperHex);
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