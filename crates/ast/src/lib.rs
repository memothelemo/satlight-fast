// TODO: Add support for lossless parsing and ast

mod ast;
mod exprs;
mod stmts;
mod tokens;

pub use smol_str::SmolStr;

pub use ast::*;
pub use exprs::*;
pub use stmts::*;
pub use tokens::*;

use satlight_common::location::Span;

pub trait Spanned {
    fn span(&self) -> Option<Span>;
}
