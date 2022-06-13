pub mod checked;
pub mod endian;
pub mod overflowing;
pub mod radix;
pub mod saturating;
pub mod unchecked;
pub mod wrapping;

macro_rules! arithmetic_impl_desc {
	($name: literal, $method: literal, $rest: literal) => {
		concat!($name, " arithmetic methods which act on `self`: `self.", $method, "_...`. ", $rest)
	};
}

pub(crate) use arithmetic_impl_desc;

macro_rules! int_str {
    ($ty: ident ::< $n: literal >) => {
        concat!(stringify!($ty), "::<", $n, ">")
    }
}

pub(crate) use int_str;

macro_rules! example_header {
    ($ty: ident) => {
        concat!(
"

# Examples
        
```
use bint::",
            stringify!($ty),
";

"
        )
    }
}

pub(crate) use example_header;

macro_rules! doc_comment {
    { $ty: ident ::< $n: literal >, $($desc: expr)+, $($code: expr)+ } => {
        concat!(
            $("\n\n", $desc), +,
            doc::example_header!($ty),
            $($code), +,
            "\n```"
        )
    }
}

pub(crate) use doc_comment;

macro_rules! min_const {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "The smallest value that can be represented by this integer type.",

            "assert_eq!(!" doc::int_str!($ty::<$n>) "::MIN, " stringify!($ty) "::MAX);"
        }
    };
}

pub(crate) use min_const;

macro_rules! max_const {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "The largest value that can be represented by this integer type.",

            "assert_eq!(" doc::int_str!($ty::<$n>) "::MAX.wrapping_add(" stringify!($ty) "::ONE), " stringify!($ty) "::MIN);"
        }
    };
}

pub(crate) use max_const;

macro_rules! zero_const {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "The value of zero represented by this type.",

            "assert_eq!(" doc::int_str!($ty::<$n>) "::ZERO, " stringify!($ty) "::from(0u8));"
        }
    }
}

pub(crate) use zero_const;

macro_rules! one_const {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "The value of one represented by this type.",

            "assert_eq!(" doc::int_str!($ty::<$n>) "::ONE, " stringify!($ty) "::from(1u8));"
        }
    }
}

pub(crate) use one_const;

macro_rules! bits_const {
    ($ty: ident ::< $n: literal >, $digit_bits: literal) => {
        concat!(
            "The size of this type in bits.",
            doc::example_header!($ty),
            "assert_eq!(",
            doc::int_str!($ty::<$n>),
            "::BITS, ",
            $n,
            " * ",
            $digit_bits,
            ");"
        )
    }
}

pub(crate) use bits_const;

macro_rules! bytes_const {
    ($ty: ident ::< $n: literal >, $digit_bytes: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "The size of this type in bytes.",

            "assert_eq!(" doc::int_str!($ty::<$n>) "::BYTES, " $n
            " * " $digit_bytes ");"
        }
    }
}

pub(crate) use bytes_const;

macro_rules! count_ones {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of ones in the binary representation of `self`.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0b010111101010000u16);\n"
            "assert_eq!(n.count_ones(), 7);"
        }
    }
}

pub(crate) use count_ones;

macro_rules! count_zeros {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of zeros in the binary representation of `self`.",

            "assert_eq!(!" doc::int_str!($ty::<$n>) "::ZERO.count_zeros(), 0);"
        }
    };
}

pub(crate) use count_zeros;

macro_rules! leading_zeros {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of leading zeros in the binary representation of `self`.",

            "let n = !" doc::int_str!($ty::<$n>) "::ZERO >> 4;\n"
            "assert_eq!(n.leading_zeros(), 4);"
        }
    };
}

pub(crate) use leading_zeros;

macro_rules! trailing_zeros {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of trailing zeros in the binary representation of `self`.",

            "let n = !" doc::int_str!($ty::<$n>) "::ZERO << 6;\n"
            "assert_eq!(n.trailing_zeros(), 6);"
        }
    };
}

pub(crate) use trailing_zeros;

macro_rules! leading_ones {
    ($ty: ident ::< $n: literal >, $c: ident) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of leading ones in the binary representation of `self`.",

            "let n = " doc::int_str!($ty::<$n>) "::" stringify!($c) " >> 8;\n"
            "assert_eq!((!n).leading_ones(), 8);"
        }
    };
}

pub(crate) use leading_ones;

macro_rules! trailing_ones {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the number of trailing ones in the binary representation of `self`.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0b1000101011011u16);\n"
            "assert_eq!(n.trailing_ones(), 2);"
        }
    };
}

pub(crate) use trailing_ones;

macro_rules! rotate_left {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end of the resulting integer."
            "Please note this isn't the same operation as the `<<` shifting operator!",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x25e800000000000000000000000039a9" $u "128);\n"
            "let m = " stringify!($ty) "::from(0x25e839a9" $u "128);\n"
            "assert_eq!(n.rotate_left(16), m);"
        }
    }
}

