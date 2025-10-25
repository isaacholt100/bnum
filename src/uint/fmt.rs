use crate::Integer;
use core::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};

impl<const S: bool, const N: usize> Binary for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        if !S {
            f.pad_integral(true, "0b", &self.to_str_radix(2))
        } else {
            Binary::fmt(&self.force_sign::<false>(), f)
        }
    }
}

impl<const S: bool, const N: usize> LowerHex for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        if !S {
            f.pad_integral(true, "0x", &self.to_str_radix(16))
        } else {
            LowerHex::fmt(&self.force_sign::<false>(), f)
        }
    }
}

impl<const S: bool, const N: usize> UpperHex for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        if !S {
            let mut s = self.to_str_radix(16);
            s.make_ascii_uppercase();
            f.pad_integral(true, "0x", &s)
        } else {
            UpperHex::fmt(&self.force_sign::<false>(), f)
        }
    }
}

impl<const S: bool, const N: usize> Octal for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        if !S {
            f.pad_integral(true, "0o", &self.to_str_radix(8))
        } else {
            Octal::fmt(&self.force_sign::<false>(), f)
        }
    }
}

impl<const S: bool, const N: usize> Debug for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        Display::fmt(&self, f)
    }
}

// implementation if we don't have alloc, as otherwise can't call assert_eq! (since this requires Debug)
#[cfg(all(test, not(feature = "alloc")))]
impl<const N: usize> core::fmt::Debug for Uint<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for bytes in self.bytes.iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<const S: bool, const N: usize> Display for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        f.pad_integral(!self.is_negative_internal(), "", &self.unsigned_abs_internal().to_str_radix(10))
    }
}

impl<const S: bool, const N: usize> Integer<S, N> {
    #[inline]
    fn exp_fmt(&self, f: &mut Formatter, e: &str) -> core::fmt::Result {
        let decimal_str = self.unsigned_abs_internal().to_str_radix(10);
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
        f.pad_integral(!self.is_negative_internal(), "", &buf)
    }
}

impl<const S: bool, const N: usize> LowerExp for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.exp_fmt(f, "e")
    }
}

impl<const S: bool, const N: usize> UpperExp for Integer<S, N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.exp_fmt(f, "E")
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::test::test_bignum;

    macro_rules! format_trait {
        ($($method: ident), *) => {
            // This trait allows us to use the default tester macro instead of creating a custom one
            pub trait Format {
                $(
                    fn $method(&self, width: Option<u8>, extra: bool) -> alloc::string::String;
                )*
            }
        };
    }

    format_trait!(
        binary, lower_hex, upper_hex, octal, display, debug, lower_exp, upper_exp
    );

    macro_rules! impl_format_method {
        { $($name: ident : $format: literal), * } => {
            $(
                fn $name(&self, width: Option<u8>, extra: bool) -> alloc::string::String {
                    if let Some(width) = width {
                        if extra {
                            format!(concat!("{:+#0width$", $format, "}"), self, width = width as usize)
                        } else {
                            format!(concat!("{:width$", $format, "}"), self, width = width as usize)
                        }
                    } else if extra {
                        format!(concat!("{:+#", $format, "}"), self)
                    } else {
                        format!(concat!("{:", $format, "}"), self)
                    }
                }
            )*
        };
    }

    macro_rules! impl_format {
        ($($ty: ty), *) => {
            $(
                impl Format for $ty {
                    impl_format_method! {
                        binary: "b",
                        lower_hex: "x",
                        upper_hex: "X",
                        octal: "o",
                        display: "",
                        debug: "?",
                        lower_exp: "e",
                        upper_exp: "E"
                    }
                }
            )*
        };
    }

    crate::test::test_all! {
        testing integers;

        impl_format!(stest);
        impl_format!(STEST);


        test_bignum! {
            function: <stest as Format>::binary(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::lower_hex(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::upper_hex(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::octal(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::display(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::debug(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::lower_exp(a: ref &stest, width: Option<u8>, extra: bool)
        }
        test_bignum! {
            function: <stest as Format>::upper_exp(a: ref &stest, width: Option<u8>, extra: bool)
        }
    }
}

