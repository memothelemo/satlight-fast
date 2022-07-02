// NOTE: This code is so horrendous to look at.
//       but at least it works and somewhat performant.
//
// TODO: I will consider refactoring this in stable release?

use crate::{expect_err, expect_token, is_token, peek_kind, tokenizer::Tokens};
use ast::{InvalidType, KeywordType, Spanned, SymbolType, TokenType};
use derive_more::Display;
use satlight_ast as ast;
use satlight_common::location::{Position, Span};

#[cfg(test)]
mod tests;

mod macros;
pub use macros::*;

pub struct Parser<'a> {
    has_varargs: bool,
    tokens: Tokens<'a>,
    is_in_loop: bool,
}

#[derive(Debug, Display)]
pub enum ParseErrorMessage {
    #[display(fmt = "invalid usage of break")]
    InvalidBreakUse,
    #[display(fmt = "invalid usage of varargs")]
    InvalidVarargsUse,
    #[display(fmt = "expected {_0}")]
    Expected(String),
    #[display(fmt = "unexpected token {_0}")]
    UnexpectedToken(String),
    #[display(fmt = "{_0}")]
    Tokenization(InvalidType),
}

#[derive(Debug, Display)]
#[display(fmt = "{position}: {message}")]
pub struct ParseError {
    pub position: Position,
    pub message: ParseErrorMessage,
}

