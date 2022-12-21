#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Float {
    pub size: u8,
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f{}", self.size)
    }
}

impl Float {
    pub const fn new(size: u8) -> Self {
        Self { size }
    }

    pub const fn f16() -> Self {
        Self::new(16)
    }

    pub const fn f32() -> Self {
        Self::new(32)
    }

    pub const fn f64() -> Self {
        Self::new(64)
    }
}
