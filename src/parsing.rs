pub mod lexer;

use crate::{
    error::ParseUriError,
    parsing::lexer::token::{kind::TokenKind, Token},
    Authority, Uri,
};
use std::collections::VecDeque;

pub struct UriParser {
    tokens: VecDeque<Token>,
}

impl UriParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn parse(&mut self) -> Result<Uri, ParseUriError> {
        let scheme = self.parse_scheme()?;
        let authority = self.parse_authority()?;
        let path = self.parse_path();
        let query = self.parse_query()?;
        let fragment = self.parse_fragment()?;

        if scheme.is_some() && authority.is_none() {
            return Err(ParseUriError::SchemeWithoutAuthority);
        }

        Ok(Uri {
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }

    fn parse_scheme(&mut self) -> Result<Option<String>, ParseUriError> {
        if let Some(Token {
            kind: TokenKind::Ident(scheme),
            ..
        }) = self.tokens.front()
        {
            if self.peek_next_token(TokenKind::Colon) {
                let mut scheme = scheme.clone();
                scheme.make_ascii_lowercase();
                self.tokens.pop_front();
                self.tokens.pop_front();
                return if is_valid_scheme(&scheme) {
                    Ok(Some(scheme))
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
                userinfo,
                host,
                port,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_userinfo(&mut self) -> Option<String> {
        if let Some(Token {
            kind: TokenKind::Ident(maybe_userinfo),
            ..
        }) = self.tokens.front()
        {
            if self.peek_next_token(TokenKind::At) {
                let mut userinfo = maybe_userinfo.clone();
                userinfo.make_ascii_lowercase();
                self.tokens.pop_front();
                self.tokens.pop_front();
                return Some(userinfo);
            }
        }
        None
    }

    fn parse_host(&mut self) -> Result<String, ParseUriError> {
        if let Some(Token {
            kind: TokenKind::Ident(mut host),
            ..
        }) = self.tokens.pop_front()
        {
            host.make_ascii_lowercase();
            return if is_valid_host(&host) {
                Ok(host)
            } else {
                Err(ParseUriError::InvalidHostCharacters)
            };
        } else {
            Err(ParseUriError::MissingHost)
        }
    }

    fn parse_port(&mut self) -> Result<Option<u16>, ParseUriError> {
        if self.peek_token(TokenKind::Colon) {
            self.tokens.pop_front();
            if let Some(Token {
                kind: TokenKind::Ident(port_str),
                ..
            }) = self.tokens.pop_front()
            {
                if port_str.is_empty() {
                    return Ok(None);
                }
                return port_str
                    .parse::<u16>()
                    .map(Some)
                    .map_err(|_| ParseUriError::InvalidPort);
            }
        }
        Ok(None)
    }

    fn parse_path(&mut self) -> String {
        let mut path = String::new();
        while let Some(token) = self.tokens.front() {
            match token.kind() {
                TokenKind::ForwardSlash => path.push('/'),
                TokenKind::Ident(part) => path.push_str(part),
                _ => break,
            }
            self.tokens.pop_front();
        }
        path
    }

    fn parse_query(&mut self) -> Result<Option<String>, ParseUriError> {
        if self.peek_token(TokenKind::QuestionMark) {
            self.tokens.pop_front();
            let mut query = String::new();
            while let Some(token) = self.tokens.front_mut() {
                if matches!(token.kind(), TokenKind::PoundSign) {
                    break;
                }
                match token.kind() {
                    TokenKind::Ident(part) => query.push_str(part),
                    other => return Err(ParseUriError::UnexpectedToken(other.to_owned())),
                }
                self.tokens.pop_front();
            }
            Ok(Some(query))
        } else {
            Ok(None)
        }
    }

    fn parse_fragment(&mut self) -> Result<Option<String>, ParseUriError> {
        if self.peek_token(TokenKind::PoundSign) {
            self.tokens.pop_front();
            match self.tokens.pop_front() {
                Some(Token {
                    kind: TokenKind::Ident(fragment),
                    ..
                }) => Ok(Some(fragment)),
                Some(Token { kind, .. }) => Err(ParseUriError::UnexpectedToken(kind.to_owned())),
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
