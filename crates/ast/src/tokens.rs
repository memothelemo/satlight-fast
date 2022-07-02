use derive_more::IsVariant;
use satlight_common::location::Span;
use satlight_macros::{symbols, Getter};
use std::borrow::Cow;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

symbols! {
    KeywordType

    And => "and",
    Break => "break",
    Do => "do",
    Else => "else",
    ElseIf => "elseif",
    End => "end",
    False => "false",
    For => "for",
    Function => "function",
    If => "if",
    In => "in",
    Local => "local",
    Nil => "nil",
    Not => "not",
    Or => "or",
    Repeat => "repeat",
    Return => "return",
    Then => "then",
    True => "true",
    Until => "until",
    While => "while",
}

symbols! {
    SymbolType

    DotDotDot => "...",
    DotDot => "..",
    Dot => ".",

    SkinnyArrow => "->",

    GreaterEqual => ">=",
    LessEqual => "<=",
    EqualEqual => "==",
    TildeEqual => "~=",

    GreaterThan => ">",
    LessThan => "<",
    Equal => "=",

    OpenParen => "(",
    CloseParen => ")",

    OpenBracket => "[",
    CloseBracket => "]",

    OpenBrace => "{",
    CloseBrace => "}",

    Semicolon  => ";",
    ColonColon => "::",
    Colon => ":",
    Comma => ",",

    Cross => "+",
    Dash => "-",
    Star => "*",
    Slash => "/",
    Percent => "%",
    Caret => "^",
    Hash => "#",

    MetatableTag => "@metatable",
    Question => "?",

    VerticalBar => "|",
    Ampersand => "&",
}

#[derive(Debug, Clone, IsVariant, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InvalidType {
    UnexpectedCharacter(char),
    UnclosedComment,
    UnclosedString,
    InvalidShebang,
}

impl std::fmt::Display for InvalidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidType::UnexpectedCharacter(c) => write!(f, "unexpected character `{c}`"),
            InvalidType::UnclosedComment => write!(f, "unclosed comment"),
            InvalidType::UnclosedString => write!(f, "unclosed string"),
            InvalidType::InvalidShebang => write!(f, "invalid shebang"),
        }
    }
}

#[derive(Debug, Clone, IsVariant, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TokenType<'a> {
    Comment(Cow<'a, str>),
    Invalid(InvalidType),
    Keyword(KeywordType),
    Name(Cow<'a, str>),
    Number(Cow<'a, str>),
    Shebang(Cow<'a, str>),
    Symbol(SymbolType),
    Str(Cow<'a, str>),
    Trivia(Cow<'a, str>),
}

#[derive(Debug, Clone, Getter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Token<'a> {
    pub(crate) token_type: TokenType<'a>,
    #[clone]
    pub(crate) span: Span,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, span: Span) -> Self {
        Token { token_type, span }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_symbols() {
        assert_eq!(SymbolType::from_str("...").unwrap(), SymbolType::DotDotDot);
        assert_eq!(SymbolType::from_str("..").unwrap(), SymbolType::DotDot);
        assert_eq!(SymbolType::from_str(".").unwrap(), SymbolType::Dot);
        assert_eq!(SymbolType::from_str("=").unwrap(), SymbolType::Equal);
        assert_eq!(SymbolType::from_str("==").unwrap(), SymbolType::EqualEqual);
        assert!(SymbolType::from_str("").is_err());
    }
}
