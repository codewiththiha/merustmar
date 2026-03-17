use std::fs;
use std::io::{self, Write};

use crate::environment::Environment;
use crate::evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_file(path: &str) {
    if !path.ends_with(".mrm") {
        eprintln!("Error: File must have .mrm extension");
        return;
    }

    let contents = fs::read_to_string(path).expect("Could not read file");

    let mut lexer = Lexer::new(&contents);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let env = Environment::new();

    let errors = parser.return_errors();
    if !errors.is_empty() {
        let mut stdout = io::stdout();
        print_parser_errors(&mut stdout, errors);
        return;
    }

    let result = evaluator::eval_program(&program, &env);

    if let Some(obj) = result {
        println!("{}", obj.inspect());
    }
}

fn print_parser_errors(out: &mut impl Write, errors: &[String]) {
    eprintln!("Woops! We ran into some stupid errors:");
    for msg in errors {
        eprintln!("\t{}", msg);
    }
}

