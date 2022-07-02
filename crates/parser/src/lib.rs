pub mod parser;
pub mod tokenizer;

use derive_more::Display;
use satlight_ast as ast;
use satlight_common as common;

use common::location::Position;

pub fn parse(input: &str) -> Result<ast::Ast, parser::ParseError> {
    let tokens = tokenizer::Tokens::new(input, true);
    let mut parser = parser::Parser::new(tokens);
    parser.parse_ast()
}

#[derive(Debug, Display)]
#[display(fmt = "{position}: {message}")]
pub struct TokenizeError {
    pub message: ast::InvalidType,
    pub position: Position,
}

pub fn tokens<'a>(
    input: &'a str,
    exclude_trivias: bool,
) -> Result<Vec<ast::Token<'a>>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut tokenizer = tokenizer::Tokens::new(input, exclude_trivias);
    while let Some(token) = tokenizer.next() {
        match token.token_type() {
            ast::TokenType::Invalid(message) => {
                return Err(TokenizeError {
                    message: message.clone(),
                    position: {
                        let mut col = 1;
                        let mut line = 1;
                        for c in input[..token.span().start()].chars() {
                            if c == '\n' {
                                col = 1;
                                line += 1;
                            } else {
                                col += 1;
                            }
                        }
                        Position::new(line, col, token.span().start())
                    },
                })
            }
            _ => {}
        };
        tokens.push(token);
    }
    Ok(tokens)
}
