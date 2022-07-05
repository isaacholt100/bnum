impl<const N: usize> crate::BUint<N> {
	#[inline]
	pub(crate) const fn is_negative(&self) -> bool {
		false
	}
}

macro_rules! try_int_impl {
    ($Struct: tt, [$($int: ty), *]) => {
        $(
			impl<const N: usize> TryFrom<$Struct<N>> for $int {
				type Error = crate::errors::TryFromIntError;
			
				#[inline]
				fn try_from(from: $Struct<N>) -> Result<Self, Self::Error> {
					let digits = from.digits();
					let mut out = 0;
					let mut i = 0;
					while i << digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
						out |= digits[i] as $int << (i << digit::BIT_SHIFT);
						i += 1;
					}
					while i < N {
						if digits[i] != 0 {
							return Err(crate::errors::TryFromIntError {
								from: stringify!($Struct),
								to: stringify!($int),
								reason: crate::errors::TryFromErrorReason::TooLarge,
							});
						}
						i += 1;
					}
					#[allow(unused_comparisons)]
					if (out < 0) ^ from.is_negative() {
						return Err(crate::errors::TryFromIntError {
							from: stringify!($Struct),
							to: stringify!($int),
							reason: crate::errors::TryFromErrorReason::TooLarge,
						});
					}
					Ok(out)
				}
			}
		)*
    }
}
pub(crate) use try_int_impl;

macro_rules! all_try_int_impls {
    ($Struct: tt) => {
        crate::int::convert::try_int_impl!($Struct, [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]);
    }
}

pub(crate) use all_try_int_impls;