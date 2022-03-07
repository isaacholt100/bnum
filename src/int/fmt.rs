use super::Bint;
use core::fmt::{Binary, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex, self};

macro_rules! fmt {
    ($format: expr, $pad: expr, $prefix: expr, $trait: tt) => {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            $trait::fmt(&self.uint, f)
        }
    }
}

impl<const N: usize> Binary for Bint<N> {
    fmt!("{:b}{:0pad$b}", Self::BITS, "0b", Binary);
}

impl<const N: usize> Display for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad_integral(!self.is_negative(), "", &format!("{}", self.unsigned_abs()))
    }
}

impl<const N: usize> fmt::Debug for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<const N: usize> LowerExp for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:e}", uint))
    }
}

impl<const N: usize> LowerHex for Bint<N> {
    fmt!("{:x}{:0pad$x}", Self::BITS / 4, "0x", LowerHex);
}

impl<const N: usize> Octal for Bint<N> {
    fmt!("{:o}{:0pad$o}", Self::BITS / 4, "0o", Octal);
}

impl<const N: usize> UpperExp for Bint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let uint = self.unsigned_abs();
        f.pad_integral(!self.is_negative(), "", &format!("{:E}", uint))
    }
}

impl<const N: usize> UpperHex for Bint<N> {
    fmt!("{:X}{:0pad$X}", Self::BITS / 4, "0x", UpperHex);
}

#[cfg(test)]
mod tests {
    use crate::I128;
    use crate::macros::test_fmt;

    test_fmt! {
        int: I128,
        name: binary_format,
        format: "{:b}",
        numbers: {
            -453003434852456456456456i128,
            459485764958645769475689456i128,
            0i128,
            1i128
        }
    }
    test_fmt! {
        int: I128,
        name: binary_verbose_format,
        format: "{:#b}",
        numbers: {
            -24010972450692475964758564i128,
            204374209724560987245697856i128,
            0i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: lower_hex_format,
        format: "{:x}",
        numbers: {
            2027204957671072945797456i128,
            -10924576109874596207495678i128,
            0i128,
            1i128
        }
    }
    test_fmt! {
        int: I128,
        name: lower_hex_verbose_format,
        format: "{:#x}",
        numbers: {
            -2495701927459602794576984756i128,
            290802546729547694759867856456i128,
            0i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: octal_format,
        format: "{:o}",
        numbers: {
            209720942567204598679457689546455i128,
            -29367204598762948576894566465i128,
            0i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: octal_verbose_format,
        format: "{:#o}",
        numbers: {
            -9720459687249857689457689456456i128,
            27102495867476897429674958764564i128,
            0i128,
            1i128
        }
    }
    test_fmt! {
        int: I128,
        name: upper_hex_format,
        format: "{:X}",
        numbers: {
            8024956720456974956759687566456i128,
            -2945762459674596748596794655i128,
            0i128,
            1i128
        }
    }
    test_fmt! {
        int: I128,
        name: upper_hex_verbose_format,
        format: "{:#X}",
        numbers: {
            -29456729045679847568947565356i128,
            7290679274596749587689247456545i128,
            0i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: display_format,
        format: "{}",
        numbers: {
            780294576934567943754566574566i128,
            -2495876290456749576984576456i128,
            0i128,
            1i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: lower_exp_format,
        format: "{:e}",
        numbers: {
            7927495674690000000000000000i128,
            -4567984275896456000000000000i128,
            5i128,
            -20i128,
            30i128,
            0i128,
            1i128,
            -1i128
        }
    }
    test_fmt! {
        int: I128,
        name: upper_exp_format,
        format: "{:E}",
        numbers: {
            73495670927459679845700000000i128,
            -2927497698456000000i128,
            745600050i128,
            -569000i128,
            0i128,
            1i128,
            -1i128
        }
    }
}