pub(crate) use rotate_left;

macro_rules! rotate_right {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end of the resulting integer."
            "Please note this isn't the same operation as the `>>` shifting operator!"
            "`rotate_right(n)` is equivalent to `rotate_left(BITS - n)` where `BITS` is the size of the type in bits.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x25e839a9" $u "128);\n"
            "let m = " stringify!($ty) "::from(0x25e800000000000000000000000039a9" $u "128);\n"
            "assert_eq!(n.rotate_right(16), m);"
        }
    }
}

pub(crate) use rotate_right;

macro_rules! swap_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Reverses the byte order of the integer.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x12345678901234567890123456789012" $u "128);\n"
            "let m = " stringify!($ty) "::from(0x12907856341290785634129078563412" $u "128);\n"
            "assert_eq!(n.swap_bytes(), m);"
        }
    }
}

pub(crate) use swap_bytes;

macro_rules! reverse_bits {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Reverses the order of bits in the integer. The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x12345678901234567890123456789012" $u "128);\n"
            "let m = " stringify!($ty) "::from(0x48091e6a2c48091e6a2c48091e6a2c48" $u "128);\n"
            "assert_eq!(n.reverse_bits(), m);\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) ", " stringify!($ty) "::ZERO.reverse_bits());"
        }
    };
}

pub(crate) use reverse_bits;

macro_rules! pow {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Raises `self` to the power of `exp`, using exponentiation by squaring.",

            "let n = " doc::int_str!($ty::<$n>) "::from(3u8);\n"
            "assert_eq!(n.pow(5), (243u8).into());"
        }
    };
}

pub(crate) use pow;

macro_rules! next_power_of_two {
    ($ty: ident ::< $n: literal >, $wrap: literal, $small: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            concat!("When return value overflows, it panics in debug mode and the return value is wrapped to", $wrap, "in release mode (the only situation in which method can return ", $wrap, ")."),
            
            "let n = " doc::int_str!($ty::<$n>) "::from(2u8);\n"
            "assert_eq!(n.next_power_of_two(), n);\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) 
            "::from(3u8).next_power_of_two(), 4u8.into());\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) "::" $small ".next_power_of_two(), " stringify!($ty) "::ONE);"
        }
    }
}

pub(crate) use next_power_of_two;

macro_rules! checked_next_power_of_two {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the smallest power of two greater than or equal to `self`. If the next power of two is greater than `Self::MAX`, `None` is returned, otherwise the power of two is wrapped in `Some`.",

            "let n = " doc::int_str!($ty::<$n>) "::from(2u8);\n"
            "assert_eq!(n.checked_next_power_of_two(), Some(n));\n"
            "let m = " doc::int_str!($ty::<$n>) "::from(3u8);\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) "::MAX.checked_next_power_of_two(), None);"
        }
    };
}

pub(crate) use checked_next_power_of_two;

macro_rules! wrapping_next_power_of_two {
    ($ty: ident ::< $n: literal >, $wrap: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            concat!("Returns the smallest power of two greater than or equal to `self`. If the next power of two is greater than `Self::MAX`, the return value is wrapped to .", $wrap),

            "let n = " doc::int_str!($ty::<$n>) "::from(31u8);\n"
            "assert_eq!(n, 32u8.into());\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) "::MAX.wrapping_next_power_of_two(), " stringify!($ty) "::MIN);"
        }
    };
}

pub(crate) use wrapping_next_power_of_two;

macro_rules! default {
    () => {
        "Returns the default value of `Self::ZERO`."
    }
}

pub(crate) use default;

macro_rules! bits {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns the fewest bits necessary to represent `self`."
            "This is equal to the size of the type in bits minus the leading zeros of `self`.",

            "assert_eq!(" doc::int_str!($ty::<$n>) "::from(0b1111001010100u16).bits(), 13);\n"
            "assert_eq!(" doc::int_str!($ty::<$n>) "::ZERO.bits(), 0);"
        }
    }
}

pub(crate) use bits;

macro_rules! bit {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns a boolean of the bit in the given position (`true` if the bit is set).",
            
            "let n = " doc::int_str!($ty::<$n>) "::from(0b001010100101010101u32);\n"
            "assert!(n.bit(0));\n"
            "assert!(!n.bit(1));\n"
            "assert!(!n.bit(" doc::int_str!($ty::<$n>) "::BITS - 1));"
        }
    };
}

pub(crate) use bit;

macro_rules! is_zero {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns whether `self` is zero.",

            "assert!(" doc::int_str!($ty::<$n>) "::ZERO.is_zero());\n"
            "assert!(!" doc::int_str!($ty::<$n>) "::ONE.is_zero());"
        }
    }
}

pub(crate) use is_zero;

