use crate::{Block, Expr, Name};
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub struct IfStmtChain {
    #[clone]
    pub(crate) span: Span,
    pub(crate) condition: Expr,
    pub(crate) block: Block,
}

impl crate::Spanned for IfStmtChain {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct IfStmt {
    #[clone]
    pub(crate) span: Span,
    pub(crate) condition: Expr,
    pub(crate) block: Block,
    pub(crate) chains: Vec<IfStmtChain>,
    pub(crate) else_block: Option<Block>,
}

impl crate::Spanned for IfStmt {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct GenericFor {
    #[clone]
    pub(crate) span: Span,
    pub(crate) names: Vec<Name>,
    pub(crate) exprlist: Vec<Expr>,
    pub(crate) block: Block,
}

impl crate::Spanned for GenericFor {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct NumericFor {
    #[clone]
    pub(crate) span: Span,
    pub(crate) name: Name,
    pub(crate) start: Expr,
    pub(crate) end: Expr,
    pub(crate) step: Option<Expr>,
    pub(crate) block: Block,
}

impl crate::Spanned for NumericFor {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct RepeatStmt {
    #[clone]
    pub(crate) span: Span,
    pub(crate) condition: Expr,
    pub(crate) block: Block,
}

impl crate::Spanned for RepeatStmt {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct WhileStmt {
    #[clone]
    pub(crate) span: Span,
    pub(crate) condition: Expr,
    pub(crate) block: Block,
}

impl crate::Spanned for WhileStmt {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}
