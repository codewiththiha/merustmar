use std::env;

mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod runner;
pub mod tests;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
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
                eprintln!("Error: Please provide a file path. Example: --run script.mrm");
            } else {
                runner::run_file(&args[2]);
            }
        }
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  --input          Start the interactive REPL");
    println!("  --run <file.mrm> Run a specific source file");
}
