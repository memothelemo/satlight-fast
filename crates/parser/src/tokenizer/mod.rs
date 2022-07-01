mod cursor;
mod inner;

pub use cursor::*;
pub use inner::*;

use satlight_ast::Token;

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token<'a>> + '_ {
    let mut cursor = Tokenizer::new(input, false);
    std::iter::from_fn(move || {
        if cursor.is_eof() {
            None
        } else {
            Some(cursor.advance_token())
        }
    })
}
