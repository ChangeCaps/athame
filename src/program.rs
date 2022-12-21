use crate::decl::Decl;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub decls: Vec<Decl>,
}
