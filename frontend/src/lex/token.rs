use crate::span::Span;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    IntegerLiteral,

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, line: usize, column: usize) -> Self {
        Self { kind, span, line, column }
    }
}