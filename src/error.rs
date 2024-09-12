use crate::parsing::lexer::token::kind::TokenKind;

#[derive(Debug)]
pub enum ParseUriError {
    InvalidSchemeCharacters,
    InvalidHostCharacters,
    InvalidPort,
    MissingHost,
    SchemeWithoutAuthority,
    UnexpectedToken(TokenKind),
}