pub type ParseResult<T> = Result<T, Option<ParseError>>;

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens,
            has_varargs: false,
            is_in_loop: false,
        }
    }

    fn peek_further_token(&mut self) -> Result<Option<&ast::Token<'a>>, ParseError> {
        let src_ptr = (self as *const Parser<'a>) as *mut Parser<'a>;
        match self.tokens.peek() {
            Some(token) => match token.token_type() {
                TokenType::Invalid(invalid) => {
                    let invalid = invalid.clone();
                    Err(ParseError {
                        position: unsafe { src_ptr.read().tokens.position() },
                        message: ParseErrorMessage::Tokenization(invalid),
                    })
                }
                _ => Ok(Some(token)),
            },
            c => Ok(c),
        }
    }

    fn peek_token(&mut self) -> Result<Option<&ast::Token<'a>>, ParseError> {
        let src_ptr = (self as *const Parser<'a>) as *mut Parser<'a>;
        match self.tokens.current() {
            Some(token) => match token.token_type() {
                TokenType::Invalid(invalid) => {
                    let invalid = invalid.clone();
                    Err(ParseError {
                        position: unsafe { src_ptr.read().tokens.position() },
                        message: ParseErrorMessage::Tokenization(invalid),
                    })
                }
                _ => Ok(Some(token)),
            },
            c => Ok(c),
        }
    }

    fn next_token(&mut self) -> Result<Option<ast::Token<'a>>, ParseError> {
        match self.tokens.current() {
            Some(token) => match token.token_type() {
                TokenType::Invalid(invalid) => {
                    let invalid = invalid.clone();
                    Err(ParseError {
                        position: self.tokens.position(),
                        message: ParseErrorMessage::Tokenization(invalid),
                    })
                }
                _ => Ok(self.tokens.next()),
            },
            _ => Ok(self.tokens.next()),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn binop(&mut self) -> ParseResult<ast::Binop> {
        let kind = match peek_kind!(self, no_span) {
            Some(TokenType::Keyword(k)) => match k {
                KeywordType::And => ast::BinopKind::And,
                KeywordType::Or => ast::BinopKind::Or,
                _ => return Err(None),
            },
            Some(TokenType::Symbol(s)) => match s {
                SymbolType::Caret => ast::BinopKind::Exponent,
                SymbolType::Star => ast::BinopKind::Multiply,
                SymbolType::Slash => ast::BinopKind::Divide,
                SymbolType::Percent => ast::BinopKind::Modulo,
                SymbolType::Cross => ast::BinopKind::Add,
                SymbolType::Dash => ast::BinopKind::Subtract,
                SymbolType::DotDot => ast::BinopKind::Concat,
                SymbolType::EqualEqual => ast::BinopKind::Equality,
                SymbolType::TildeEqual => ast::BinopKind::Inequality,
                SymbolType::GreaterThan => ast::BinopKind::GreaterThan,
                SymbolType::GreaterEqual => ast::BinopKind::GreaterEqual,
                SymbolType::LessThan => ast::BinopKind::LessThan,
                SymbolType::LessEqual => ast::BinopKind::LessEqual,
                _ => return Err(None),
            },
            _ => return Err(None),
        };
        let token = self.peek_token()?.unwrap();
        Ok(ast::Binop::new(kind, token.span()))
    }

    pub fn unop(&mut self) -> ParseResult<ast::Unop> {
        let kind = match peek_kind!(self, no_span) {
            Some(TokenType::Keyword(k)) => match k {
                KeywordType::Not => ast::UnopKind::Not,
                _ => return Err(None),
            },
            Some(TokenType::Symbol(s)) => match s {
                SymbolType::Dash => ast::UnopKind::Negate,
                SymbolType::Hash => ast::UnopKind::Length,
                _ => return Err(None),
            },
            _ => return Err(None),
        };
        let token = self.next_token()?.unwrap();
        Ok(ast::Unop::new(kind, token.span()))
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_table(&mut self) -> ParseResult<ast::TableConstructor> {
        let start_brace = if let Some(token) = self.next_token()? {
            token.span()
        } else {
            expect_err!(self, "{")
        };
        let mut fields = Vec::new();
        loop {
            if is_token!(self, TokenType::Symbol(SymbolType::CloseBrace))
                || self.tokens.current().is_none()
            {
                break;
            }
            match peek_kind!(self) {
                Some((TokenType::Symbol(SymbolType::OpenBracket), start)) => {
                    self.next_token()?;
                    let expr = self.parse_expr()?;
                    if !is_token!(self, TokenType::Symbol(SymbolType::CloseBracket)) {
                        expect_err!(self, "]")
                    }
                    let closing = self.next_token()?.unwrap().span();
                    if !is_token!(self, TokenType::Symbol(SymbolType::Equal)) {
                        expect_err!(self, "=")
                    }
                    self.next_token()?;
                    let value = self.parse_expr()?;
                    fields.push(ast::TableField::Computed {
                        span: start.with_end(closing.end()),
                        key: Box::new(expr),
                        value: Box::new(value),
                    });
                }
                Some((TokenType::Name(n), span)) => {
                    // huge con about Lua
                    let n: ast::SmolStr = n.into();
                    if self
                        .peek_further_token()?
                        .map(|v| v.token_type() == &TokenType::Symbol(SymbolType::Equal))
                        .unwrap_or(false)
                    {
                        self.next_token()?;
                        self.next_token()?;
                        let value = self.parse_expr()?;
                        fields.push(ast::TableField::Named {
                            span: span.with_end(value.span().unwrap().end()),
                            key: ast::Name::new(span, n),
                            value: Box::new(value),
                        });
                    } else {
                        fields.push(ast::TableField::Value(Box::new(self.parse_expr()?)));
                    }
                }
                _ => fields.push(ast::TableField::Value(Box::new(self.parse_expr()?))),
            };

            match peek_kind!(self, no_span) {
                Some(TokenType::Symbol(SymbolType::Comma | SymbolType::Semicolon)) => {
                    self.next_token()?;
                }
                _ => break,
            }
        }
        if !is_token!(self, TokenType::Symbol(SymbolType::CloseBrace)) {
            expect_err!(self, "}")
        }
        let end_brace = self.tokens.next().unwrap().span();
        Ok(ast::TableConstructor::new(
            start_brace.with_end(end_brace.end()),
            fields,
        ))
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_name(&mut self) -> ParseResult<ast::Name> {
        if let Some(TokenType::Name(n)) = peek_kind!(self, no_span) {
            let n = n.clone();
            Ok(ast::Name::new(self.next_token()?.unwrap().span(), n.into()))
        } else {
            expect_err!(self, "<name>")
        }
    }

    /// Use this if you really need one or more expressions
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_exprlist(&mut self) -> ParseResult<Vec<ast::Expr>> {
        let mut exprs = vec![self.parse_expr()?];
        while let Some(TokenType::Symbol(SymbolType::Comma)) = peek_kind!(self, no_span) {
            self.next_token()?;
            let stop_loop = is_token!(self, TokenType::Symbol(SymbolType::DotDotDot));
            exprs.push(self.parse_expr()?);
            if stop_loop {
                break;
            }
        }
        Ok(exprs)
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_call_args_parens(&mut self, left_span: Span) -> ParseResult<ast::CallArgs> {
        self.next_token()?;
        let list =
            if let Some(TokenType::Symbol(SymbolType::CloseParen)) = peek_kind!(self, no_span) {
                Vec::new()
            } else {
                self.parse_exprlist()?
            };

        if !list.is_empty() && !is_token!(self, TokenType::Symbol(SymbolType::CloseParen)) {
            expect_err!(self, ")")
        }

        Ok(ast::CallArgs::Multiple {
            parens: left_span.with_end(self.next_token()?.unwrap().span().end()),
            exprs: list,
        })
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_call_args(&mut self) -> ParseResult<ast::CallArgs> {
        match peek_kind!(self) {
            Some((TokenType::Symbol(SymbolType::OpenParen), left_span)) => {
                self.parse_call_args_parens(left_span)
            }
            Some((TokenType::Symbol(SymbolType::OpenBrace), ..)) => {
                Ok(ast::CallArgs::Table(self.parse_table()?))
            }
            Some((TokenType::Str(str), span)) => {
                let str: ast::SmolStr = str.into();
                self.next_token()?;
                Ok(ast::CallArgs::Str(ast::Str::new(span, str.into())))
            }
            _ => expect_err!(self, "function arguments"),
        }
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_prefix_expr(&mut self) -> ParseResult<ast::Expr> {
        Ok(match peek_kind!(self) {
            Some((TokenType::Symbol(SymbolType::OpenParen), left_span)) => {
                self.next_token()?;
                let expr = self.parse_expr()?;
                if !is_token!(self, TokenType::Symbol(SymbolType::CloseParen)) {
                    expect_err!(self, ")")
                }
                let right_span = self.next_token()?.unwrap().span();
                ast::Expr::Parentheses(ast::Parentheses::new(
                    left_span.with_end(right_span.end()),
                    Box::new(expr),
                ))
            }
            Some((TokenType::Name(name), span)) => {
                let span = span.clone();
                let name: ast::SmolStr = name.into();
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Name(ast::Name::new(span, name.into())))
            }
            _ => expect_err!(self, "<expr>"),
        })
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_primary_expr(&mut self) -> ParseResult<ast::Expr> {
        let primary = self.parse_prefix_expr()?;
        let mut suffixes: Vec<ast::Suffix> = Vec::new();
        loop {
            match peek_kind!(self) {
                Some((TokenType::Symbol(SymbolType::OpenParen), left_span)) => {
                    suffixes.push(ast::Suffix::Call(ast::Call::Args(
                        self.parse_call_args_parens(left_span)?,
                    )));
                }
                Some((TokenType::Symbol(SymbolType::OpenBracket), left_span)) => {
                    self.next_token()?;
                    let expr = self.parse_expr()?;
                    if !is_token!(self, TokenType::Symbol(SymbolType::CloseBracket)) {
                        expect_err!(self, "]");
                    }
                    suffixes.push(ast::Suffix::Index(ast::SuffixIndex::Computed {
                        spanned: left_span.with_end(self.next_token()?.unwrap().span().end()),
                        expr: Box::new(expr),
                    }))
                }
                Some((TokenType::Symbol(SymbolType::Dot), dot)) => {
                    self.next_token()?;
                    let indexer = self.parse_name()?;
                    suffixes.push(ast::Suffix::Index(ast::SuffixIndex::Named { dot, indexer }));
                }
                Some((TokenType::Symbol(SymbolType::Colon), colon)) => {
                    self.next_token()?;
                    let indexer = self.parse_name()?;
                    suffixes.push(ast::Suffix::Call(ast::Call::Method(ast::MethodCall::new(
                        colon,
                        indexer,
                        Box::new(self.parse_call_args()?),
                    ))))
                }
                Some((TokenType::Str(str), span)) => {
                    let str: ast::SmolStr = str.into();
                    self.next_token()?;
                    suffixes.push(ast::Suffix::Call(ast::Call::Args(ast::CallArgs::Str(
                        ast::Str::new(span, str.into()),
                    ))));
                }
                Some((TokenType::Symbol(SymbolType::OpenBrace), ..)) => suffixes.push(
                    ast::Suffix::Call(ast::Call::Args(ast::CallArgs::Table(self.parse_table()?))),
                ),
                _ => break,
            }
        }
        if suffixes.is_empty() {
            Ok(primary)
        } else {
            Ok(ast::Expr::Suffixed(ast::Suffixed::new(
                Box::new(primary),
                suffixes,
            )))
        }
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_simple_expr(&mut self) -> ParseResult<ast::Expr> {
        Ok(match peek_kind!(self) {
            Some((TokenType::Str(str), span)) => {
                let str: ast::SmolStr = str.into();
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Str(ast::Str::new(span, str.into())))
            }
            Some((TokenType::Keyword(KeywordType::Nil), span)) => {
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Nil(span))
            }
            Some((TokenType::Keyword(KeywordType::True), span)) => {
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Bool(ast::Bool::new(span, true)))
            }
            Some((TokenType::Keyword(KeywordType::False), span)) => {
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Bool(ast::Bool::new(span, false)))
            }
            Some((TokenType::Keyword(KeywordType::Function), span)) => {
                self.next_token()?;
                ast::Expr::Literal(satlight_ast::Literal::Function(
                    self.parse_anynomous_function(span)?,
                ))
            }
            Some((TokenType::Symbol(SymbolType::OpenBrace), ..)) => {
                ast::Expr::Literal(ast::Literal::Table(self.parse_table()?))
            }
            Some((TokenType::Symbol(SymbolType::DotDotDot), span)) => {
                if self.has_varargs {
                    self.next_token()?;
                    ast::Expr::Literal(ast::Literal::Varargs(span))
                } else {
                    return Err(Some(ParseError {
                        position: self.tokens.position(),
                        message: ParseErrorMessage::InvalidVarargsUse,
                    }));
                }
            }
            Some((TokenType::Number(n), span)) => {
                let n: ast::SmolStr = n.into();
                self.next_token()?;
                ast::Expr::Literal(ast::Literal::Number(ast::Number::new(span, n)))
            }
            _ => return self.parse_primary_expr(),
        })
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_non_complex_expr(&mut self) -> ParseResult<ast::Expr> {
        Ok(match self.unop().ok() {
            Some(op) => {
                let expr = self.parse_complex_expr(op.kind().order())?;
                ast::Expr::Unary(ast::Unary::new(op, Box::new(expr)))
            }
            None => self.parse_simple_expr()?,
        })
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_complex_expr(&mut self, min_precedence: usize) -> ParseResult<ast::Expr> {
        let mut expr = self.parse_non_complex_expr()?;
        while let Some(binop) = self.binop().ok() {
            let kind = binop.kind();
            let order = kind.order();
            if order < min_precedence {
                break;
            }
            self.next_token()?;
            let is_right_associative = kind.is_right_associative();
            let right = self.parse_complex_expr(if is_right_associative {
                order
            } else {
                order + 1
            })?;
            expr = ast::Expr::Binary(ast::Binary::new(Box::new(expr), binop, Box::new(right)));
        }
        Ok(expr)
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_expr(&mut self) -> ParseResult<ast::Expr> {
        self.parse_complex_expr(1)
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_function_params(&mut self) -> ParseResult<Vec<ast::FunctionParam>> {
        let mut params = Vec::new();
        expect_token!(self, TokenType::Symbol(SymbolType::OpenParen), "(");
        if !is_token!(self, TokenType::Symbol(SymbolType::CloseParen)) {
            let mut current_param = self.parse_function_param()?;
            while is_token!(self, TokenType::Symbol(SymbolType::Comma)) {
                if matches!(current_param, ast::FunctionParam::Varargs(..)) {
                    break;
                }
                params.push(current_param);
                self.tokens.next();
                current_param = self.parse_function_param()?;
            }
            self.has_varargs = matches!(current_param, ast::FunctionParam::Varargs(..));
            params.push(current_param);
        } else {
            self.has_varargs = false;
        }
        expect_token!(self, TokenType::Symbol(SymbolType::CloseParen), ")");
        Ok(params)
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_anynomous_function(
        &mut self,
        start_span: Span,
    ) -> ParseResult<ast::AnynomousFunction> {
        let prev_varargs = self.has_varargs;
        let params = self.parse_function_params()?;
        let body = self.parse_block(false)?;
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        self.has_varargs = prev_varargs;
        Ok(ast::AnynomousFunction::new(
            start_span.with_start(body.span().end()),
            params,
            body,
        ))
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_name_list(&mut self) -> ParseResult<Vec<ast::Name>> {
        let mut names = vec![self.parse_name()?];
        while let Some(TokenType::Symbol(SymbolType::Comma)) = peek_kind!(self, no_span) {
            self.next_token()?;
            names.push(self.parse_name()?);
        }
        Ok(names)
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_value_assign_name(&mut self) -> ParseResult<ast::ValueAssignName> {
        match self.parse_primary_expr()? {
            ast::Expr::Suffixed(suffixed) => Ok(ast::ValueAssignName::Suffixed(suffixed)),
            ast::Expr::Literal(ast::Literal::Name(n)) => Ok(ast::ValueAssignName::Name(n)),
            _ => expect_err!(self, "assign name"),
        }
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_expr_stmt(&mut self) -> ParseResult<ast::Stmt> {
        let suffixed = self.parse_primary_expr()?;

        // assignment or call?
        let first_suffix = if let ast::Expr::Suffixed(suffixed) = suffixed {
            if let ast::Suffix::Call(call) = suffixed.suffixes().last().unwrap() {
                // TODO: calling clone is a bad idea
                return Ok(ast::Stmt::Call(call.clone()));
            };
            ast::ValueAssignName::Suffixed(suffixed)
        } else if let ast::Expr::Literal(ast::Literal::Name(n)) = suffixed {
            ast::ValueAssignName::Name(n)
        } else {
            expect_err!(self, "assign or call statement")
        };

        if !matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Symbol(SymbolType::Comma) | TokenType::Symbol(SymbolType::Equal))
        ) {
            expect_err!(self, "assign or call statement")
        }

        let start_span = first_suffix.span().unwrap();

        let mut names = vec![first_suffix];
        while matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Symbol(SymbolType::Comma))
        ) {
            self.tokens.next();
            names.push(self.parse_value_assign_name()?);
        }

        if !matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Symbol(SymbolType::Equal))
        ) {
            expect_err!(self, "=")
        }
        self.next_token()?;

        let rhs = self.parse_exprlist()?;
        Ok(ast::Stmt::ValueAssign(ast::ValueAssign::new(
            start_span.with_end(rhs.last().unwrap().span().unwrap().end()),
            names,
            rhs,
        )))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_local_assign(&mut self, span: Span) -> ParseResult<ast::LocalAssign> {
        let mut exprs = None;
        let names = self.parse_name_list()?;
        if matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Symbol(SymbolType::Equal))
        ) {
            self.next_token()?;
            exprs = Some(self.parse_exprlist()?);
        }

        let last_span = exprs
            .as_ref()
            .and_then(|v| v.last())
            .and_then(|v| v.span())
            .unwrap_or(names.last().unwrap().span());

        Ok(ast::LocalAssign::new(
            span.with_end(last_span.end()),
            names,
            exprs,
        ))
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn parse_function_param(&mut self) -> ParseResult<ast::FunctionParam> {
        if is_token!(self, TokenType::Symbol(SymbolType::DotDotDot)) {
            Ok(ast::FunctionParam::Varargs(
                self.tokens.next().unwrap().span(),
            ))
        } else {
            Ok(ast::FunctionParam::Name(self.parse_name()?))
        }
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_local_function(&mut self, span: Span) -> ParseResult<ast::LocalFunction> {
        self.next_token()?;

        let name = self.parse_name()?;

        let prev_varargs = self.has_varargs;
        let params = self.parse_function_params()?;

        let body = self.parse_block(false)?;
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        self.has_varargs = prev_varargs;

        Ok(ast::LocalFunction::new(
            span.with_start(body.span().end()),
            name,
            params,
            body,
        ))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_local_lookalikes(&mut self, span: Span) -> ParseResult<ast::Stmt> {
        if let Some(TokenType::Keyword(KeywordType::Function)) = peek_kind!(self, no_span) {
            Ok(ast::Stmt::LocalFunction(self.parse_local_function(span)?))
        } else {
            Ok(ast::Stmt::LocalAssign(self.parse_local_assign(span)?))
        }
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_generic_for(
        &mut self,
        span: Span,
        name: ast::Name,
    ) -> ParseResult<ast::GenericFor> {
        let mut names = vec![name];
        while is_token!(self, TokenType::Symbol(SymbolType::Comma)) {
            self.tokens.next();
            names.push(self.parse_name()?);
        }
        expect_token!(self, TokenType::Keyword(KeywordType::In), "in");
        let exprlist = self.parse_exprlist()?;
        expect_token!(self, TokenType::Keyword(KeywordType::Do), "do");
        let block = self.parse_block(true)?;
        let end_span = self.peek_token()?.map(|v| v.span());
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        Ok(ast::GenericFor::new(
            span.with_end(end_span.unwrap().end()),
            names,
            exprlist,
            block,
        ))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_numeric_for(
        &mut self,
        span: Span,
        name: ast::Name,
    ) -> ParseResult<ast::NumericFor> {
        self.next_token()?;
        let start = self.parse_expr()?;
        expect_token!(self, TokenType::Symbol(SymbolType::Comma), ",");
        let end = self.parse_expr()?;
        let step = if is_token!(self, TokenType::Symbol(SymbolType::Comma)) {
            self.next_token()?;
            Some(self.parse_expr()?)
        } else {
            None
        };
        expect_token!(self, TokenType::Keyword(KeywordType::Do), "do");
        let block = self.parse_block(true)?;
        let end_span = self.peek_token()?.map(|v| v.span());
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        Ok(ast::NumericFor::new(
            span.with_end(end_span.unwrap().end()),
            name,
            start,
            end,
            step,
            block,
        ))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_for_lookalikes(&mut self, span: Span) -> ParseResult<ast::Stmt> {
        let name = self.parse_name()?;
        match peek_kind!(self, no_span) {
            Some(TokenType::Symbol(SymbolType::Equal)) => {
                Ok(ast::Stmt::NumericFor(self.parse_numeric_for(span, name)?))
            }
            _ => Ok(ast::Stmt::GenericFor(self.parse_generic_for(span, name)?)),
        }
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_if_stmt_chain(&mut self) -> ParseResult<ast::IfStmtChain> {
        let span = self.tokens.next().unwrap().span();
        let condition = self.parse_expr()?;
        expect_token!(self, TokenType::Keyword(KeywordType::Then), "then");
        let block = self.parse_block(self.is_in_loop)?;
        Ok(ast::IfStmtChain::new(
            span.with_end(block.span().end()),
            condition,
            block,
        ))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_if_stmt(&mut self, start_span: Span) -> ParseResult<ast::IfStmt> {
        let condition = self.parse_expr()?;
        expect_token!(self, TokenType::Keyword(KeywordType::Then), "then");
        let block = self.parse_block(self.is_in_loop)?;
        let mut chains = Vec::new();
        while is_token!(self, TokenType::Keyword(KeywordType::ElseIf)) {
            chains.push(self.parse_if_stmt_chain()?);
        }
        let else_block = if is_token!(self, TokenType::Keyword(KeywordType::Else)) {
            self.next_token()?;
            Some(self.parse_block(self.is_in_loop)?)
        } else {
            None
        };
        let end_span = self.peek_token()?.map(|v| v.span());
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        Ok(ast::IfStmt::new(
            start_span.with_end(end_span.unwrap().end()),
            condition,
            block,
            chains,
            else_block,
        ))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_function_assign_name(&mut self) -> ParseResult<ast::FunctionAssignName> {
        let mut method_indexer = None;
        let mut names = vec![self.parse_name()?];
        loop {
            match peek_kind!(self, no_span) {
                Some(TokenType::Symbol(SymbolType::Colon)) => {
                    self.tokens.next();
                    method_indexer = Some(self.parse_name()?);
                    break;
                }
                Some(TokenType::Symbol(SymbolType::Dot)) => {
                    self.tokens.next();
                    names.push(self.parse_name()?);
                }
                _ => break,
            }
        }
        Ok(ast::FunctionAssignName::new(names, method_indexer))
    }

    #[inline(always)]
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_function_stmt(&mut self, start_span: Span) -> ParseResult<ast::FunctionAssign> {
        let name = self.parse_function_assign_name()?;
        let prev_varargs = self.has_varargs;
        let params = self.parse_function_params()?;
        let body = self.parse_block(false)?;
        let end_span = self.peek_token()?.map(|v| v.span());
        expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
        self.has_varargs = prev_varargs;

        Ok(ast::FunctionAssign::new(
            start_span.with_end(end_span.unwrap().end()),
            name,
            params,
            body,
        ))
    }
}

impl<'a> Parser<'a> {
    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_last_stmt(&mut self) -> ParseResult<ast::LastStmt> {
        match peek_kind!(self, no_span) {
            Some(TokenType::Keyword(KeywordType::Break)) => {
                if self.is_in_loop {
                    Ok(ast::LastStmt::Break(self.tokens.next().unwrap().span()))
                } else {
                    Err(Some(ParseError {
                        position: self.tokens.position(),
                        message: ParseErrorMessage::InvalidBreakUse,
                    }))
                }
            }
            Some(TokenType::Keyword(KeywordType::Return)) => {
                let token = self.tokens.next().unwrap().span();
                let exprlist = if is_token!(self, TokenType::Symbol(SymbolType::Semicolon))
                    || self.is_token_last_stmt_part()?
                {
                    Vec::new()
                } else {
                    self.parse_exprlist()?
                };
                Ok(ast::LastStmt::Return(ast::Return::new(token, exprlist)))
            }
            _ => Err(None),
        }
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    pub fn parse_stmt(&mut self) -> ParseResult<ast::Stmt> {
        let stmt = match peek_kind!(self, no_span) {
            Some(TokenType::Keyword(KeywordType::If)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                Ok(ast::Stmt::IfStmt(self.parse_if_stmt(span)?))
            }
            Some(TokenType::Keyword(KeywordType::While)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                let condition = self.parse_expr()?;
                expect_token!(self, TokenType::Keyword(KeywordType::Do), "do");
                let block = self.parse_block(true)?;
                let end_span = self.peek_token()?.map(|v| v.span());
                expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
                Ok(ast::Stmt::While(ast::WhileStmt::new(
                    span.with_end(end_span.unwrap().end()),
                    condition,
                    block,
                )))
            }
            Some(TokenType::Keyword(KeywordType::Do)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                let block = self.parse_block(self.is_in_loop)?;
                let end_span = self.peek_token()?.map(|v| v.span());
                expect_token!(self, TokenType::Keyword(KeywordType::End), "end");
                Ok(ast::Stmt::Do(ast::DoStmt::new(
                    span.with_end(end_span.unwrap().end()),
                    block,
                )))
            }
            Some(TokenType::Keyword(KeywordType::For)) => {
                let span = self.next_token()?.unwrap().span();
                self.parse_for_lookalikes(span)
            }
            Some(TokenType::Keyword(KeywordType::Repeat)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                let block = self.parse_block(true)?;
                expect_token!(self, TokenType::Keyword(KeywordType::Until), "until");
                let condition = self.parse_expr()?;
                Ok(ast::Stmt::Repeat(ast::RepeatStmt::new(
                    span.with_end(condition.span().unwrap().end()),
                    condition,
                    block,
                )))
            }
            Some(TokenType::Keyword(KeywordType::Function)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                Ok(ast::Stmt::FunctionAssign(self.parse_function_stmt(span)?))
            }
            Some(TokenType::Keyword(KeywordType::Local)) => {
                let span = self.peek_token()?.unwrap().span();
                self.tokens.next();
                self.parse_local_lookalikes(span)
            }
            _ => self.parse_expr_stmt(),
        }?;
        if is_token!(self, TokenType::Symbol(SymbolType::Semicolon)) {
            self.tokens.next();
        }
        Ok(stmt)
    }

    #[cfg_attr(feature = "full_profile", profiling::function)]
    fn is_token_last_stmt_part(&mut self) -> ParseResult<bool> {
        Ok(matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Keyword(
                KeywordType::Break
                    | KeywordType::End
                    | KeywordType::ElseIf
                    | KeywordType::Else
                    | KeywordType::Until
                    | KeywordType::Return
            )) | None
        ))
    }

    #[profiling::function]
    pub fn parse_block(&mut self, do_loop: bool) -> ParseResult<ast::Block> {
        let mut stmts = Vec::<ast::Stmt>::new();
        let last_loop = self.is_in_loop;
        self.is_in_loop = do_loop;
        let start = self.peek_token()?.map(|v| v.span().start()).unwrap_or(0);
        while !self.is_token_last_stmt_part()? {
            stmts.push(self.parse_stmt()?);
        }
        let last_stmt = if matches!(
            peek_kind!(self, no_span),
            Some(TokenType::Keyword(KeywordType::Break | KeywordType::Return))
        ) {
            let stmt = self.parse_last_stmt()?;
            if is_token!(self, TokenType::Symbol(SymbolType::Semicolon)) {
                self.tokens.next();
            }
            Some(stmt)
        } else {
            None
        };
        let end = self.peek_token()?.map(|v| v.span().start()).unwrap_or(0);
        self.is_in_loop = last_loop;
        Ok(ast::Block::new(Span::new(start, end), stmts, last_stmt))
    }

    #[profiling::function]
    pub fn parse_ast(&mut self) -> Result<ast::Ast, ParseError> {
        self.has_varargs = true;
        match self.parse_block(false) {
            // hold up! make sure it doesn't leave anything there!
            Ok(block) if self.peek_token()?.is_none() => Ok(ast::Ast::new(block)),
            Err(Some(err)) => Err(err),
            _ => {
                // getting the source of the token otherwise return <unknown>
                // maybe there's a bug?
                let span = self.tokens.current().map(|v| v.span()).unwrap_or_default();
                let source = self.tokens.source();
                let range = if source.is_empty() || span.end() > source.len() {
                    "<unknown>"
                } else {
                    &source[span.start()..span.end()]
                }
                .to_string();
                Err(ParseError {
                    position: self.tokens.position(),
                    message: ParseErrorMessage::UnexpectedToken(range),
                })
            }
        }
    }
}
