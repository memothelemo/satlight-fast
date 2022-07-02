use satlight_common::location::Span;
use satlight_macros::node;

use crate::{Expr, Name, Str, TableConstructor};

#[node]
pub enum SuffixIndex {
    Computed { spanned: Span, expr: Box<Expr> },
    Named { dot: Span, indexer: Name },
}

impl crate::Spanned for SuffixIndex {
    fn span(&self) -> Option<Span> {
        match self {
            SuffixIndex::Computed { spanned, .. } => Some(*spanned),
            SuffixIndex::Named { dot, indexer } => Some(dot.with_start(indexer.span().end())),
        }
    }
}

#[node]
pub enum CallArgs {
    Multiple { parens: Span, exprs: Vec<Expr> },
    Str(Str),
    Table(TableConstructor),
}

impl crate::Spanned for CallArgs {
    fn span(&self) -> Option<Span> {
        match self {
            CallArgs::Multiple { parens, .. } => Some(*parens),
            CallArgs::Str(str) => Some(str.span()),
            CallArgs::Table(n) => n.span(),
        }
    }
}

#[node]
pub struct MethodCall {
    pub(crate) colon: Span,
    pub(crate) indexer: Name,
    pub(crate) args: Box<CallArgs>,
}

impl crate::Spanned for MethodCall {
    fn span(&self) -> Option<Span> {
        Some(self.colon.with_end(self.args.span()?.end()))
    }
}

#[node]
pub enum Call {
    Args(CallArgs),
    Method(MethodCall),
}

impl crate::Spanned for Call {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Args(n) => n.span(),
            Self::Method(n) => n.span(),
        }
    }
}

#[node]
pub enum Suffix {
    Call(Call),
    Index(SuffixIndex),
}

impl crate::Spanned for Suffix {
    fn span(&self) -> Option<Span> {
        match self {
            Suffix::Call(n) => n.span(),
            Suffix::Index(n) => n.span(),
        }
    }
}

#[node]
pub struct Suffixed {
    pub(crate) prefix: Box<Expr>,
    pub(crate) suffixes: Vec<Suffix>,
}

impl crate::Spanned for Suffixed {
    fn span(&self) -> Option<Span> {
        Some(
            self.prefix
                .span()?
                .with_end(self.suffixes.last().unwrap().span()?.end()),
        )
    }
}
