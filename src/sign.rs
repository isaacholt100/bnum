#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Minus,
    Zero,
    Plus,
}

impl Default for Sign {
    fn default() -> Self {
        Self::Zero
    }
}

impl Sign {
    pub const fn negate(&self) -> Self {
        match self {
            Self::Minus => Self::Plus,
            Self::Zero => Self::Zero,
            Self::Plus => Self::Minus,
        }
    }
    pub const fn combine(&self, other: Self) -> Self {
        match self {
            Self::Minus => other.negate(),
            Self::Zero => Self::Zero,
            Self::Plus => other,
        }
    }
    pub const fn not(&self) -> Self {
        match self {
            Self::Minus => Self::Plus,
            Self::Zero => Self::Minus,
            Self::Plus => Self::Minus,
        }
    }
    pub const fn bit_and(&self, other: Self) -> Self {
        match (self, other) {
            (Self::Zero, _) => Self::Zero,
            (_, Self::Zero) => Self::Zero,
            (Self::Minus, Self::Minus) => Self::Minus,
            _ => Self::Plus,
        }
    }
}