pub trait CastFrom<T> {
    fn cast_from(from: T) -> Self;
}

pub trait As {
    fn as_<T>(self) -> T where T: CastFrom<Self>, Self: Sized;
}

impl<U> As for U {
    fn as_<T>(self) -> T where T: CastFrom<Self>, Self: Sized {
        T::cast_from(self)
    }
}
