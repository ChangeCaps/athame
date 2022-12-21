use std::collections::HashMap;

use crate::{
    data::{ClassId, FunctionId, ModuleId},
    ident::Ident,
    span::Span,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub name: Ident,
    pub modules: HashMap<Ident, ModuleId>,
    pub classes: HashMap<Ident, ClassId>,
    pub functions: HashMap<Ident, FunctionId>,
    pub span: Span,
}
