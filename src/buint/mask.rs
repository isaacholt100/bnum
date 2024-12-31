// macro_rules! mask {
//     ($BUint: ident, $BInt: ident, $Digit: ident) => {
//         impl<const N: usize> $BUint<N> {
//             #[inline]
//             pub(crate) const fn least_significant_n_bits(self, n: ExpType) -> Self {
//                 let mut mask = Self::ZERO;
//                 let mut digit_index = n as usize >> digit::$Digit::BIT_SHIFT;
//                 let mut i = 0;
//                 while i < digit_index {
//                     mask.digits[i] = $Digit::MAX;
//                     i += 1;
//                 }
                
//                 self.bitand(mask)
//             }
//         }
//     };
// }