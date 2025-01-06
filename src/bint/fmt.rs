use super::BIntD8;
use crate::{BUintD8, Digit};

use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};

macro_rules! fmt_trait {
    ($trait: tt) => {
        impl<const N: usize> $trait for BIntD8<N> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                $trait::fmt(&self.bits, f)
            }
        }
    };
}

fmt_trait!(Binary);

impl<const N: usize> Display for BIntD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(!self.is_negative(), "", &format!("{}", self.unsigned_abs()))
    }
}

impl<const N: usize> Debug for BIntD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> LowerExp for BIntD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:e}", uint))
    }
}
fmt_trait!(LowerHex);
fmt_trait!(Octal);

impl<const N: usize> UpperExp for BIntD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:E}", uint))
    }
}

fmt_trait!(UpperHex);

#[cfg(test)]
mod tests {
    crate::int::fmt::tests!(itest);
}
