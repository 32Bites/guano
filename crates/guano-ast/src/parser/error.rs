use super::token_stream::Span;

pub type ParseResult<T, E> = Result<T, ParseError<E>>;

#[macro_export]
macro_rules! empty_error {
    ($t:tt, $v:vis) => {
        #[derive(Debug)]
        $v struct $t;

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("empty error, this is a placeholder, this should not be visible")
            }
        }

        impl std::error::Error for $t {}
    };

    ($t:tt) => {
        empty_error!($t, pub);
    };
}

empty_error!(EmptyError);

#[derive(Debug)]
pub enum ParseError<E: std::error::Error> {
    Provided(Option<E>, Span),
    EndOfFile,
}

impl<E: std::error::Error> ParseError<E> {
    pub fn spanned(error: Option<E>, span: Span) -> Self {
        Self::Provided(error, span)
    }

    pub fn unspanned(error: Option<E>) -> Self {
        Self::Provided(error, None)
    }

    pub fn eof() -> Self {
        Self::EndOfFile
    }

    pub fn unexpected_token(span: Span) -> Self {
        Self::Provided(None, span)
    }

    pub fn span(&self) -> Span {
        match self {
            ParseError::Provided(_, span) => span.clone(),
            _ => None,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            ParseError::Provided(error, _) => error
                .as_ref()
                .map_or("unexpected token".to_string(), |e| e.to_string()),
            ParseError::EndOfFile => "unexpected end of file".to_string(),
        }
    }

    pub fn convert<T: std::error::Error + From<E>>(self) -> ParseError<T> {
        match self {
            ParseError::Provided(error, span) => {
                ParseError::Provided(error.map(|e| e.into()), span)
            }
            ParseError::EndOfFile => ParseError::EndOfFile,
        }
    }

    pub fn convert_boxed<T: std::error::Error + From<Box<E>>>(self) -> ParseError<T> {
        match self {
            ParseError::Provided(error, span) => {
                ParseError::Provided(error.map(|e| Box::new(e).into()), span)
            }
            ParseError::EndOfFile => ParseError::EndOfFile,
        }
    }
}

impl<E: std::error::Error> std::fmt::Display for ParseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = self.error_message();
        match self {
            ParseError::Provided(_, span) => write!(f, "{error} at byte range {:?}", span),
            ParseError::EndOfFile => write!(f, "{error}"),
        }
    }
}

impl<E: std::error::Error> std::error::Error for ParseError<E> {}

pub trait ToParseError<E: std::error::Error>: Sized {
    fn to_parse_error(self, span: Span) -> ParseError<E>;
}

impl<E: std::error::Error> ToParseError<E> for E {
    fn to_parse_error(self, span: Span) -> ParseError<E> {
        ParseError::spanned(Some(self), span)
    }
}

pub trait ToParseResult<E: std::error::Error, T> {
    fn to_parse_result(self) -> ParseResult<T, E>;
}

impl<E: std::error::Error, T> ToParseResult<E, T> for ParseError<E> {
    fn to_parse_result(self) -> ParseResult<T, E> {
        Err(self)
    }
}

impl<O: std::error::Error, E: std::error::Error + From<O>, T> ToParseResult<E, T>
    for ParseResult<T, O>
{
    fn to_parse_result(self) -> ParseResult<T, E> {
        self.map_err(|p| p.convert())
    }
}
