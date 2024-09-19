pub mod token;

use crate::error::Unexpected;
use std::iter::Peekable;
use std::str::Chars;
use token::{kind::TokenKind, span::Span, Token};

pub struct Lexer<'lexer> {
    peekable: Peekable<Chars<'lexer>>,
    position: usize,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(input: &'lexer str) -> Self {
        Self {
            peekable: input.chars().peekable(),
            position: 0,
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Unexpected> {
        let mut tokens = Vec::with_capacity(24); // TODO: try to optimize
        while let Some(token) = self.next_token()? {
            tokens.push(token)
        }
        Ok(tokens)
    }
    pub fn next_token(&mut self) -> Result<Option<Token>, Unexpected> {
        let start = self.position();
        match self.next() {
            Some(ch) if ch.is_alphabetic() || ch.is_alphanumeric() => {
                let mut len = 1usize;
                while let Some(ch) = self.peek() {
                    if ch.is_alphabetic()
                        || ch.is_alphanumeric()
                        || ch == '.'
                        || ch == '-'
                        || ch == '='
                    {
                        self.next();
                        len += 1
                    } else {
                        break;
                    }
                }
                self.add_to_position(len);
                let span = Span::new(start, self.position() - start);
                Ok(Some(Token::new(TokenKind::Ident(span), span)))
            }
            Some(ch) => {
                self.add_to_position(1);
                let token = match ch {
                    ':' => Token::new(TokenKind::Colon, Span::new(start, 1)),
                    '/' => Token::new(TokenKind::ForwardSlash, Span::new(start, 1)),
                    '?' => Token::new(TokenKind::QuestionMark, Span::new(start, 1)),
                    '#' => Token::new(TokenKind::PoundSign, Span::new(start, 1)),
                    '@' => Token::new(TokenKind::Asterisk, Span::new(start, 1)),
                    other => return Err(Unexpected::new(TokenKind::Other(other), start)),
                };
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.peekable.peek().copied()
    }

    fn next(&mut self) -> Option<char> {
        self.peekable.next()
    }

    fn add_to_position(&mut self, num: usize) {
        self.position += num
    }

    const fn position(&self) -> usize {
        self.position
    }
}
