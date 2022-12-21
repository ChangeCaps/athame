use athame::{
    error::Error,
    sources::{Source, SourceId},
    span::Span,
};

use crate::{
    keyword::Keyword,
    lexer::Lexer,
    symbol::Symbol,
    token::{Token, TokenKind},
};

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    source_id: SourceId,
}

impl Parser {
    pub fn new(source: &Source, source_id: SourceId) -> Result<Self, Error> {
        let mut lexer = Lexer::new(source, source_id);
        let mut tokens = Vec::new();

        while !lexer.is_empty() {
            tokens.push(lexer.lex()?);
        }

        Ok(Self {
            index: 0,
            tokens,
            source_id,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.remaining().is_empty()
    }

    pub fn remaining(&self) -> &[Token] {
        &self.tokens[self.index..]
    }

    pub fn span(&self) -> Span {
        if self.tokens.len() == 0 {
            Span::new(0, 0, self.source_id)
        } else {
            let index = self.index.min(self.tokens.len() - 1);
            self.tokens[index].span
        }
    }

    pub fn try_next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index)?;
        self.index += 1;
        Some(token)
    }

    pub fn try_peek(&self) -> Option<&Token> {
        self.remaining().first()
    }

    pub fn next(&mut self) -> Result<&Token, Error> {
        if let Some(token) = self.try_next() {
            Ok(token)
        } else {
            let token = Error::new("unexpected end of file");

            Err(token)
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, Error> {
        T::parse(self)
    }

    pub fn try_parse<T: Parse>(&mut self) -> Option<T> {
        let index = self.index;

        match T::parse(self) {
            Ok(value) => Some(value),
            Err(_) => {
                self.index = index;
                None
            }
        }
    }

    pub fn parse_spanned<T: Parse>(&mut self) -> Result<(T, Span), Error> {
        let start = self.span();
        let value = T::parse(self)?;
        let end = self.span();

        Ok((value, start.with(end)))
    }

    pub fn try_parse_spanned<T: Parse>(&mut self) -> Option<(T, Span)> {
        let start = self.span();
        let value = self.try_parse::<T>()?;
        let end = self.span();

        Some((value, start.with(end)))
    }

    pub fn expect<T: Expect>(&mut self, value: T) -> Result<(), Error> {
        if value.is(self.next()?) {
            Ok(())
        } else {
            Err(Error::new("unexpected token"))
        }
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, Error>;
}

pub trait Expect {
    fn is(&self, token: &TokenKind) -> bool;
}

impl Expect for Token {
    fn is(&self, token: &TokenKind) -> bool {
        self.kind.is(token)
    }
}

impl Expect for TokenKind {
    fn is(&self, token: &TokenKind) -> bool {
        self == token
    }
}

impl<T: Expect> Expect for &T {
    fn is(&self, token: &TokenKind) -> bool {
        (*self).is(token)
    }
}

impl Expect for Symbol {
    fn is(&self, token: &TokenKind) -> bool {
        let TokenKind::Symbol(symbol) = token else {
            return false;
        };

        self == symbol
    }
}

impl Expect for Keyword {
    fn is(&self, token: &TokenKind) -> bool {
        let TokenKind::Keyword(keyword) = token else {
            return false;
        };

        self == keyword
    }
}
