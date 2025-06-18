use super::Int;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> Int<N> {
    crate::int::unchecked::impls!(I);
}

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(itest);
}

use crate::doc;
