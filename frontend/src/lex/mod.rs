
pub mod lexer;
pub mod token;

use token::Token;

pub trait TokenStream {
    fn next(&mut self) -> Token;
}

pub struct Lexer<'a> {
    source: &'a [u8],
    current: usize,
    start: usize,
    line: usize,
    column: usize,
    start_column: usize,
}
