# Merustmar

A Myanmar (Burmese) scripting language interpreter written in Rust.

**Status:** In development. Keywords and syntax may change without notice. Not yet stable for production use.

## Overview

Merustmar is a dynamic, interpreted language that utilizes Myanmar script keywords and punctuation. It is implemented as a tree-walking interpreter, featuring a Pratt parser and a Unicode-aware lexer. The language supports standard programming constructs—such as variables, functions, and complex data types—and includes a specialized set of built-in functions for terminal manipulation (TUI).

The project is a learning exercise inspired by *Writing an Interpreter in Go* (Thorsten Ball), adapted for the Rust ecosystem and localized for the Myanmar language.

## Features

### Core Language
- **Variable Bindings**: Single declarations (`ထား`) and multi-variable declarations (`လို့ထား`).
- **Data Types**: Integers, Floats, Booleans, Strings, Arrays, and Hashes (Key-Value pairs).
- **Arithmetic and Logic**: 
    - Operators: `+`, `-`, `*`, `/`, `%`
    - Comparisons: `==`, `!=`, `<`, `>`
    - Boolean Logic: `!`, `&&`, `||`
- **Control Flow**: 
    - Conditionals: `တကယ်လို့` (if) and `မဟုတ်ရင်` (else).
    - Functions: First-class functions defined with `ဖန်ရှင်` (fn).
    - Return values: `ဒါယူ` (return).
- **Looping Constructs**:
    - Infinite Loops: `ပတ် { ... }`
    - While-style Loops: `ပတ် condition { ... }`
    - Fixed-iteration Loops: `N ခါပတ် { ... }`
- **Collections**: Array indexing (`array[index]`) and Hash literals (`{"key": value}`).

### Terminal User Interface (TUI)
Merustmar integrates with `crossterm` to allow the creation of terminal-based applications.
- **Coordinate Printing**: Print text at specific X/Y coordinates.
- **Screen Management**: Initialize alternate screens, clear screens, and flush buffers.
- **UI Elements**: Built-in support for drawing centered borders and boxes.
- **Input Handling**: Blocking and non-blocking key polling.

## Keywords (Myanmar)

| Keyword | Meaning |
| :--- | :--- |
| `ထား` | `let` |
| `လို့ထား` | multi-let separator |
| `ဖန်ရှင်` | `fn` |
| `တကယ်လို့` | `if` |
| `မဟုတ်ရင်` | `else` |
| `ဒါယူ` | `return` |
| `မှန်` | `true` |
| `မှား` | `false` |
| `ပတ်` | `loop` |
| `ခါပတ်` | times-loop marker |
| `ရေး` | `print` (built-in) |

## Built-in Functions

### General Utilities
- `ရေး(val)`: Prints values to the console.
- `len(obj)`: Returns length of strings or arrays.
- `first(arr)`, `last(arr)`, `rest(arr)`: Array element access and slicing.
- `push(arr, val)`: Appends an element to an array.
- `sleep(ms)`: Pauses execution.
- `rand(min, max)`: Generates a random integer.

### Terminal Control
- `terminal_init()`: Initializes raw mode and the alternate screen.
- `terminal_end()`: Cleans up and restores the terminal.
- `clear()`: Clears the terminal screen.
- `terminal_size()`: Returns the current terminal width and height.
- `print_at(x, y, text)`: Prints text at a specific location.
- `print_at_center(x, y, cols, rows, text)`: Centers text within a specified box.
- `draw_border(cols, rows)`: Draws a centered border.
- `read_key()`: Blocks until a key is pressed.
- `poll_key(timeout)`: Checks for key input without blocking indefinitely.

## Building

Ensure you have a stable Rust toolchain (1.70+ recommended).

```bash
git clone https://github.com/codewiththiha/merustmar.git
cd merustmar
cargo build --release
```

## Usage

### REPL
Start the interactive Read-Eval-Print Loop:

```bash
cargo run -- --input
```

Example session:
```text
>> ထား x = 10။
>> ထား y = 20။
>> x + y
30
>> ရေး("မင်္ဂလာပါ")
မင်္ဂလာပါ
```

### Run a Script
Execute a source file with the `.mrm` extension:

```bash
cargo run -- --run examples/hello.mrm
```

## Project Structure

```
src/
├── ast.rs           # Abstract Syntax Tree nodes
├── builtins.rs      # Standard and Terminal built-in functions
├── environment.rs   # Variable environment and scoping
├── evaluator.rs     # Tree-walking evaluation logic
├── lexer.rs         # Unicode-aware lexer (supports Myanmar script)
├── main.rs          # CLI entry point
├── object.rs        # Object system and type definitions
├── parser.rs        # Pratt parser implementation
├── repl.rs          # Interactive shell logic
├── runner.rs        # Script file execution logic
├── token.rs         # Token definitions and keyword mapping
└── terminal.rs      # Low-level TUI wrappers (crossterm)
```

## License

This project is licensed under the **MIT License**.

## Acknowledgements

- Inspired by the Monkey programming language and the book *Writing an Interpreter in Go* by Thorsten Ball.
- Implemented using Rust's ownership model, pattern matching, and algebraic data types.