macro_rules! is_one {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Returns whether `self` is one.",

            "assert!(" doc::int_str!($ty::<$n>) "::ONE.is_one());\n"
            "assert!(!" doc::int_str!($ty::<$n>) "::MAX.is_one());"
        }
    }
}

pub(crate) use is_one;

macro_rules! from_be {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Converts an integer from big endian to the target’s endianness."
            "On big endian this is a no-op. On little endian the bytes are swapped.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"big\") {\n"
            "    assert_eq!(" stringify!($ty) "::from_be(n), n);\n"
            "} else {\n"
            "    assert_eq!(" stringify!($ty) "::from_be(n), n.swap_bytes());\n"
            "}"
        }
    }
}

pub(crate) use from_be;

macro_rules! from_le {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Converts an integer from little endian to the target’s endianness."
            "On little endian this is a no-op. On big endian the bytes are swapped.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"little\") {\n"
            "    assert_eq!(" stringify!($ty) "::from_le(n), n);\n"
            "} else {\n"
            "    assert_eq!(" stringify!($ty) "::from_le(n), n.swap_bytes());\n"
            "}"
        }
    }
}

pub(crate) use from_le;

macro_rules! to_be {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Converts `self` from big endian to the target’s endianness."
            "On big endian this is a no-op. On little endian the bytes are swapped.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"big\") {\n"
            "    assert_eq!(n.to_be(), n);\n"
            "} else {\n"
            "    assert_eq!(n.to_be(), n.swap_bytes());\n"
            "}"
        }
    }
}

pub(crate) use to_be;

macro_rules! to_le {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Converts `self` from little endian to the target’s endianness."
            "On little endian this is a no-op. On big endian the bytes are swapped.",

            "let n = " doc::int_str!($ty::<$n>) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"little\") {\n"
            "    assert_eq!(n.to_le(), n);\n"
            "} else {\n"
            "    assert_eq!(n.to_le(), n.swap_bytes());\n"
            "}"
        }
    }
}

pub(crate) use to_le;

macro_rules! to_be_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Return the memory representation of this integer as a byte array in big-endian byte order.",

            "let bytes = " doc::int_str!($ty::<$n>) "::from(0x12345678901234567890123456789012" $u "128).to_be_bytes();\n"
            "assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]);"
        }
    }
}

pub(crate) use to_be_bytes;

macro_rules! to_le_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Return the memory representation of this integer as a byte array in little-endian byte order.",

            "let bytes = " doc::int_str!($ty::<$n>) "::from(0x12345678901234567890123456789012" $u "128).to_le_bytes();\n"
            "assert_eq!(bytes, [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);"
        }
    }
}

pub(crate) use to_le_bytes;

macro_rules! to_ne_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Return the memory representation of this integer as a byte array in native byte order.",

            "let bytes = " doc::int_str!($ty::<$n>) "::from(0x12345678901234567890123456789012" $u "128).to_ne_bytes();\n"
"assert_eq!(
    bytes,
    if cfg!(target_endian = \"big\") {
        [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
    } else {
        [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    }
);"
        }
    }
}

pub(crate) use to_ne_bytes;

macro_rules! from_be_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Create an endian integer value from its representation as a byte array in big endian.",

            "let value = " doc::int_str!($ty::<$n>) "::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]);\n"
            "assert_eq!(value, 0x12345678901234567890123456789012" $u "128.into());"
        }
    }
}

pub(crate) use from_be_bytes;

macro_rules! from_le_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Create an endian integer value from its representation as a byte array in little endian.",

            "let value = " doc::int_str!($ty::<$n>) "::from_le_bytes([0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);\n"
            "assert_eq!(value, 0x12345678901234567890123456789012" $u "128.into());"
        }
    }
}

pub(crate) use from_le_bytes;

macro_rules! from_ne_bytes {
    ($ty: ident ::< $n: literal >, $u: literal) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Create an endian integer value from its representation as a byte array in little endian.",

"let bytes = if cfg!(target_endian = \"big\") {
    [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
} else {
    [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
};\n"
            "let value = " doc::int_str!($ty::<$n>) "::from_ne_bytes(bytes);\n"
            "assert_eq!(value, 0x12345678901234567890123456789012" $u "128.into());"
        }
    }
}

pub(crate) use from_ne_bytes;

macro_rules! checked_add {
    ($ty: ident ::< $n: literal >) => {
        doc::doc_comment! {
            $ty::<$n>,
            "Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.",

            "assert_eq!((" doc::int_str!($ty::<$n>) "::MAX - " stringify!($ty) "::TWO).checked_add(" stringify!($ty) "::ONE), Some(" stringify!($ty) "::MAX - " stringify!($ty) "::ONE));\n"
            "assert_eq!((" doc::int_str!($ty::<$n>) "::MAX - " stringify!($ty) "::TWO).checked_add(" stringify!($ty) "::THREE), None);"
        }
    }
}

pub(crate) use checked_add;