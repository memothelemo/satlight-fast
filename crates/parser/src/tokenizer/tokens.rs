use super::Tokenizer;
use satlight_ast::Token;
use satlight_common::location::Position;

#[derive(Debug)]
pub struct Tokens<'a> {
    exclude_trivias: bool,
    lookahead: Option<Token<'a>>,
    current: Option<Token<'a>>,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Tokens<'a> {
    pub fn new(input: &'a str, exclude_trivias: bool) -> Self {
        Self {
            exclude_trivias,
            lookahead: None,
            current: None,
            tokenizer: Tokenizer::new(input),
        }
    }

    #[inline(always)]
    pub fn source(&self) -> &'a str {
        self.tokenizer.source()
    }

    #[inline(always)]
    pub fn position(&self) -> Position {
        self.tokenizer.position()
    }

    #[inline(always)]
    fn advance_token_inner(&mut self) -> Option<Token<'a>> {
        while !self.tokenizer.is_eof() {
            let token = self.tokenizer.advance_token();
            if token.token_type().is_trivia() && self.exclude_trivias {
                continue;
            } else {
                return Some(token);
            }
        }
        None
    }

    #[inline(always)]
    pub fn advance_token(&mut self) -> Option<Token<'a>> {
        if self.lookahead.is_some() {
            self.lookahead.take()
        } else {
            self.advance_token_inner()
        }
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Option<&Token<'a>> {
        self.lookahead = self.advance_token_inner();
        self.lookahead.as_ref()
    }

    #[inline(always)]
    pub fn current(&mut self) -> Option<&Token<'a>> {
        if self.current.is_none() {
            self.current = self.advance_token();
        }
        self.current.as_ref()
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(v) => Some(v),
            _ => self.advance_token(),
        }
    }
}
