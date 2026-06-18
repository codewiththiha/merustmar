use std::env;

mod ast;
mod builtins;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod runner;
mod terminal;
pub mod tests;
mod token;

const HELP_TEXT: &str = "\
Merustmar — a Myanmar (Burmese) scripting language interpreter.

USAGE:
    merustmar <COMMAND> [ARGS]

COMMANDS:
    --input                  Start the interactive REPL
    --run <file.mrm>         Run a Merustmar source file
    --help, -h               Show this help message
    --version, -V            Print version information

EXAMPLES:
    merustmar --input
    merustmar --run examples/hello.mrm

REPL:
    Type expressions and press Enter to evaluate.
    Press Ctrl+D (or Ctrl+C) to exit the REPL.
";

fn main() {
    let args: Vec<String> = env::args().collect();

    // With no arguments, print the help text and exit.
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "--input" => {
            println!("Hello! This is the Merustmar programming language!");
            println!("Feel free to type in commands (Ctrl+D to exit)");
            repl::start();
        }
        "--run" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a file path.");
                eprintln!("Example: merustmar --run script.mrm");
                std::process::exit(1);
            } else {
                runner::run_file(&args[2]);
            }
        }
        "--help" | "-h" => print_help(),
        "--version" | "-V" => {
            println!("merustmar {}", env!("CARGO_PKG_VERSION"));
        }
        unknown => {
            eprintln!("Error: Unknown command '{}'", unknown);
            eprintln!();
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    print!("{}", HELP_TEXT);
}
