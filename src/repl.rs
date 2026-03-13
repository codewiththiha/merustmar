use std::io::{self, Write};

use crate::evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // println!("Hello! This is the Merustmar programming language!");
    // println!("Feel free to type in commands (Ctrl+D to exit)");
    loop {
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break; // EOF (Ctrl+D)
        }

        // Parse the input
        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        // Check for parser errors
        let errors = parser.return_errors();
        if !errors.is_empty() {
            print_parser_errors(&mut stdout, errors);
            continue;
        }

        let result = evaluator::eval_program(&program);

        if let Some(obj) = result {
            println!("{}", obj.inspect());
        }
        // If None, don't print anything (statement had no value)
    }
}

fn print_parser_errors(out: &mut impl Write, errors: &[String]) {
    writeln!(out, "Woops! We ran into some parser errors:").unwrap();
    for msg in errors {
        writeln!(out, "\t{}", msg).unwrap();
    }
}
