use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Error {
    span: Span,
    message: String,
}

pub fn error(span: impl Into<Span>, message: impl Into<String>) -> Error {
    Error {
        span: span.into(),
        message: message.into(),
    }
}

pub type Result<V> = std::result::Result<V, Error>;
