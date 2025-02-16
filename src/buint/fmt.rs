use super::BUintD8;
use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};


impl<const N: usize> Binary for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(true, "0b", &self.to_str_radix(2))
    }
}

impl<const N: usize> LowerHex for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(true, "0x", &self.to_str_radix(16))
    }
}

impl<const N: usize> UpperHex for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let mut s = self.to_str_radix(16);
        s.make_ascii_uppercase();
        f.pad_integral(true, "0x", &s)
    }
}

impl<const N: usize> Octal for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(true, "0o", &self.to_str_radix(8))
    }
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

impl<const N: usize> BUintD8<N> {
    #[inline]
    fn exp_fmt(&self, f: &mut Formatter, e: &str) -> core::fmt::Result {
        let decimal_str = self.to_str_radix(10);
        let buf = if decimal_str == "0" {
            format!("{}{}0", 0, e)
        } else {
            let exp = decimal_str.len() - 1;
            let decimal_str = decimal_str.trim_end_matches('0');
            if decimal_str.len() == 1 {
                format!("{}{}{}", &decimal_str[0..1], e, exp)
            } else {
                format!("{}.{}{}{}", &decimal_str[0..1], &decimal_str[1..], e, exp)
            }
        };
        f.pad_integral(true, "", &buf)
    }
}

impl<const N: usize> LowerExp for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.exp_fmt(f, "e")
    }
}

impl<const N: usize> UpperExp for BUintD8<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.exp_fmt(f, "E")
    }
}

#[cfg(test)]
mod tests {
    crate::int::fmt::tests!(utest);
}
