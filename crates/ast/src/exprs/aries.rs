use crate::{Binop, Expr, Unop};
use satlight_macros::node;

#[node]
pub struct Unary {
    pub(crate) op: Unop,
    pub(crate) expr: Box<Expr>,
}

impl crate::Spanned for Unary {
    fn span(&self) -> Option<satlight_common::location::Span> {
        Some(self.op.span.with_end(self.expr.span()?.end()))
    }
}

#[node]
pub struct Binary {
    pub(crate) left: Box<Expr>,
    pub(crate) op: Binop,
    pub(crate) right: Box<Expr>,
}

impl crate::Spanned for Binary {
    fn span(&self) -> Option<satlight_common::location::Span> {
        Some(self.left.span()?.with_end(self.right.span()?.end()))
    }
}
