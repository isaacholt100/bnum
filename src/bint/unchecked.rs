use super::BIntD8;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> BIntD8<N> {
    crate::int::unchecked::impls!(I);
}

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(itest);
}

use crate::doc;
