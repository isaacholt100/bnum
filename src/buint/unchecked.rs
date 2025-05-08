use super::BUintD8;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    crate::int::unchecked::impls!(U);
}

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(utest);
}

use crate::doc;
