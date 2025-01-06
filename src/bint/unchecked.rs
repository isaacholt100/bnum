use super::BIntD8;
use crate::{BUintD8, Digit};


crate::int::unchecked::impls!(BIntD8, I);

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(itest);
}

use crate::doc;
