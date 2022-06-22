use super::BInt;
use core::fmt::{Binary, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $pad: expr, $prefix: expr, $trait: tt) => {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            $trait::fmt(&self.bits, f)
        }
    }
}

impl<const N: usize> Binary for BInt<N> {
    fmt!("{:b}{:0pad$b}", Self::BITS, "0b", Binary);
}

impl<const N: usize> Display for BInt<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad_integral(!self.is_negative(), "", &format!("{}", self.unsigned_abs()))
    }
}

impl<const N: usize> fmt::Debug for BInt<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> LowerExp for BInt<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:e}", uint))
    }
}

impl<const N: usize> LowerHex for BInt<N> {
    fmt!("{:x}{:0pad$x}", Self::BITS / 4, "0x", LowerHex);
}

impl<const N: usize> Octal for BInt<N> {
    fmt!("{:o}{:0pad$o}", Self::BITS / 4, "0o", Octal);
}

impl<const N: usize> UpperExp for BInt<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:E}", uint))
    }
}

impl<const N: usize> UpperHex for BInt<N> {
    fmt!("{:X}{:0pad$X}", Self::BITS / 4, "0x", UpperHex);
}

crate::int::fmt::tests!(itest);