#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Colon,
    ForwardSlash,
    QuestionMark,
    PoundSign,
    At,
    Ident(String),
}
