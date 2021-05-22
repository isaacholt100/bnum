use super::BintTest;
use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $pad: expr, $prefix: expr, $trait: tt) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            $trait::fmt(&self.uint, f)
        }
    }
}

impl<const N: usize> Binary for BintTest<N> {
    fmt!("{:b}{:0pad$b}", Self::UINT_BITS, "0b", Binary);
}

impl<const N: usize> Display for BintTest<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

/*impl<const N: usize> Debug for BintTest<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Binary::fmt(self, f)
    }
}*/

impl<const N: usize> LowerExp for BintTest<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl<const N: usize> LowerHex for BintTest<N> {
    fmt!("{:x}{:0pad$x}", Self::UINT_BITS / 4, "0x", LowerHex);
}

impl<const N: usize> Octal for BintTest<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(8))
    }
}

impl<const N: usize> UpperExp for BintTest<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl<const N: usize> UpperHex for BintTest<N> {
    fmt!("{:X}{:0pad$X}", Self::UINT_BITS / 4, "0x", UpperHex);
}

#[cfg(test)]
mod tests {
    use crate::I128Test;

    #[test]
    fn test_binary_format() {
        let u = -3459837459374957398457354i128;
        assert_eq!(format!("{:#b}", I128Test::from(u)), format!("{:#b}", u));
    }
}