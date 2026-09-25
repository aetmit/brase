use std::println;

use common::io::file::{SourceManager};
use frontend::lex::{Lexer, TokenStream, token::TokenKind};

fn main() -> std::io::Result<()> {
    let sources = SourceManager;
    let file = sources.read("main.br")?;

    let mut lexer = Lexer::new(&file.source);
    loop {
        let token = lexer.next();
        println!("{:?}", token) ;
        if token.kind == TokenKind::Eof {
            break;
        }
    }

    Ok(())
}
