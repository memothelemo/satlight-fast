use crate::{Block, FunctionParam, Spanned, TableConstructor};
use satlight_common::location::Span;
use satlight_macros::node;
use smol_str::SmolStr;

#[node]
pub struct Bool {
    #[clone]
    pub(crate) span: Span,
    pub(crate) value: bool,
}

#[node]
pub struct AnynomousFunction {
    #[clone]
    pub(crate) span: Span,
    pub(crate) params: Vec<FunctionParam>,
    pub(crate) body: Block,
}

impl Spanned for AnynomousFunction {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

impl Spanned for Bool {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct Name {
    #[clone]
    pub(crate) span: Span,
    pub(crate) name: SmolStr,
}

impl Spanned for Name {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct Number {
    #[clone]
    pub(crate) span: Span,
    pub(crate) value: SmolStr,
}

impl Spanned for Number {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct Str {
    #[clone]
    pub(crate) span: Span,
    pub(crate) value: SmolStr,
}

impl Spanned for Str {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub enum Literal {
    Bool(Bool),
    Function(AnynomousFunction),
    Name(Name),
    Nil(Span),
    Number(Number),
    Str(Str),
    Table(TableConstructor),
    Varargs(Span),
}

impl crate::Spanned for Literal {
    fn span(&self) -> Option<Span> {
        match self {
            Literal::Bool(n) => Some(n.span()),
            Literal::Function(n) => Some(n.span),
            Literal::Name(n) => Some(n.span()),
            Literal::Nil(n) => Some(*n),
            Literal::Number(n) => Some(n.span()),
            Literal::Str(n) => Some(n.span()),
            Literal::Table(n) => n.span(),
            Literal::Varargs(n) => Some(*n),
        }
    }
}
