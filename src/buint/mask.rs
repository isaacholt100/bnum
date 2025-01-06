// macro_rules! mask {
//     (BUintD8: ident, BIntD8: ident, Digit: ident) => {
//         impl<const N: usize> BUintD8<N> {
//             #[inline]
//             pub(crate) const fn least_significant_n_bits(self, n: ExpType) -> Self {
//                 let mut mask = Self::ZERO;
//                 let mut digit_index = n as usize >> digit::BIT_SHIFT;
//                 let mut i = 0;
//                 while i < digit_index {
//                     mask.digits[i] = Digit::MAX;
//                     i += 1;
//                 }

//                 self.bitand(mask)
//             }
//         }
//     };
// }
