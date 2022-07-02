use crate::{Expr, Name, Suffixed};
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub enum ValueAssignName {
    Name(Name),
    Suffixed(Suffixed),
}

impl crate::Spanned for ValueAssignName {
    fn span(&self) -> Option<satlight_common::location::Span> {
        match self {
            Self::Name(n) => Some(n.span),
            Self::Suffixed(n) => n.span(),
        }
    }
}

// you can modify tables wrapped in parentheses btw
// so I named this to ValueAssign instead of VarAssign
#[node]
pub struct ValueAssign {
    #[clone]
    pub(crate) span: Span,
    pub(crate) names: Vec<ValueAssignName>,
    pub(crate) exprlist: Vec<Expr>,
}

impl crate::Spanned for ValueAssign {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}
