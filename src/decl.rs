use crate::{
    block::Block, function::Argument, generics::Generics, ident::Ident, span::Span, ty::Type,
};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub generics: Generics,
    pub arguments: Vec<Argument>,
    pub return_type: Type,
    pub body: Block,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Function(FunctionDecl),
}
