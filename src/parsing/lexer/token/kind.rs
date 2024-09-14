use super::span::Span;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Colon,
    ForwardSlash,
    QuestionMark,
    PoundSign,
    Asterisk,
    Ident(Span),
    Other(char),
}
