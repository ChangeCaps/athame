use athame::{
    error::Error,
    expr::{Expr, ExprKind, ParenExpr},
};

use crate::{
    parser::{Parse, Parser},
    symbol::Symbol,
};

impl Parse for ParenExpr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let start = parser.span();

        parser.expect(Symbol::LeftParen)?;
    }
}

impl Parse for ExprKind {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        todo!()
    }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        todo!()
    }
}
