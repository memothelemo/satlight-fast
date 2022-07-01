#!/usr/bin
use crate::tokenizer::tokenize;
use satlight_ast::{InvalidType, SymbolType, Token, TokenType};
use satlight_common::location::Span;
use std::borrow::Cow;

macro_rules! token {
    ($typ:expr, $start:expr, $end:expr) => {
        Token::new($typ, Span::new($start, $end))
    };
}
macro_rules! expect_tok {
    ($tokens:expr, $token:expr) => {
        assert_eq!($tokens.next(), Some($token));
    };
}
macro_rules! expect_seq {
    { tokens = $tokens:expr, $( $token:expr, )* } => { $( expect_tok!($tokens, $token); )* };
}

mod scripts {
    use super::*;

    macro_rules! test_script {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let contents = include_str!(concat!("./scripts/", $path));
                for token in tokenize(contents) {
                    if token.token_type().is_invalid() {
                        panic!("{:#?}", token);
                    }
                }
            }
        };
    }

    test_script!(luaminify, "luaminify.txt");
    test_script!(profile_service, "profileservice.txt");
}

#[test]
fn two_letter_symbols() {
    let mut tokens = tokenize("==~=<=>=..~=");
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Symbol(SymbolType::EqualEqual), 0, 2),
        token!(TokenType::Symbol(SymbolType::TildeEqual), 2, 4),
        token!(TokenType::Symbol(SymbolType::LessEqual), 4, 6),
        token!(TokenType::Symbol(SymbolType::GreaterEqual), 6, 8),
        token!(TokenType::Symbol(SymbolType::DotDot), 8, 10),
        token!(TokenType::Symbol(SymbolType::TildeEqual), 10, 12),
    }
}

#[test]
fn disallowed_numbers() {
    let mut tokens = tokenize(r#"1."#);
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Invalid(InvalidType::UnexpectedCharacter('\0')), 0, 2),
    }
    let mut tokens = tokenize(r#"0x"#);
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Invalid(InvalidType::UnexpectedCharacter('\0')), 0, 2),
    }
    let mut tokens = tokenize(r#"1e"#);
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Invalid(InvalidType::UnexpectedCharacter('\0')), 0, 2),
    }
}

#[test]
fn numbers() {
    let mut tokens = tokenize(r#"123 123.123 .123 123e123 123e+123 123.123e+123 0x12AB"#);
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Number(Cow::Owned("123".to_string())), 0, 3),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 3, 4),
        token!(TokenType::Number(Cow::Owned("123.123".to_string())), 4, 11),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 11, 12),
        token!(TokenType::Number(Cow::Owned(".123".to_string())), 12, 16),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 16, 17),
        token!(TokenType::Number(Cow::Owned("123e123".to_string())), 17, 24),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 24, 25),
        token!(TokenType::Number(Cow::Owned("123e+123".to_string())), 25, 33),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 33, 34),
        token!(TokenType::Number(Cow::Owned("123.123e+123".to_string())), 34, 46),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 46, 47),
        token!(TokenType::Number(Cow::Owned("0x12AB".to_string())), 47, 53),
    }
}

#[test]
fn line_strings() {
    let mut tokens = tokenize(r#""Hello world" 'Hi' "\"" '"#);
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Str(Cow::Owned("Hello world".to_string())), 0, 13),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 13, 14),
        token!(TokenType::Str(Cow::Owned("Hi".to_string())), 14, 18),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 18, 19),
        token!(TokenType::Str(Cow::Owned("\\\"".to_string())), 19, 23),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 23, 24),
        token!(TokenType::Invalid(InvalidType::UnclosedString), 24, 25),
    }
    let mut tokens = tokenize("\"\\\n\"");
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Str(Cow::Owned("\\\n".to_string())), 0, 4),
    };
}

#[test]
fn dot_symbols() {
    let mut tokens = tokenize("...,..,.");
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Symbol(SymbolType::DotDotDot), 0, 3),
        token!(TokenType::Symbol(SymbolType::Comma), 3, 4),
        token!(TokenType::Symbol(SymbolType::DotDot), 4, 6),
        token!(TokenType::Symbol(SymbolType::Comma), 6, 7),
        token!(TokenType::Symbol(SymbolType::Dot), 7, 8),
    }
}

#[test]
fn multi_line_strings() {
    let mut tokens = tokenize("[[]][[foo]][==[hi]==][==[==[]==][==[]===]==][[]");
    expect_seq!(
        tokens = tokens,
        token!(TokenType::Str(Cow::Owned("".to_string())), 0, 4),
        token!(TokenType::Str(Cow::Owned("foo".to_string())), 4, 11),
        token!(TokenType::Str(Cow::Owned("hi".to_string())), 11, 21),
        token!(TokenType::Str(Cow::Owned("==[".to_string())), 21, 32),
        token!(TokenType::Str(Cow::Owned("]===".to_string())), 32, 44),
        token!(TokenType::Invalid(InvalidType::UnclosedString), 44, 47),
    );
}

