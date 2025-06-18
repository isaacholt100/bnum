use super::Uint;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> Uint<N> {
    crate::int::unchecked::impls!(U);
}

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(utest);
}

use crate::doc;
