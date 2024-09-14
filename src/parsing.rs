pub mod lexer;

use crate::parsing::lexer::token::span::Span;
use crate::{
    error::{ParseUriError, Unexpected},
    parsing::lexer::{
        token::{kind::TokenKind, Token},
        Lexer,
    },
    Authority, Uri,
};
use std::collections::VecDeque;

pub struct UriParser {
    input: String,
    tokens: VecDeque<Token>,
}

impl UriParser {
    pub fn new(input: &str) -> Result<Self, Unexpected> {
        let mut input = input.to_owned();
        input.make_ascii_lowercase();
        let tokens = Lexer::new(&input).tokenize()?;
        Ok(Self {
            input,
            tokens: tokens.into(),
        })
    }

    pub fn parse(mut self) -> Result<Uri, ParseUriError> {
        let scheme = self.parse_scheme()?;
        let authority = self.parse_authority()?;
        let path = self.parse_path();
        let query = self.parse_query()?;
        let fragment = self.parse_fragment()?;

        if scheme.is_some() && authority.is_none() {
            return Err(ParseUriError::SchemeWithoutAuthority);
        }

        Ok(Uri {
            input: self.input,
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }

    fn parse_scheme(&mut self) -> Result<Option<Span>, ParseUriError> {
        if let Some(Token {
            kind: TokenKind::Ident(span),
            ..
        }) = self.tokens.front()
        {
            if self.peek_next_token(TokenKind::Colon) {
                let span = span.to_owned();
                let start = span.start();
                let length = span.length();
                let scheme = &self.input[start..start + length];
                self.tokens.pop_front();
                self.tokens.pop_front();
                return if is_valid_scheme(scheme) {
                    Ok(Some(span))
                } else {
                    Err(ParseUriError::InvalidSchemeCharacters)
                };
            }
        }
        Ok(None)
    }

    fn parse_authority(&mut self) -> Result<Option<Authority>, ParseUriError> {
        if self.peek_token(TokenKind::ForwardSlash) && self.peek_next_token(TokenKind::ForwardSlash)
        {
            self.tokens.pop_front();
            self.tokens.pop_front();

            let userinfo = self.parse_userinfo();
            let host = self.parse_host()?;
            let port = self.parse_port()?;

            Ok(Some(Authority {
                input: self.input.clone(),
                userinfo,
                host,
                port,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_userinfo(&mut self) -> Option<Span> {
        if let Some(Token {
            kind: TokenKind::Ident(span),
            ..
        }) = self.tokens.front()
        {
            if self.peek_next_token(TokenKind::Asterisk) {
                let span = span.to_owned();
                self.tokens.pop_front();
                self.tokens.pop_front();
                return Some(span);
            }
        }
        None
    }

    fn parse_host(&mut self) -> Result<Span, ParseUriError> {
        if let Some(Token {
            kind: TokenKind::Ident(span),
            ..
        }) = self.tokens.pop_front()
        {
            let start = span.start();
            let length = span.length();
            let host = &self.input[start..start + length];
            return if is_valid_host(&host) {
                Ok(span)
            } else {
                Err(ParseUriError::InvalidHostCharacters)
            };
        } else {
            Err(ParseUriError::MissingHost)
        }
    }

    fn parse_port(&mut self) -> Result<Option<Span>, ParseUriError> {
        if self.peek_token(TokenKind::Colon) {
            self.tokens.pop_front();
            if let Some(Token {
                kind: TokenKind::Ident(span),
                ..
            }) = self.tokens.pop_front()
            {
                // The default `len` that is added to the position is one, and if `length` is one, then the port is empty
                if span.length() == 1 {
                    return Ok(None);
                }
                return Ok(Some(span));
            }
        }
        Ok(None)
    }

    fn parse_path(&mut self) -> Span {
        let mut path = Span::new(0, 1);
        let mut first_forward_slash = false;
        while let Some(token) = self.tokens.front() {
            match token.kind() {
                TokenKind::ForwardSlash => {
                    if !first_forward_slash {
                        first_forward_slash = true;
                        path.set_start(token.span().start());
                    }
                    path.add_to_length(1);
                }
                TokenKind::Ident(part) => path.add_to_length(part.length()),
                _ => break,
            }
            self.tokens.pop_front();
        }
        path
    }

    fn parse_query(&mut self) -> Result<Option<Span>, ParseUriError> {
        if self.peek_token(TokenKind::QuestionMark) {
            let start = self
                .tokens
                .pop_front()
                .map_or(0, |token| token.span().start());
            let mut query = Span::new(start, 1);
            while let Some(token) = self.tokens.front() {
                if matches!(token.kind(), TokenKind::PoundSign) {
                    break;
                }
                match token.kind() {
                    TokenKind::Ident(part) => query.add_to_length(part.length()),
                    other => {
                        return Err(ParseUriError::UnexpectedToken(Unexpected::new(
                            other.to_owned(),
                            token.span().start(),
                        )))
                    }
                }
                self.tokens.pop_front();
            }
            Ok(Some(query))
        } else {
            Ok(None)
        }
    }

    fn parse_fragment(&mut self) -> Result<Option<Span>, ParseUriError> {
        if self.peek_token(TokenKind::PoundSign) {
            self.tokens.pop_front();
            match self.tokens.pop_front() {
                Some(Token {
                    kind: TokenKind::Ident(span),
                    ..
                }) => Ok(Some(span)),
                Some(Token { kind, span }) => Err(ParseUriError::UnexpectedToken(Unexpected::new(
                    kind,
                    span.start(),
                ))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn peek_token(&self, kind: TokenKind) -> bool {
        self.tokens
            .front()
            .map_or(false, |token| *token.kind() == kind)
    }

    fn peek_next_token(&self, kind: TokenKind) -> bool {
        self.tokens
            .get(1)
            .map_or(false, |token| *token.kind() == kind)
    }
}

fn is_valid_scheme(scheme: &str) -> bool {
    scheme
        .chars()
        .all(|ch| ch.is_ascii_alphabetic() || ch == '+' || ch == '-' || ch == '.')
}

fn is_valid_host(host: &str) -> bool {
    host.chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '.')
}

fn normalize_path(path: &mut String) {
    todo!()
}
