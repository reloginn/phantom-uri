pub mod token;

use std::fmt::Formatter;
use std::iter::Peekable;
use std::str::Chars;
use token::{kind::TokenKind, span::Span, Token};

#[derive(Debug)]
pub struct Unexpected(char, usize);

impl std::fmt::Display for Unexpected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected char `{}` at position {}", self.0, self.1)
    }
}

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
    pub fn next_token(&mut self) -> Result<Option<Token>, Unexpected> {
        let start = self.position();
        match self.next() {
            Some(ch) if ch.is_alphabetic() => {
                let mut ident = String::with_capacity(24);
                ident.push(ch);
                while let Some(next) = self.peek() {
                    if next.is_alphabetic() || next.is_alphanumeric() || next == '.' {
                        ident.push(self.next().unwrap_or_default());
                    } else {
                        break;
                    }
                }
                self.add_to_position(ident.len());
                Ok(Some(Token::new(
                    TokenKind::Ident(ident),
                    Span::new(start, self.position() - start),
                )))
            }
            Some(ch) => {
                self.add_to_position(1);
                let token = match ch {
                    ':' => Token::new(TokenKind::Colon, Span::new(start, 1)),
                    '/' => Token::new(TokenKind::ForwardSlash, Span::new(start, 1)),
                    '?' => Token::new(TokenKind::QuestionMark, Span::new(start, 1)),
                    '#' => Token::new(TokenKind::PoundSign, Span::new(start, 1)),
                    '@' => Token::new(TokenKind::At, Span::new(start, 1)),
                    other => return Err(Unexpected(other, start)),
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

#[cfg(test)]
mod tests {
    use super::{
        token::{kind::TokenKind, span::Span, Token},
        Lexer,
    };

    #[test]
    fn peek() {
        let mut lexer = Lexer::new("https://?#condition");
        let mut tokens = Vec::new();
        while let Ok(Some(token)) = lexer.next_token() {
            tokens.push(token)
        }
        assert_eq!(
            tokens,
            vec![
                Token::new(TokenKind::Ident("https".into()), Span::new(0, 5)),
                Token::new(TokenKind::Colon, Span::new(5, 1)),
                Token::new(TokenKind::ForwardSlash, Span::new(6, 1)),
                Token::new(TokenKind::ForwardSlash, Span::new(7, 1)),
                Token::new(TokenKind::QuestionMark, Span::new(8, 1)),
                Token::new(TokenKind::PoundSign, Span::new(9, 1)),
                Token::new(TokenKind::Ident("condition".into()), Span::new(10, 9))
            ]
        )
    }
}
