pub mod lexer;

use crate::{
    error::{Error, Unexpected},
    parsing::lexer::{
        token::{kind::TokenKind, span::Span, Token},
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
        let input = input.to_owned();
        let tokens = Lexer::new(&input).tokenize()?;
        Ok(Self {
            input,
            tokens: tokens.into(),
        })
    }

    pub fn parse(mut self) -> Result<Uri, Error> {
        let scheme = self.parse_scheme()?;
        let authority = self.parse_authority()?;
        let path = self.parse_path();
        let query = self.parse_query()?;
        let fragment = self.parse_fragment()?;

        if scheme.is_some() && authority.is_none() && path.length() != 0 {
            return Err(Error::SchemeWithoutAuthority);
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

    fn parse_scheme(&mut self) -> Result<Option<Span>, Error> {
        if let Some(Token {
            kind: TokenKind::Ident(span),
            ..
        }) = self.tokens.front()
        {
            if self.peek_next_token(TokenKind::Colon) {
                let span = span.to_owned();
                let start = span.start();
                let length = span.length();
                self.tokens.pop_front();
                self.tokens.pop_front();
                let scheme = &mut self.input[start..start + length];
                return if is_valid_scheme(scheme) {
                    Ok(Some(span))
                } else {
                    Err(Error::InvalidSchemeCharacters)
                };
            }
        }
        Ok(None)
    }

    fn parse_authority(&mut self) -> Result<Option<Authority>, Error> {
        if self.peek_token(TokenKind::ForwardSlash) && self.peek_next_token(TokenKind::ForwardSlash)
        {
            self.tokens.pop_front();
            self.tokens.pop_front();

            let userinfo = self.parse_userinfo();
            let host = self.parse_host()?;
            let port = self.parse_port()?;

            Ok(Some(Authority {
                input: self.input.to_owned(),
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

    fn parse_host(&mut self) -> Result<Span, Error> {
        if let Some(Token {
            kind: TokenKind::Ident(span),
            ..
        }) = self.tokens.pop_front()
        {
            let start = span.start();
            let length = span.length();
            let host = &mut self.input[start..start + length];
            if is_valid_host(host) {
                Ok(span)
            } else {
                Err(Error::InvalidHostCharacters)
            }
        } else {
            Err(Error::MissingHost)
        }
    }

    fn parse_port(&mut self) -> Result<Option<Span>, Error> {
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
        let mut path = Span::new(0, 0);
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

    fn parse_query(&mut self) -> Result<Option<Span>, Error> {
        if self.peek_token(TokenKind::QuestionMark) {
            self.tokens.pop_front();
            let start = self.tokens.front().map_or(0, |token| token.span().start());
            let mut query = Span::new(start, 0);
            while let Some(token) = self.tokens.front() {
                if matches!(token.kind(), TokenKind::PoundSign) {
                    break;
                }
                match token.kind() {
                    TokenKind::Ident(part) => query.add_to_length(part.length()),
                    other => {
                        return Err(Error::UnexpectedToken(Unexpected::new(
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

    fn parse_fragment(&mut self) -> Result<Option<Span>, Error> {
        if self.peek_token(TokenKind::PoundSign) {
            self.tokens.pop_front();
            match self.tokens.pop_front() {
                Some(Token {
                    kind: TokenKind::Ident(span),
                    ..
                }) => Ok(Some(span)),
                Some(Token { kind, span }) => {
                    Err(Error::UnexpectedToken(Unexpected::new(kind, span.start())))
                }
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

fn is_valid_scheme(scheme: &mut str) -> bool {
    scheme.make_ascii_lowercase();
    scheme
        .chars()
        .all(|ch| ch.is_ascii_alphabetic() || ch == '+' || ch == '-' || ch == '.')
}

fn is_valid_host(host: &mut str) -> bool {
    host.make_ascii_lowercase();
    host.chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '.')
}
