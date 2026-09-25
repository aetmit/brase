use crate::span::Span;

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