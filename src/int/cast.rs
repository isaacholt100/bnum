macro_rules! tests {
	($($int: ty), *) => {
		#[cfg(test)]
		mod tests {
			use crate::types::{I128, U128, I64, U64};
			use crate::test;
			use crate::cast::{CastTo, CastFrom};
			
			$(
				test::test_from! {
					function: <$int as CastFrom>::cast_from,
					from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char)
				}
		
				test::test_into! {
					function: <$int as CastTo>::cast_to,
					into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, U64, I64, I128, U128)
				}
			)*
		}
	};
}

pub(crate) use tests;