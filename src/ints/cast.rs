#[cfg(test)]
macro_rules! tests {
    ($($int: ty), *) => {
        use crate::test;
        use crate::cast::{CastTo, CastFrom};

        use crate::test::cast_types::*;

        $(
            test::test_from! {
                function: <$int as CastFrom>::cast_from,
                from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10)
            }
            
            #[cfg(feature = "signed")]
            test::test_from! {
                function: <$int as CastFrom>::cast_from,
                from_types: (TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
            }

            test::test_into! {
                function: <$int as CastTo>::cast_to,
                into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10)
            }

            #[cfg(feature = "signed")]
            test::test_into! {
                function: <$int as CastTo>::cast_to,
                into_types: (TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
            }
        )*
    };
}

#[cfg(test)]
pub(crate) use tests;

