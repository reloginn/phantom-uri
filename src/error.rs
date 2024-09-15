use crate::parsing::lexer::token::kind::TokenKind;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Unexpected(TokenKind, usize);

impl Unexpected {
    pub const fn new(kind: TokenKind, start: usize) -> Self {
        Self(kind, start)
    }
}

impl std::fmt::Display for Unexpected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected token `{:?}` at position {}", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum ParseUriError {
    InvalidSchemeCharacters,
    InvalidHostCharacters,
    InvalidPort,
    MissingHost,
    SchemeWithoutAuthority,
    UnexpectedToken(Unexpected),
}

impl From<Unexpected> for ParseUriError {
    fn from(value: Unexpected) -> Self {
        Self::UnexpectedToken(value)
    }
}
