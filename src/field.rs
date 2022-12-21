use crate::{expr::Expr, ident::Ident, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}
