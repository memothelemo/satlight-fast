use satlight_macros::node;

use crate::{LastStmt, Stmt};

#[node]
pub struct Block {
    pub(crate) stmts: Vec<Stmt>,
    pub(crate) last_stmt: Option<LastStmt>,
}

#[node]
pub struct Ast {
    pub(crate) block: Block,
}
