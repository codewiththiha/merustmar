use crate::lexer::Lexer;
use crate::token::TokenType;
use std::fs;

pub fn run_file(path: &str) {
    if !path.ends_with(".mrm") {
        eprintln!("Error: File must have .mrm extension");
        return;
    }

    let contents = fs::read_to_string(path).expect("Could not read file");
    let mut l = Lexer::new(&contents);

    loop {
        let tok = l.next_token();
        if tok.token_type == TokenType::Eof {
            break;
        }
        println!("{:?}: {:?}", tok.token_type, tok.literal);
    }
}
