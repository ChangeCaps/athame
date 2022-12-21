use crate::{span::Span, stmt::Stmt};

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl Block {
    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
}
