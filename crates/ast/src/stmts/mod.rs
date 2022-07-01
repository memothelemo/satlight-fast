use satlight_common::location::Span;
use satlight_macros::node;

mod last;

pub use last::*;

#[node]
pub enum Stmt {
    Semicolon(Span),
}

impl crate::Spanned for Stmt {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Semicolon(n) => Some(*n),
        }
    }
}

#[node]
pub enum LastStmt {
    Break(Span),
    Return(Return),
}

impl crate::Spanned for LastStmt {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Break(n) => Some(*n),
            Self::Return(n) => n.span(),
        }
    }
}
