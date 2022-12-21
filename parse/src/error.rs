use athame::error::Error;

use crate::{keyword::Keyword, symbol::Symbol, token::Token};

#[allow(dead_code)]
pub enum Expected {
    Ident,
    Symbol(Symbol),
    Keyword(Keyword),
}

impl std::fmt::Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident => write!(f, "identifier"),
            Self::Symbol(symbol) => write!(f, "'{}'", symbol),
            Self::Keyword(keyword) => write!(f, "'{}'", keyword),
        }
    }
}

pub fn expected_any(token: &Token, tokens: &[Expected]) -> Error {
    let strings: Vec<_> = tokens.iter().map(|e| format!("{}", e)).collect();

    Error::new(format!("unexpected token '{}'", token.kind))
        .with_span(token.span)
        .with_note(format!(
            "expected one of the following: {}",
            strings.join(", ")
        ))
}
