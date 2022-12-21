use athame::{
    error::Error,
    ident::Ident,
    sources::{Source, SourceId},
    span::Span,
};

use crate::{
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
        let span = self.span();

        if let Some(token) = self.try_next() {
            Ok(token)
        } else {
            let token = Error::new("unexpected end of file").with_span(span);

            Err(token)
        }
    }

    pub fn peek(&self) -> Result<&Token, Error> {
        if let Some(token) = self.try_peek() {
            Ok(token)
        } else {
            let token = Error::new("unexpected end of file").with_span(self.span());

            Err(token)
        }
    }

    pub fn next_is<T>(&mut self, value: T) -> bool
    where
        TokenKind: PartialEq<T>,
    {
        let Some(token) = self.try_peek() else {
            return false;
        };

        if token.kind == value {
            self.index += 1;

            true
        } else {
            false
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

    pub fn expect<T>(&mut self, value: T) -> Result<(), Error>
    where
        TokenKind: PartialEq<T>,
        T: std::fmt::Display,
    {
        if self.next()?.kind == value {
            Ok(())
        } else {
            Err(Error::new("unexpected token"))
        }
    }

    pub fn parse_list<T: Parse>(&mut self, terminator: &TokenKind) -> Result<Vec<T>, Error> {
        let mut list = Vec::new();

        loop {
            if self.next_is(terminator.clone()) {
                break;
            }

            list.push(self.parse()?);

            if self.next_is(terminator.clone()) {
                break;
            }

            self.expect(Symbol::Comma)?;
        }

        Ok(list)
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, Error>;
}

impl Parse for Ident {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let token = parser.next()?;

        if let TokenKind::Ident(ref ident) = token.kind {
            Ok(Ident::new(ident.as_str(), token.span))
        } else {
            Err(Error::new("expected identifier").with_span(token.span))
        }
    }
}
