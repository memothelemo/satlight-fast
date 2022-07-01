use crate::Expr;
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub struct Return {
    pub(crate) token: Span,
    pub(crate) exprs: Vec<Expr>,
}

impl crate::Spanned for Return {
    fn span(&self) -> Option<Span> {
        Some(
            self.token.with_end(
                self.exprs
                    .last()
                    .and_then(|v| v.span())
                    .map(|v| v.end())
                    .unwrap_or(self.token.end()),
            ),
        )
    }
}
