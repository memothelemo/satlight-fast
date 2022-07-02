use super::Cursor;
use memchr::memchr;
use satlight_ast::{InvalidType, KeywordType, SymbolType, Token, TokenType};
use satlight_common::location::{Position, Span};
use std::{borrow::Cow, str::FromStr};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    cursor: Cursor<'a>,
}

macro_rules! symbol {
    ($ident:ident) => {{
        TokenType::Symbol(SymbolType::$ident)
    }};
    ($self:expr, $ident:ident) => {{
        $self.cursor.bump();
        TokenType::Symbol(SymbolType::$ident)
    }};
    ($self:expr, $ident:ident, $skips:expr) => {{
        $self.cursor.shift($skips);
        TokenType::Symbol(SymbolType::$ident)
    }};
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        self.cursor.position()
    }

    #[inline(always)]
    pub fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }

    #[inline(always)]
    pub fn source(&self) -> &'a str {
        self.cursor.input
    }

    pub fn advance_token(&mut self) -> Token<'a> {
        profiling::scope!("Tokenizer::advance_token");
        let start = self.cursor.offset();

        let bump = self.cursor.bump();
        let current = self.cursor.current();

        let token_type = match bump {
            b'=' => match current {
                b'=' => symbol!(self, EqualEqual),
                _ => symbol!(Equal),
            },

            b'~' => match current {
                b'=' => symbol!(self, TildeEqual),
                _ => TokenType::Invalid(InvalidType::UnexpectedCharacter('~')),
            },

            b'<' => match current {
                b'=' => symbol!(self, LessEqual),
                _ => symbol!(LessThan),
            },

            b'>' => match current {
                b'=' => symbol!(self, GreaterEqual),
                _ => symbol!(GreaterThan),
            },

            b'(' => symbol!(OpenParen),
            b')' => symbol!(CloseParen),

            b'{' => symbol!(OpenBrace),
            b'}' => symbol!(CloseBrace),

            b'[' => match current {
                b'=' | b'[' => self.multi_line_string(),
                _ => symbol!(OpenBracket),
            },
            b']' => symbol!(CloseBracket),

            b';' => symbol!(Semicolon),
            b':' => symbol!(Colon),

            b',' => symbol!(Comma),

            b'.' => match (current, self.cursor.peek()) {
                (b'0'..=b'9', ..) => self.number(),
                (b'.', b'.') => symbol!(self, DotDotDot, 1),
                (b'.', ..) => symbol!(self, DotDot, 0),
                _ => symbol!(Dot),
            },

            b'+' => symbol!(Cross),

            b'-' => match current {
                b'-' => self.comment(),
                _ => symbol!(Dash),
            },

            b'*' => symbol!(Star),
            b'/' => symbol!(Slash),
            b'%' => symbol!(Percent),
            b'^' => symbol!(Caret),

            b'#' => match current {
                b'!' if self.cursor.offset() == 1 => self.shebang(),
                _ => symbol!(Hash),
            },

            b' ' | b'\t' | b'\n' => self.whitespace(),
            b'\r' if current == b'\n' => self.whitespace(),

            b'_' => self.identifier(),
            b'a'..=b'z' | b'A'..=b'Z' => self.identifier(),

            b'0'..=b'9' => self.number(),

            c if matches!(c, b'\'' | b'"') => self.line_string(c),
            c => TokenType::Invalid(InvalidType::UnexpectedCharacter(c as char)),
        };

        Token::new(token_type, Span::new(start, self.cursor.offset()))
    }
}

macro_rules! is_lua_newline {
    ($ch:expr) => {
        matches!($ch, b'\r' | b'\n')
    };
}

macro_rules! is_numeral_digit {
    ($expr:expr) => {
        matches!($expr, b'0'..=b'9')
    };
}