#[test]
fn multi_line_comments() {
    let mut tokens = tokenize("--[[]]--[[foo]]--[==[hi]==]--[==[==[]==]--[==[]===]==]--[[");
    expect_seq!(
        tokens = tokens,
        token!(TokenType::Trivia(Cow::Owned("".to_string())), 0, 6),
        token!(TokenType::Trivia(Cow::Owned("foo".to_string())), 6, 15),
        token!(TokenType::Trivia(Cow::Owned("hi".to_string())), 15, 27),
        token!(TokenType::Trivia(Cow::Owned("==[".to_string())), 27, 40),
        token!(TokenType::Trivia(Cow::Owned("]===".to_string())), 40, 54),
        token!(TokenType::Invalid(InvalidType::UnclosedComment), 54, 58),
    );
}

#[test]
fn single_line_comments() {
    let mut tokens = tokenize("--\n--hello\n--[\n--[==");
    expect_seq!(
        tokens = tokens,
        token!(TokenType::Trivia(Cow::Owned("".to_string())), 0, 2),
        token!(TokenType::Trivia(Cow::Owned("\n".to_string())), 2, 3),
        token!(TokenType::Trivia(Cow::Owned("hello".to_string())), 3, 10),
        token!(TokenType::Trivia(Cow::Owned("\n".to_string())), 10, 11),
        token!(TokenType::Trivia(Cow::Owned("[".to_string())), 11, 14),
        token!(TokenType::Trivia(Cow::Owned("\n".to_string())), 14, 15),
        token!(TokenType::Trivia(Cow::Owned("[==".to_string())), 15, 20),
    );
}

#[test]
fn identifiers() {
    let mut tokens = tokenize("_foo bar f1 b2_ _1 _1a_2");
    expect_seq!(
        tokens = tokens,
        token!(TokenType::Name(Cow::Owned("_foo".to_string())), 0, 4),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 4, 5),
        token!(TokenType::Name(Cow::Owned("bar".to_string())), 5, 8),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 8, 9),
        token!(TokenType::Name(Cow::Owned("f1".to_string())), 9, 11),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 11, 12),
        token!(TokenType::Name(Cow::Owned("b2_".to_string())), 12, 15),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 15, 16),
        token!(TokenType::Name(Cow::Owned("_1".to_string())), 16, 18),
        token!(TokenType::Trivia(Cow::Owned(" ".to_string())), 18, 19),
        token!(TokenType::Name(Cow::Owned("_1a_2".to_string())), 19, 24),
    );
}

#[test]
fn shebang() {
    let mut tokens = tokenize("#!/usr/bin/lua");
    expect_seq!(
        tokens = tokens,
        token!(
            TokenType::Trivia(Cow::Owned("/usr/bin/lua".to_string())),
            0,
            14
        ),
    );
}

#[test]
fn whitespaces() {
    let mut tokens = tokenize("\r\n \t\n");
    expect_seq!(
        tokens = tokens,
        token!(TokenType::Trivia(Cow::Owned("\r\n \t\n".to_string())), 0, 5),
    );
}

#[test]
fn disallowed_whitespace() {
    let mut tokens = tokenize("\r");
    expect_seq!(
        tokens = tokens,
        token!(
            TokenType::Invalid(InvalidType::UnexpectedCharacter('\r')),
            0,
            1
        ),
    );
}

#[test]
fn one_letter_symbols() {
    let mut tokens = tokenize("(){}[]<>;:.+-*/%^#");
    expect_seq! {
        tokens = tokens,
        token!(TokenType::Symbol(SymbolType::OpenParen), 0, 1),
        token!(TokenType::Symbol(SymbolType::CloseParen), 1, 2),
        token!(TokenType::Symbol(SymbolType::OpenBrace), 2, 3),
        token!(TokenType::Symbol(SymbolType::CloseBrace), 3, 4),
        token!(TokenType::Symbol(SymbolType::OpenBracket), 4, 5),
        token!(TokenType::Symbol(SymbolType::CloseBracket), 5, 6),
        token!(TokenType::Symbol(SymbolType::LessThan), 6, 7),
        token!(TokenType::Symbol(SymbolType::GreaterThan), 7, 8),
        token!(TokenType::Symbol(SymbolType::Semicolon), 8, 9),
        token!(TokenType::Symbol(SymbolType::Colon), 9, 10),
        token!(TokenType::Symbol(SymbolType::Dot), 10, 11),
        token!(TokenType::Symbol(SymbolType::Cross), 11, 12),
        token!(TokenType::Symbol(SymbolType::Dash), 12, 13),
        token!(TokenType::Symbol(SymbolType::Star), 13, 14),
        token!(TokenType::Symbol(SymbolType::Slash), 14, 15),
        token!(TokenType::Symbol(SymbolType::Percent), 15, 16),
        token!(TokenType::Symbol(SymbolType::Caret), 16, 17),
        token!(TokenType::Symbol(SymbolType::Hash), 17, 18),
    }
}
