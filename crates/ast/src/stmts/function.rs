use crate::{Block, FunctionParam, Name};
use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub struct FunctionAssignName {
    pub(crate) names: Vec<Name>,
    pub(crate) method_indexer: Option<Name>,
}

impl crate::Spanned for FunctionAssignName {
    fn span(&self) -> Option<Span> {
        Some(match &self.method_indexer {
            Some(n) => self.names.first()?.span().with_start(n.span().end()),
            None => self
                .names
                .first()?
                .span()
                .with_end(self.names.last()?.span().end()),
        })
    }
}

#[node]
pub struct FunctionAssign {
    pub(crate) span: Span,
    pub(crate) name: FunctionAssignName,
    pub(crate) params: Vec<FunctionParam>,
    pub(crate) body: Block,
}

impl crate::Spanned for FunctionAssign {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}
