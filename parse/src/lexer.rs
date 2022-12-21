use athame::{
    error::Error,
    sources::{Source, SourceId},
    span::Span,
};

use crate::{
    keyword::Keyword,
    symbol::Symbol,
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a Source,
    source_id: SourceId,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source, source_id: SourceId) -> Self {
        Self {
            source,
            source_id,
            index: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.remaining().is_empty()
    }

    pub fn remaining(&self) -> &'a str {
        &self.source.source()[self.index..]
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn span(&self, end: usize) -> Span {
        Span::new(self.index, end - self.index, self.source_id)
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.remaining().chars().next()?;
        self.index += c.len_utf8();
        Some(c)
    }

    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.remaining().chars().nth(n)
    }

    pub fn take2(&mut self) {
        self.next();
        self.next();
    }

    pub fn take_whitespace(&mut self) -> &'a str {
        let index = self.index();

        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.next();
        }

        &self.source.source()[index..self.index]
    }

    pub fn lex_integer(&mut self, radix: u32) -> Result<u64, Error> {
        let mut value = 0;
        let mut digits = 0;

        while let Some(c) = self.peek() {
            let digit = match c.to_digit(radix) {
                Some(digit) => digit,
                None => break,
            };

            value = value * radix as u64 + digit as u64;
            digits += 1;

            self.next();
        }

        if digits == 0 {
            let err = Error::new(format!(
                "expected at least one digit in base {} integer",
                radix
            ))
            .with_span(self.span(self.index));

            return Err(err);
        }

        Ok(value)
    }

    pub fn lex_number(&mut self) -> Result<TokenKind, Error> {
        let mut radix = 10;

        if self.remaining().starts_with("0x") {
            self.take2();
            radix = 16;
        } else if self.remaining().starts_with("0o") {
            self.take2();
            radix = 8;
        } else if self.remaining().starts_with("0b") {
            self.take2();
            radix = 2;
        }

        let value = self.lex_integer(radix)?;

        if self.peek() == Some('.') && radix == 10 {
            self.next();

            let decimal = self.lex_integer(10)?;
            let digits = decimal.to_string().len() as i32;

            let value = value as f64 + decimal as f64 / f64::powi(10.0, digits);
        }

        todo!()
    }

    pub fn lex_ident(&mut self) -> Result<String, Error> {
        let mut ident = String::new();

        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            ident.push(c);
            self.next();
        }

        Ok(ident)
    }

    pub fn lex_symbol(&mut self, c: char) -> Result<Symbol, Error> {
        if let Some(symbol) = Symbol::from_parts(c, self.peek_nth(1)) {
            return Ok(symbol);
        }

        Err(Error::new(format!("invalid character '{}'", c)).with_span(self.span(self.index + 1)))
    }

    pub fn lex_kind(&mut self) -> Result<TokenKind, Error> {
        let c = self.next().unwrap();

        if c.is_alphabetic() || c == '_' {
            let ident = self.lex_ident()?;

            if let Some(keyword) = Keyword::from_str(&ident) {
                return Ok(TokenKind::Keyword(keyword));
            } else {
                return Ok(TokenKind::Ident(ident));
            }
        }

        if c.is_digit(10) {
            return self.lex_number();
        }

        if c == '.' {
            if let Some(c) = self.peek() {
                if c.is_digit(10) {
                    return self.lex_number();
                }
            }
        }

        Ok(TokenKind::Symbol(self.lex_symbol(c)?))
    }

    pub fn lex(&mut self) -> Result<Token, Error> {
        self.take_whitespace();

        let start = self.index();
        let kind = self.lex_kind()?;
        let span = self.span(start);

        Ok(Token::new(kind, span))
    }
}
