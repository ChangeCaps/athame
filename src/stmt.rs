use crate::{expr::Expr, ident::Ident, span::Span, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub struct LetStmt {
    pub name: Ident,
    pub ty: Option<Type>,
    pub expr: Option<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Expr(Expr),
    Let(LetStmt),
    Return(ReturnStmt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}
