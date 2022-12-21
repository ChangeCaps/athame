use crate::{function::Argument, generics::Generics, ident::Ident, span::Span, ty::Type};

#[derive(Clone, Debug, PartialEq)]
pub struct SelfArgument {
    pub is_pointer: bool,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    pub name: Ident,
    pub generics: Generics,
    pub self_argument: Option<SelfArgument>,
    pub arguments: Vec<Argument>,
    pub return_ty: Type,
    pub span: Span,
}
