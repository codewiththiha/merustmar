use crate::environment::Environment;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::{evaluator, terminal};
use std::fs;
use std::process;

pub fn run_file(path: &str) {
    if !path.ends_with(".mrm") {
        eprintln!("Error: File must have a .mrm extension. Got: {}", path);
        process::exit(1);
    }

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error: Could not read file '{}': {}", path, err);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&contents);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    let env = Environment::new();

    let errors = parser.return_errors();
    if !errors.is_empty() {
        print_parser_errors(errors);
        terminal::cleanup();
        return;
    }

    let result = evaluator::eval_program(&program, &env);
    terminal::cleanup();

    if let Some(obj) = result {
        // Skip printing `null` so scripts ending in a block don't dump a stray `null`.
        if !matches!(obj, crate::object::Object::Null) {
            println!("{}", obj.inspect());
        }
    }
}

fn print_parser_errors(errors: &[String]) {
    eprintln!("Oops! We ran into some syntax errors:");
    for msg in errors {
        // Each error message is already multi-line (header + source line + ^^^).
        // Separate consecutive errors with a blank line for readability.
        eprintln!("{}\n", msg);
    }
}
