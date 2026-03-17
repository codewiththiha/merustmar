use std::io::{self, Write};

use crate::environment::Environment;
use crate::evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let env = Environment::new(); // ← outside the loop so state persists!

    loop {
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut lexer = Lexer::new(&input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();

        let errors = parser.return_errors();
        if !errors.is_empty() {
            print_parser_errors(&mut stdout, errors);
            continue;
        }

        let result = evaluator::eval_program(&program, &env);

        if let Some(obj) = result {
            println!("{}", obj.inspect());
        }
    }
}

fn print_parser_errors(out: &mut impl Write, errors: &[String]) {
    writeln!(out, "Woops! We ran into some parser errors:").unwrap();
    for msg in errors {
        writeln!(out, "\t{}", msg).unwrap();
    }
}

