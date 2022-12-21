use athame::span::Span;

use crate::{keyword::Keyword, symbol::Symbol};

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Symbol(Symbol),
    Keyword(Keyword),
}

impl PartialEq<Symbol> for TokenKind {
    fn eq(&self, other: &Symbol) -> bool {
        match self {
            Self::Symbol(symbol) => symbol == other,
            _ => false,
        }
    }
}

impl PartialEq<Keyword> for TokenKind {
    fn eq(&self, other: &Keyword) -> bool {
        match self {
            Self::Keyword(keyword) => keyword == other,
            _ => false,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "{}", ident),
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}
