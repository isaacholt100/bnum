pub enum Assert<const COND: bool> {}

//pub type AssertN<const N: usize, const M: usize> = Assert<{N > M}>;

pub trait IsTrue {}

impl IsTrue for Assert<true> {}