pub mod ast;
pub mod builtins;
pub mod environment;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod terminal;
pub mod token;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_merustmar(input: &str) -> String {
    // Clear the output buffer from previous runs
    crate::builtins::OUTPUT_BUFFER.with(|b| b.borrow_mut().clear());

    let mut lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program();
    let env = environment::Environment::new();

    let errors = parser.return_errors();
    if !errors.is_empty() {
        return format!("Parser Errors:\n{}", errors.join("\n"));
    }

    // Evaluate
    evaluator::eval_program(&program, &env);

    // Fetch whatever was printed
    crate::builtins::OUTPUT_BUFFER.with(|b| b.borrow().clone())
}
