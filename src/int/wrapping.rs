macro_rules! wrapping_method {
	($wrap: ident, $overflow: ident $(, $rhs: ty)?) => {
		#[inline]
		pub const fn $wrap(self $(, rhs: $rhs)?) -> Self {
			self.$overflow($(rhs as $rhs)?).0
		}
	};
}

pub(crate) use wrapping_method;