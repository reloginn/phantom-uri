use self::{kind::TokenKind, span::Span};

pub mod kind;
pub mod span;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }
    pub const fn span(&self) -> Span {
        self.span
    }
}
