use crate::lex::{Lexer, TokenStream};
use crate::lex::token::{Token, TokenKind};
use crate::span::Span;

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { 
            source,
            current: 0,
            start: 0,
            line: 0,
            column: 0,
            start_column: 0,
        } 
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<u8> {
        let c = self.peek();

        match c {
            Some(c) => {
                self.current += 1;

                if c == b'\n' {
                    self.column = 0;
                    self.line += 1;
                } else {
                    self.column += 1;
                }
            }
            None => {
                return None;
            }
        }

        c
    }

    fn create_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, Span::new(self.start, self.current - self.start), self.line, self.start_column)
    }
}

impl<'a> TokenStream for Lexer<'a> {
    fn next(&mut self) -> Token {
        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_whitespace() {
                    self.advance();
                    continue;
                }
            } else {
                return Token::new(TokenKind::Eof, Span::new(self.current, 0), self.line, self.column)
            }

            self.start = self.current;
            self.start_column = self.column;

            let c = match self.peek() {
                Some(x) => x,
                None => {
                    return self.create_token(TokenKind::Eof);
                }
            };

            match c {
                b'+' => {
                    self.advance();
                    return self.create_token(TokenKind::Plus);
                }
                b'-' =>  {
                    self.advance();
                    return self.create_token(TokenKind::Minus);
                }
                b'*' => {
                    self.advance();
                    return self.create_token(TokenKind::Star);
                }
                b'/' => {
                    self.advance();
                    return self.create_token(TokenKind::Slash);
                }
                b'(' => {
                    self.advance();
                    return self.create_token(TokenKind::OpenParen);
                }
                b')' => {
                    self.advance();
                    return self.create_token(TokenKind::CloseParen);
                }
                _ => {
                    if c.is_ascii_digit() {
                        while let Some(d) = self.peek() {
                            if d.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        return self.create_token(TokenKind::IntegerLiteral);
                    }

                }
            }
        }
    }
}