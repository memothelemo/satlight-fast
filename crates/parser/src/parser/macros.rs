#[macro_export]
macro_rules! peek_kind {
    ($self:expr) => {
        $self.peek_token()?.map(|v| (v.token_type(), v.span()))
    };
    ($self:expr, no_span) => {
        $self.peek_token()?.map(|v| v.token_type())
    };
}

#[macro_export]
macro_rules! is_token {
    ($self:expr, $cmp:expr) => {
        $self
            .peek_token()?
            .map(|v| v.token_type() == &$cmp)
            .unwrap_or(false)
    };
}

#[macro_export]
macro_rules! expect_err {
    ($self:expr, $message:expr) => {{
        #[cfg(all(debug_assertions, not(test)))]
        {
            eprintln!("{:?}", $self.peek_token());
            eprintln!("backtrace: {:#?}", backtrace::Backtrace::new());
        }
        return Err(Some(ParseError {
            message: ParseErrorMessage::Expected($message.to_string()),
            position: $self.tokens.position(),
        }));
    }};
}

#[macro_export]
macro_rules! test_next {
    ($self:expr, $cmp:expr) => {
        if is_token!($self, $cmp) {
            $self.next_token()?;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! expect_token {
    ($self:expr, $kind:pat, $msg:expr) => {
        if !matches!(peek_kind!($self, no_span), Some(&$kind)) {
            expect_err!($self, $msg);
        }
        $self.tokens.next();
    };
}
