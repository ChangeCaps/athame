use athame::{
    error::Error,
    expr::{
        AssignExpr, BinOp, BinaryExpr, CallExpr, Expr, FieldExpr, IndexExpr, ParenExpr, UnaryOp,
    },
    ident::Ident,
};

use crate::{
    error::{expected_any, Expected},
    parser::{Parse, Parser},
    symbol::Symbol,
    token::TokenKind,
};

impl Parse for ParenExpr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let start = parser.span();

        parser.expect(Symbol::LeftParen)?;
        let expr = parser.parse::<Expr>()?;
        parser.expect(Symbol::RightParen)?;

        let end = parser.span();

        Ok(Self {
            expr: Box::new(expr),
            span: start.with(end),
        })
    }
}

fn term(parser: &mut Parser) -> Result<Expr, Error> {
    let token = parser.next()?;

    match token.kind {
        TokenKind::Symbol(Symbol::LeftParen) => {
            let expr = parser.parse::<ParenExpr>()?;

            Ok(Expr::Paren(expr))
        }
        _ => Err(Error::new("unexpected token")),
    }
}

fn field(parser: &mut Parser) -> Result<Expr, Error> {
    let expr = term(parser)?;

    if parser.next_is(Symbol::Dot) {
        let field = parser.parse::<Ident>()?;
        let span = expr.span().with(field.span());

        Ok(Expr::Field(FieldExpr {
            class: Box::new(expr),
            field,
            span,
        }))
    } else {
        Ok(expr)
    }
}

fn call(parser: &mut Parser) -> Result<Expr, Error> {
    let expr = field(parser)?;

    if parser.next_is(Symbol::LeftParen) {
        let arguments = parser.parse_list::<Expr>(&TokenKind::Symbol(Symbol::Comma))?;
        let span = expr.span().with(arguments.last().unwrap().span());

        Ok(Expr::Call(CallExpr {
            callee: Box::new(expr),
            arguments,
            span,
        }))
    } else {
        Ok(expr)
    }
}

fn index(parser: &mut Parser) -> Result<Expr, Error> {
    let expr = call(parser)?;

    if parser.next_is(Symbol::LeftBracket) {
        let index = parser.parse::<Expr>()?;
        parser.expect(Symbol::RightBracket)?;
        let span = expr.span().with(index.span());

        Ok(Expr::Index(IndexExpr {
            expr: Box::new(expr),
            index: Box::new(index),
            span,
        }))
    } else {
        Ok(expr)
    }
}

impl Parse for UnaryOp {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let token = parser.next()?;

        match token.kind {
            TokenKind::Symbol(Symbol::Minus) => Ok(Self::Neg),
            TokenKind::Symbol(Symbol::Bang) => Ok(Self::Not),
            TokenKind::Symbol(Symbol::Amp) => Ok(Self::Ref),
            TokenKind::Symbol(Symbol::Star) => Ok(Self::Deref),
            _ => Err(expected_any(
                token,
                &[
                    Expected::Symbol(Symbol::Minus),
                    Expected::Symbol(Symbol::Bang),
                    Expected::Symbol(Symbol::Amp),
                    Expected::Symbol(Symbol::Star),
                ],
            )),
        }
    }
}

fn unary(parser: &mut Parser) -> Result<Expr, Error> {
    let start = parser.span();

    if let Some(op) = parser.try_parse::<UnaryOp>() {
        let expr = unary(parser)?;
        let span = start.with(expr.span());

        Ok(Expr::Unary(athame::expr::UnaryExpr {
            op,
            expr: Box::new(expr),
            span,
        }))
    } else {
        index(parser)
    }
}

impl Parse for BinOp {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        let token = parser.next()?;

        match token.kind {
            TokenKind::Symbol(Symbol::Plus) => Ok(Self::Add),
            TokenKind::Symbol(Symbol::Minus) => Ok(Self::Sub),
            TokenKind::Symbol(Symbol::Star) => Ok(Self::Mul),
            TokenKind::Symbol(Symbol::Slash) => Ok(Self::Div),
            TokenKind::Symbol(Symbol::Percent) => Ok(Self::Mod),
            TokenKind::Symbol(Symbol::Amp) => Ok(Self::And),
            TokenKind::Symbol(Symbol::EqEq) => Ok(Self::Eq),
            TokenKind::Symbol(Symbol::NotEq) => Ok(Self::Ne),
            TokenKind::Symbol(Symbol::Lt) => Ok(Self::Lt),
            TokenKind::Symbol(Symbol::LtEq) => Ok(Self::Le),
            TokenKind::Symbol(Symbol::Gt) => Ok(Self::Gt),
            TokenKind::Symbol(Symbol::GtEq) => Ok(Self::Ge),
            _ => Err(expected_any(
                token,
                &[
                    Expected::Symbol(Symbol::Plus),
                    Expected::Symbol(Symbol::Minus),
                    Expected::Symbol(Symbol::Star),
                    Expected::Symbol(Symbol::Slash),
                    Expected::Symbol(Symbol::Percent),
                    Expected::Symbol(Symbol::Amp),
                    Expected::Symbol(Symbol::EqEq),
                    Expected::Symbol(Symbol::NotEq),
                    Expected::Symbol(Symbol::Lt),
                    Expected::Symbol(Symbol::LtEq),
                    Expected::Symbol(Symbol::Gt),
                    Expected::Symbol(Symbol::GtEq),
                ],
            )),
        }
    }
}

fn binary(parser: &mut Parser) -> Result<Expr, Error> {
    let expr = unary(parser)?;

    if let Some(op) = parser.try_parse::<BinOp>() {
        let rhs = binary(parser)?;
        let span = expr.span().with(rhs.span());

        if let Expr::Binary(ref rhs) = rhs {
            if rhs.op.precedence() > op.precedence() {
                let expr = Expr::Binary(BinaryExpr {
                    lhs: Box::new(expr),
                    op,
                    rhs: rhs.lhs.clone(),
                    span: span.with(rhs.span),
                });

                return Ok(Expr::Binary(BinaryExpr {
                    lhs: Box::new(expr),
                    op: rhs.op.clone(),
                    rhs: rhs.rhs.clone(),
                    span: span.with(rhs.span),
                }));
            }
        }

        Ok(Expr::Binary(BinaryExpr {
            lhs: Box::new(expr),
            op,
            rhs: Box::new(rhs),
            span,
        }))
    } else {
        Ok(expr)
    }
}

fn assign(parser: &mut Parser) -> Result<Expr, Error> {
    let expr = binary(parser)?;

    if parser.next_is(Symbol::Eq) {
        let rhs = assign(parser)?;
        let span = expr.span().with(rhs.span());

        Ok(Expr::Assign(AssignExpr {
            lhs: Box::new(expr),
            rhs: Box::new(rhs),
            span,
        }))
    } else {
        Ok(expr)
    }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        assign(parser)
    }
}
