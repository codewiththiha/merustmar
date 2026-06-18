# Contributing to Merustmar

Thanks for your interest in improving Merustmar! This document describes how to
set up a development environment and what conventions to follow.

## Development setup

Merustmar is a pure Rust project. You need a stable Rust toolchain (1.70+).

```bash
git clone https://github.com/codewiththiha/merustmar.git
cd merustmar
cargo build
cargo test
```

To build the WASM target (used by the in-browser playground):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Pre-commit checklist

Before pushing, please make sure all of the following pass:

```bash
cargo fmt --all -- --check        # formatting is clean
cargo clippy --all-targets -- -D warnings   # zero clippy warnings
cargo test                         # all tests pass
```

The CI workflow (`.github/workflows/ci.yml`) runs these same checks on every
push and pull request, so a green CI is the source of truth.

## Project layout

```
src/
├── ast.rs           # AST node definitions
├── builtins.rs      # Standard and terminal built-in functions
├── environment.rs   # Lexical environment (variable scoping)
├── evaluator.rs     # Tree-walking interpreter
├── lexer.rs         # Unicode-aware lexer
├── lib.rs           # Library entry (used by WASM build)
├── main.rs          # CLI entry point
├── object.rs        # Runtime object system
├── parser.rs        # Pratt parser
├── repl.rs          # Interactive REPL
├── runner.rs        # Script file runner
├── terminal.rs      # Low-level TUI wrappers (crossterm)
├── token.rs         # Token types and keyword mapping
└── tests/           # Inline test modules
```

## Adding a new built-in function

1. Implement the function in `src/builtins.rs` as `fn builtin_<name>(args: Vec<Object>) -> Object`.
2. Use `check_arg_count(expected, &args)` at the top for arity validation.
3. Register it inside `get_builtin` with the appropriate name (Myanmar or ASCII).
4. Document it in `README.md` under "Built-in Functions".
5. Add at least one test in `src/tests/evaluator_tests.rs`.

## Adding a new keyword or operator

1. Add the token type in `src/token.rs` (and the Myanmar keyword mapping if applicable).
2. Handle it in the lexer (`src/lexer.rs`).
3. Register prefix / infix parse functions in `src/parser.rs`.
4. Implement evaluation in `src/evaluator.rs`.
5. Add tests in `src/tests/`.

## Style conventions

- Run `cargo fmt` before every commit. Don't reformat code outside the lines you actually change.
- Avoid leaving commented-out dead code in the source — use git history instead.
- Keep tests focused: one concept per `#[test]` function where practical.
- When fixing a bug, add a regression test that fails before the fix and passes after.

## Reporting issues

Open a GitHub issue with:

- A minimal `.mrm` script that reproduces the problem.
- The exact `cargo run -- --run script.mrm` output.
- The Rust toolchain version (`rustc --version`).

## Pull requests

- Keep PRs focused — one feature or fix per PR.
- Reference any issue the PR closes ("Closes #12").
- Make sure CI is green before requesting review.
