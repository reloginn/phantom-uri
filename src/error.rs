use crate::parsing::lexer::token::kind::TokenKind;

#[derive(Debug)]
pub struct Unexpected(TokenKind, usize);

impl Unexpected {
    pub const fn new(kind: TokenKind, start: usize) -> Self {
        Self(kind, start)
    }
}

impl std::fmt::Display for Unexpected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected token `{:?}` at position {}", self.0, self.1)
    }
}

impl std::error::Error for Unexpected {}

#[derive(Debug)]
pub enum Error {
    InvalidSchemeCharacters,
    InvalidHostCharacters,
    InvalidPort,
    MissingHost,
    SchemeWithoutAuthority,
    UnexpectedToken(Unexpected),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let s = match self {
            InvalidSchemeCharacters => "invalid scheme characters",
            InvalidHostCharacters => "invalid host characters",
            InvalidPort => "invalid port",
            MissingHost => "missing host",
            SchemeWithoutAuthority => "scheme without authority",
            UnexpectedToken(err) => {
                write!(f, "{}", err)?;
                return Ok(());
            }
        };
        f.write_str(s)
    }
}

impl std::error::Error for Error {}

impl From<Unexpected> for Error {
    fn from(value: Unexpected) -> Self {
        Self::UnexpectedToken(value)
    }
}
