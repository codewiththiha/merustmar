# Merustmar

A Myanmar (Burmese) scripting language interpreter written in Rust.

**Status:** In development. Keywords and syntax may change without notice. Not yet stable for production use.

## Overview

Merustmar is an implementation of a dynamic, interpreted language that uses Myanmar script keywords and punctuation. It supports variables, arithmetic, booleans, conditionals, functions, arrays, hashes, built-in functions, loops (infinite, while, fixed‑iteration), a REPL, and script file execution. The project is a learning exercise inspired by *Writing an Interpreter in Go* (Thorsten Ball) but adapted to Rust and Myanmar language syntax.

## Features

- **Variable bindings** (`ထား`, `လို့ထား` for multi‑let)
- **Integer, boolean, string, array, and hash data types**
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `==`, `!=`, `<`, `>`
- **Boolean operators**: `!` (not)
- **Conditionals**: `တကယ်လို့` (if), `မဟုတ်ရင်` (else)
- **Functions**: `ဖန်ရှင်` (fn)
- **Return**: `ဒါယူ` (return)
- **Built‑in functions**: `len`, `first`, `last`, `rest`, `push`, `ရေး` (print)
- **Loops**:
  - Infinite: `ပတ် { ... }`
  - While‑like: `ပတ် condition { ... }`
  - Fixed iteration: `N ခါပတ် { ... }`
- **Array indexing**: `array[index]`
- **Hash literals and indexing**: `{"key": value}`
- **REPL** for interactive experimentation
- **Run scripts** from `.mrm` files

## Keywords (Myanmar)

| Keyword      | Meaning      |
|--------------|--------------|
| `ထား`        | `let`        |
| `လို့ထား`    | multi‑let separator |
| `ဖန်ရှင်`     | `fn`         |
| `တကယ်လို့`   | `if`         |
| `မဟုတ်ရင်`   | `else`       |
| `ဒါယူ`       | `return`     |
| `မှန်`       | `true`       |
| `မှား`       | `false`      |
| `ပတ်`        | `loop`       |
| `ခါပတ်`      | times‑loop marker |
| `ရေး`        | `print` (built‑in) |

> **Note:** Keywords are subject to change as the language evolves.

## Building

Make sure you have a stable Rust toolchain (1.70+ recommended).

```bash
git clone https://github.com/codewiththiha/merustmar.git
cd merustmar
cargo build --release
```

## Usage

### REPL

```bash
cargo run -- --input
```

Example REPL session:

```text
>> ထား x = 10။
>> ထား y = 20။
>> x + y
30
>> ဖန်ရှင် (a, b) { a * b } (5, 6)
30
>> ရေး("မင်္ဂလာပါ")
မင်္ဂလာပါ
```

### Run a script

Scripts must have the `.mrm` extension.

```bash
cargo run -- --run examples/hello.mrm
```

Example `hello.mrm`:

```rust
ထား message = "Hello, Merustmar!"။
ရေး(message)။
```

## Testing

The project includes unit and integration tests for lexer, parser, and evaluator.

```bash
cargo test
```

## Project Structure

```
src/
├── ast.rs           # Abstract Syntax Tree nodes
├── builtins.rs      # Built‑in functions
├── environment.rs   # Variable environment (with nested scopes)
├── evaluator.rs     # Tree‑walking evaluator
├── lexer.rs         # Lexer (Unicode‑aware, supports Myanmar)
├── main.rs          # CLI entry point
├── object.rs        # Object system (Integer, Boolean, Array, Hash, etc.)
├── parser.rs        # Pratt parser
├── repl.rs          # Read‑Eval‑Print Loop
├── runner.rs        # File execution
├── token.rs         # Token types and keyword lookup
└── tests/           # Lexer, parser, evaluator tests
```

## License

This project is licensed under the **MIT License**. You are free to use, modify, and distribute it as long as the original copyright and permission notice are included.

## Acknowledgements

- Inspired by the [Monkey programming language](https://monkeylang.org/) and the book *Writing an Interpreter in Go* by Thorsten Ball.
- Built with Rust’s powerful pattern matching, algebraic data types, and ownership system.
