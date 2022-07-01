mod aries;
mod literal;
mod op;
mod parentheses;
mod suffix;

pub use aries::*;
pub use literal::*;
pub use op::*;
pub use parentheses::*;
pub use suffix::*;

use satlight_macros::node;

#[node]
pub enum Expr {
    Binary(Binary),
    Literal(Literal),
    Parentheses(Parentheses),
    Suffixed(Suffixed),
    Unary(Unary),
}

impl crate::Spanned for Expr {
    fn span(&self) -> Option<satlight_common::location::Span> {
        match self {
            Expr::Binary(n) => n.span(),
            Expr::Literal(n) => n.span(),
            Expr::Parentheses(n) => n.span(),
            Expr::Suffixed(n) => n.span(),
            Expr::Unary(n) => n.span(),
        }
    }
}
