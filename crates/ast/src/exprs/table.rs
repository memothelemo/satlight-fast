use super::*;

use satlight_common::location::Span;
use satlight_macros::node;

#[node]
pub enum TableField {
    Computed {
        span: Span,
        key: Box<Expr>,
        value: Box<Expr>,
    },
    Named {
        span: Span,
        key: Name,
        value: Box<Expr>,
    },
    Value(Box<Expr>),
}

impl crate::Spanned for TableField {
    fn span(&self) -> Option<Span> {
        Some(match self {
            TableField::Computed { span, .. } => *span,
            TableField::Named { span, .. } => *span,
            TableField::Value(value) => return value.span(),
        })
    }
}

#[node]
pub struct TableConstructor {
    pub(crate) braces: Span,
    pub(crate) fields: Vec<TableField>,
}

impl crate::Spanned for TableConstructor {
    fn span(&self) -> Option<Span> {
        Some(self.braces)
    }
}
