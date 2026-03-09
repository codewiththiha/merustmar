use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!(">> ");
        stdout.flush().unwrap(); // Ensure ">> " shows up before we wait for input

        input.clear();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break; // EOF (Ctrl+D)
        }

        let mut l = Lexer::new(&input);

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            }
            println!("{:?}: {:?}", tok.token_type, tok.literal);
        }
    }
}
