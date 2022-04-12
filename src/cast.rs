pub trait CastFrom<T> {
    fn cast_from(from: T) -> Self;
}

pub trait As {
    fn as_<T>(self) -> T where T: ~const CastFrom<Self>, Self: Sized;
}

impl<U> const As for U {
    #[inline]
    fn as_<T>(self) -> T where T: ~const CastFrom<Self>, Self: Sized {
        T::cast_from(self)
    }
}

macro_rules! primitive_cast_impl {
    ($from: ty as [$($ty: ty), *]) => {
        $(
            impl const CastFrom<$from> for $ty {
                #[inline]
                fn cast_from(from: $from) -> Self {
                    from as Self
                }
            }
        )*
    };
}

macro_rules! multiple_impls {
    ($($from: ty), *) => {
        $(
            primitive_cast_impl!($from as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64]);
        )*
    };
}

primitive_cast_impl!(bool as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool]);
primitive_cast_impl!(char as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char]);
primitive_cast_impl!(u8 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char]);
multiple_impls!(u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);