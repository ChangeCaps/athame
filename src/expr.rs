use crate::{ident::Ident, path::Path, span::Span};

#[derive(Clone, Debug, PartialEq)]
pub struct ParenExpr {
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldExpr {
    pub class: Box<Expr>,
    pub field: Ident,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexExpr {
    pub expr: Box<Expr>,
    pub index: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Ref,
    Deref,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl BinOp {
    pub const fn precedence(&self) -> u8 {
        match self {
            Self::Mul | Self::Div | Self::Mod => 2,
            Self::Add | Self::Sub => 3,
            Self::Lt | Self::Le | Self::Gt | Self::Ge => 4,
            Self::Eq | Self::Ne => 5,
            Self::And => 6,
            Self::Or => 7,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Paren(ParenExpr),
    Path(Path),
    Field(FieldExpr),
    Call(CallExpr),
    Index(IndexExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Assign(AssignExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Paren(expr) => expr.span,
            Self::Path(expr) => expr.span,
            Self::Field(expr) => expr.span,
            Self::Call(expr) => expr.span,
            Self::Index(expr) => expr.span,
            Self::Unary(expr) => expr.span,
            Self::Binary(expr) => expr.span,
            Self::Assign(expr) => expr.span,
        }
    }
}
