use satlight_common::location::Span;
use satlight_macros::node;

use crate::{Block, Expr, FunctionParam, Name};

#[node]
pub struct LocalFunction {
    #[clone]
    pub(crate) span: Span,
    pub(crate) name: Name,
    pub(crate) params: Vec<FunctionParam>,
    pub(crate) body: Block,
}

impl crate::Spanned for LocalFunction {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct LocalAssign {
    #[clone]
    pub(crate) span: Span,
    pub(crate) names: Vec<Name>,
    pub(crate) exprs: Option<Vec<Expr>>,
}

impl crate::Spanned for LocalAssign {
    fn span(&self) -> Option<satlight_common::location::Span> {
        Some(self.span)
    }
}
