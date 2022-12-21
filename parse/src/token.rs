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

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Int(u64),
    Float(f64),
    Symbol(Symbol),
    Keyword(Keyword),
}
