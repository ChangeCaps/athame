use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Ident {
    value: Arc<str>,
    span: Span,
}

impl Ident {
    pub fn new(value: impl Into<Arc<str>>, span: Span) -> Self {
        Self {
            value: value.into(),
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Ident {}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
