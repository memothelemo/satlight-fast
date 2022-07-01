use crate::Expr;
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub struct Parentheses {
    pub(crate) spanned: Span,
    pub(crate) inner: Box<Expr>,
}

impl crate::Spanned for Parentheses {
    fn span(&self) -> Option<Span> {
        Some(self.spanned)
    }
}
