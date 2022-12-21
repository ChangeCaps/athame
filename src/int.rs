#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Int {
    pub signed: bool,
    pub size: Option<u8>,
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.signed {
            write!(f, "i")?;
        } else {
            write!(f, "u")?;
        }

        if let Some(size) = self.size {
            write!(f, "{}", size)
        } else {
            write!(f, "size")
        }
    }
}

impl Int {
    pub const fn new(signed: bool, size: Option<u8>) -> Self {
        Self { signed, size }
    }

    pub const fn signed(size: Option<u8>) -> Self {
        Self::new(true, size)
    }

    pub const fn unsigned(size: Option<u8>) -> Self {
        Self::new(false, size)
    }

    pub const fn i8() -> Self {
        Self::signed(Some(8))
    }

    pub const fn i16() -> Self {
        Self::signed(Some(16))
    }

    pub const fn i32() -> Self {
        Self::signed(Some(32))
    }

    pub const fn i64() -> Self {
        Self::signed(Some(64))
    }

    pub const fn i128() -> Self {
        Self::signed(Some(128))
    }

    pub const fn isize() -> Self {
        Self::signed(None)
    }

    pub const fn u8() -> Self {
        Self::unsigned(Some(8))
    }

    pub const fn u16() -> Self {
        Self::unsigned(Some(16))
    }

    pub const fn u32() -> Self {
        Self::unsigned(Some(32))
    }

    pub const fn u64() -> Self {
        Self::unsigned(Some(64))
    }

    pub const fn u128() -> Self {
        Self::unsigned(Some(128))
    }

    pub const fn usize() -> Self {
        Self::unsigned(None)
    }
}
