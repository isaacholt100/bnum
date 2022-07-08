macro_rules! impl_desc {
    ($ty: ty) => {
        concat!(
            "Methods which convert a `",
            stringify!($ty),
            "` to and from data stored in different endianness."
        )
    };
}

pub(crate) use impl_desc;

macro_rules! from_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_be,
            $sign $bits,
            "Converts an integer from big endian to the target’s endianness."
            "On big endian this is a no-op. On little endian the bytes are swapped.",

            "let n = " doc::type_str!($sign $bits) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"big\") {\n"
            "    assert_eq!(" doc::type_str!($sign $bits) "::from_be(n), n);\n"
            "} else {\n"
            "    assert_eq!(" doc::type_str!($sign $bits) "::from_be(n), n.swap_bytes());\n"
            "}"
        }
    };
}

pub(crate) use from_be;

macro_rules! from_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #from_le,
            $sign $bits,
            "Converts an integer from little endian to the target’s endianness."
            "On little endian this is a no-op. On big endian the bytes are swapped.",

            "let n = " doc::type_str!($sign $bits) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"little\") {\n"
            "    assert_eq!(" doc::type_str!($sign $bits) "::from_le(n), n);\n"
            "} else {\n"
            "    assert_eq!(" doc::type_str!($sign $bits) "::from_le(n), n.swap_bytes());\n"
            "}"
        }
    };
}

pub(crate) use from_le;

macro_rules! to_be {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #to_be,
            $sign $bits,
            "Converts `self` from big endian to the target’s endianness."
            "On big endian this is a no-op. On little endian the bytes are swapped.",

            "let n = " doc::type_str!($sign $bits) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"big\") {\n"
            "    assert_eq!(n.to_be(), n);\n"
            "} else {\n"
            "    assert_eq!(n.to_be(), n.swap_bytes());\n"
            "}"
        }
    };
}

pub(crate) use to_be;

macro_rules! to_le {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #to_le,
            $sign $bits,
            "Converts `self` from little endian to the target’s endianness."
            "On little endian this is a no-op. On big endian the bytes are swapped.",

            "let n = " doc::type_str!($sign $bits) "::from(0x1Au8);\n"
            "if cfg!(target_endian = \"little\") {\n"
            "    assert_eq!(n.to_le(), n);\n"
            "} else {\n"
            "    assert_eq!(n.to_le(), n.swap_bytes());\n"
            "}"
        }
    };
}

pub(crate) use to_le;

#[cfg(feature = "nightly")]
crate::doc::link_doc_comment! {
    to_be_bytes,
    to_le_bytes,
    to_ne_bytes,
    from_be_bytes,
    from_le_bytes,
    from_ne_bytes
}
