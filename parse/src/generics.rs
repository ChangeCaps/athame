use athame::{
    error::Error,
    generics::{Generic, Generics},
};

use crate::{
    parser::{Parse, Parser},
    symbol::Symbol,
    token::TokenKind,
};

impl Parse for Generic {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        Ok(Self {
            name: parser.parse()?,
        })
    }
}

impl Parse for Generics {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let mut params = Vec::new();

        if parser.next_is(Symbol::Lt) {
            params = parser.parse_list::<Generic>(&TokenKind::Symbol(Symbol::Gt))?;
        }

        Ok(Self { params })
    }
}
