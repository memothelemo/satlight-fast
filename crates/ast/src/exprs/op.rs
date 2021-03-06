use satlight_common::location::Span;
use satlight_macros::{node, operators};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

operators! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum UnopKind {
        Length => 7,
        Not => 7,
        Negate => 7,
    }
}

operators! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BinopKind {
        Exponent => 8,
        Multiply => 6,
        Divide => 6,
        Modulo => 6,
        Add => 5,
        Subtract => 5,
        Concat => 4,
        Equality => 3,
        Inequality => 3,
        GreaterThan => 3,
        GreaterEqual => 3,
        LessThan => 3,
        LessEqual => 3,
        And => 2,
        Or => 1,
    }
}

impl BinopKind {
    pub fn is_right_associative(&self) -> bool {
        matches!(self, Self::Concat | Self::Exponent)
    }
}

#[node]
pub struct Binop {
    pub(crate) kind: BinopKind,
    #[clone]
    pub(crate) span: Span,
}

impl crate::Spanned for Binop {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}

#[node]
pub struct Unop {
    pub(crate) kind: UnopKind,
    #[clone]
    pub(crate) span: Span,
}

impl crate::Spanned for Unop {
    fn span(&self) -> Option<Span> {
        Some(self.span)
    }
}
