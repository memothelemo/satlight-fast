mod cursor;
mod inner;
mod tokens;

pub use cursor::*;
pub use inner::*;
pub use tokens::*;

use satlight_ast::Token;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token<'a>> + '_ {
    let mut cursor = Tokenizer::new(input);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            Some(cursor.advance_token())
        }
    })
}
