use satlight_common::location::Span;
use satlight_macros::node;

mod assign;
mod conditional;
mod function;
mod last;
mod local;

pub use assign::*;
pub use conditional::*;
pub use function::*;
pub use last::*;
pub use local::*;

use crate::{Block, Name};

#[node]
pub enum FunctionParam {
    Name(Name),
    Varargs(Span),
}

impl crate::Spanned for FunctionParam {
    fn span(&self) -> Option<Span> {
        match self {
            FunctionParam::Name(n) => Some(n.span),
            FunctionParam::Varargs(n) => Some(*n),
        }
    }
}

#[node]
pub struct DoStmt {
    pub(crate) span: Span,
    pub(crate) block: Block,
}

impl crate::Spanned for DoStmt {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub enum Stmt {
    Call(super::Call),
    Do(DoStmt),
    IfStmt(IfStmt),
    GenericFor(GenericFor),
    FunctionAssign(FunctionAssign),
    LocalAssign(LocalAssign),
    LocalFunction(LocalFunction),
    NumericFor(NumericFor),
    Repeat(RepeatStmt),
    While(WhileStmt),
    ValueAssign(ValueAssign),
}

impl crate::Spanned for Stmt {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Call(n) => n.span(),
            Self::Do(n) => Some(n.span),
            Self::FunctionAssign(n) => Some(n.span),
            Self::GenericFor(n) => Some(n.span()),
            Self::IfStmt(n) => Some(n.span()),
            Self::LocalAssign(n) => Some(n.span),
            Self::LocalFunction(n) => Some(n.span()),
            Self::NumericFor(n) => Some(n.span()),
            Self::Repeat(n) => Some(n.span()),
            Self::While(n) => Some(n.span()),
            Self::ValueAssign(n) => Some(n.span()),
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