macro_rules! is_hexadecimal_digit {
    ($expr:expr) => {
        matches!($expr, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    };
}

impl<'a> Tokenizer<'a> {
    fn skip_sep(&mut self) -> Option<usize> {
        debug_assert!(matches!(self.cursor.current(), b'[' | b']'));
        profiling::scope!("Tokenizer::skip_sep");

        let mut count = 0;
        let current = self.cursor.current();

        self.cursor.bump();
        self.cursor.eat_while(|v| {
            if v == b'=' {
                count += 1;
                true
            } else {
                false
            }
        });

        if current == self.cursor.current() {
            Some(count)
        } else {
            None
        }
    }

    fn read_long_string(&mut self, is_comment: bool, sep: usize) -> Result<usize, TokenType<'a>> {
        profiling::scope!("Tokenizer::read_long_string");
        loop {
            match self.cursor.current() {
                b'\0' => {
                    return if is_comment {
                        Err(TokenType::Invalid(InvalidType::UnclosedComment))
                    } else {
                        Err(TokenType::Invalid(InvalidType::UnclosedString))
                    };
                }
                _ => {
                    if let Some(index) = memchr(
                        b']',
                        &self.cursor.input[self.cursor.offset() - 1..].as_bytes(),
                    ) {
                        // no need to move things around..
                        self.cursor.mov(index.saturating_sub(1) as isize);

                        // quick bug fix for now...
                        if !matches!(self.cursor.current(), b']' | b'=') {
                            self.cursor.bump();
                            continue;
                        }

                        if let Some(new_sep) = self.skip_sep() {
                            if new_sep == sep {
                                let offset = self.cursor.offset() - 1 - new_sep;
                                return Ok(offset);
                            }
                        }
                    }
                }
            }
            // match self.cursor.current() {
            //     b'\0' => {

            //     }
            //     b']' => {
            //         if let Some(new_sep) = self.skip_sep() {
            //             if new_sep == sep {
            //                 let offset = self.cursor.offset() - 1 - new_sep;
            //                 return Ok(offset);
            //             }
            //         }
            //     }
            //     _ => {
            //         self.cursor.bump();
            //     }
            // }
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn number(&mut self) -> TokenType<'a> {
        debug_assert!(matches!(self.cursor.prev(), b'0'..=b'9' | b'.'));

        self.cursor.shift(-1);
        let start = self.cursor.offset();

        // branching is bad for performance per se.
        if self.cursor.current() == b'0' && self.cursor.peek() == b'x' {
            self.cursor.shift(1);
            let current = self.cursor.current();
            if !is_hexadecimal_digit!(current) {
                return TokenType::Invalid(InvalidType::UnexpectedCharacter(current as char));
            }
            self.cursor.eat_while(|v| is_hexadecimal_digit!(v));
        } else {
            self.cursor.eat_while(|v| is_numeral_digit!(v));
            if self.cursor.current() == b'.' {
                self.cursor.bump();
                let current = self.cursor.current();
                if !is_numeral_digit!(current) {
                    return TokenType::Invalid(InvalidType::UnexpectedCharacter(current as char));
                }
                self.cursor.bump();
                self.cursor.eat_while(|v| is_numeral_digit!(v));
            }
            if matches!(self.cursor.current(), b'e' | b'E') {
                // optional exponent sign
                if matches!(self.cursor.peek(), b'+' | b'-') {
                    self.cursor.shift(1);
                } else {
                    self.cursor.bump();
                }
                let current = self.cursor.current();
                if !is_numeral_digit!(current) {
                    return TokenType::Invalid(InvalidType::UnexpectedCharacter(current as char));
                }
                self.cursor.eat_while(|v| is_numeral_digit!(v));
            }
        }

        TokenType::Number(Cow::Borrowed(
            &self.cursor.input[start..self.cursor.offset()],
        ))
    }

    fn line_string(&mut self, delim: u8) -> TokenType<'a> {
        debug_assert!(matches!(self.cursor.prev(), b'"' | b'\''));
        let start = self.cursor.offset();
        loop {
            match self.cursor.current() {
                0 | b'\n' | b'\r' => {
                    return TokenType::Invalid(InvalidType::UnclosedString);
                }
                b'\\' => {
                    // TODO: string escape check
                    self.cursor.shift(1);
                }
                c if c == delim => {
                    let end = self.cursor.offset();
                    self.cursor.bump();
                    return TokenType::Str(Cow::Borrowed(&self.cursor.input[start..end]));
                }
                _ => {
                    self.cursor.bump();
                }
            };
        }
    }

    fn multi_line_string(&mut self) -> TokenType<'a> {
        debug_assert_eq!(self.cursor.prev(), b'[');
        debug_assert!(matches!(self.cursor.current(), b'=' | b'['));

        self.cursor.shift(-1);

        if let Some(sep) = self.skip_sep() {
            self.cursor.bump();
            let start = self.cursor.offset();
            match self.read_long_string(false, sep) {
                Ok(end) => {
                    self.cursor.bump();
                    return TokenType::Str(Cow::Borrowed(&self.cursor.input[start..end]));
                }
                Err(err) => return err,
            }
        } else {
            TokenType::Invalid(InvalidType::UnclosedString)
        }
    }

    fn comment(&mut self) -> TokenType<'a> {
        debug_assert_eq!(self.cursor.prev(), b'-');
        debug_assert_eq!(self.cursor.current(), b'-');
        profiling::scope!("Tokenizer::comment");

        self.cursor.bump();

        let start = self.cursor.offset();
        match self.cursor.current() {
            b'[' => {
                if let Some(sep) = self.skip_sep() {
                    self.cursor.bump();
                    let start = self.cursor.offset();
                    match self.read_long_string(true, sep) {
                        Ok(end) => {
                            self.cursor.bump();
                            return TokenType::Trivia(Cow::Borrowed(
                                &self.cursor.input[start..end],
                            ));
                        }
                        Err(err) => return err,
                    }
                }
            }
            _ => {}
        };

        if let Some(index) = memchr(
            b'\n',
            &self.cursor.input[self.cursor.offset() - 1..].as_bytes(),
        ) {
            self.cursor.mov((index - 1) as isize);
        } else {
            self.cursor
                .mov((self.cursor.length - self.cursor.offset()) as isize);
        }

        // self.cursor.eat_while(|v| !is_lua_newline!(v));
        TokenType::Trivia(Cow::Borrowed(
            &self.cursor.input[start..self.cursor.offset()],
        ))
    }

    fn shebang(&mut self) -> TokenType<'a> {
        debug_assert_eq!(self.cursor.prev(), b'#');
        debug_assert_eq!(self.cursor.current(), b'!');
        profiling::scope!("Tokenizer::shebang");

        self.cursor.bump();

        let start = self.cursor.offset();
        self.cursor.eat_while(|v| !is_lua_newline!(v));

        TokenType::Trivia(Cow::Borrowed(
            &self.cursor.input[start..self.cursor.offset()],
        ))
    }

    fn identifier(&mut self) -> TokenType<'a> {
        profiling::scope!("Tokenizer::identifier");
        debug_assert!(matches!(self.cursor.prev(), b'_' | b'a'..=b'z' | b'A'..=b'Z'));

        let start = self.cursor.offset() - 1;
        self.cursor
            .eat_while(|v| matches!(v, b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'));

        let str = &self.cursor.input[start..self.cursor.offset()];
        if let Ok(sym) = KeywordType::from_str(str) {
            TokenType::Keyword(sym)
        } else {
            TokenType::Name(Cow::Borrowed(str))
        }
    }

    fn whitespace(&mut self) -> TokenType<'a> {
        profiling::scope!("Tokenizer::whitespace");
        debug_assert!(matches!(self.cursor.prev(), b' ' | b'\t' | b'\n' | b'\r'));

        let start = self.cursor.offset() - 1;

        // unlike most other languages, Lua expects <CR> must be beside <LF>
        loop {
            match self.cursor.current() {
                b' ' | b'\t' | b'\n' => self.cursor.bump(),
                b'\r' => match self.cursor.peek() {
                    b'\n' => self.cursor.shift(1),
                    _ => return TokenType::Invalid(InvalidType::UnexpectedCharacter('\r')),
                },
                _ => break,
            };
        }

        TokenType::Trivia(Cow::Borrowed(
            &self.cursor.input[start..self.cursor.offset()],
        ))
    }
}
