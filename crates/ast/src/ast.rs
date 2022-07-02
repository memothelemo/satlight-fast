use crate::{LastStmt, Stmt};
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub struct Block {
    #[clone]
    pub(crate) span: Span,
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) last_stmt: Option<LastStmt>,
}

impl crate::Spanned for Block {
    fn span(&self) -> Option<satlight_common::location::Span> {
        Some(self.span)
    }
}

#[node]
pub struct Ast {
    pub(crate) block: Block,
}

impl crate::Spanned for Ast {
    fn span(&self) -> Option<Span> {
        Some(self.block.span().clone())
    }
}
