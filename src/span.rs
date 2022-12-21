use crate::sources::SourceId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub index: usize,
    pub length: usize,
    pub source: SourceId,
}

impl Span {
    pub const fn new(index: usize, length: usize, source: SourceId) -> Self {
        Self {
            index,
            length,
            source,
        }
    }

    pub const fn null() -> Self {
        Self {
            index: 0,
            length: 0,
            source: SourceId::null(),
        }
    }

    pub const fn end(&self) -> usize {
        self.index + self.length
    }

    pub fn with(self, other: Span) -> Span {
        let index = self.index.min(other.index);
        let end = self.end().max(other.end());

        Span {
            index,
            length: end - index,
            source: self.source,
        }
    }

    pub fn is_null(&self) -> bool {
        self.index == 0 && self.length == 0 && self.source == SourceId::null()
    }
}
