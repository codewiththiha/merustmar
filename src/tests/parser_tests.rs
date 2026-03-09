use crate::{ast::Statement, lexer::Lexer, parser::Parser};

#[test]
fn test_let_statements() {
    let input = "ထား x = 5။
ထား y = 10။
ထား foobar = 838383။
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    // ← NEW: Check for parser errors
    check_parser_errors(&parser);

    // Check program is not empty
    assert!(
        !program.statements.is_empty(),
        "parse_program() returned empty program"
    );

    // Check statement count
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    // Expected identifiers
    let tests = vec!["x", "y", "foobar"];

    for (i, expected_identifier) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        if !test_let_statement(stmt, expected_identifier) {
            return;
        }
    }
}

// ← NEW: Helper function to check parser errors
fn check_parser_errors(parser: &Parser) {
    let errors = parser.return_errors();
    if errors.is_empty() {
        return;
    }

    eprintln!("parser has {} errors", errors.len());
    for msg in errors {
        eprintln!("parser error: {}", msg);
    }
    panic!("parser has {} errors", errors.len());
}

/// Helper function to validate let statements
fn test_let_statement(stmt: &Statement, name: &str) -> bool {
    // Check TokenLiteral()
    if stmt.token_literal() != "ထား" {
        eprintln!(
            "stmt.token_literal() not 'ထား'. got={:?}",
            stmt.token_literal()
        );
        return false;
    }

    // Pattern match instead of type assertion
    match stmt {
        Statement::Let(let_stmt) => {
            // Check Name.Value
            if let_stmt.name.value != name {
                eprintln!(
                    "let_stmt.name.value not '{}'. got={}",
                    name, let_stmt.name.value
                );
                return false;
            }

            // Check Name.TokenLiteral()
            if let_stmt.name.token_literal() != name {
                eprintln!(
                    "let_stmt.name.token_literal() not '{}'. got={}",
                    name,
                    let_stmt.name.token_literal()
                );
                return false;
            }

            true
        }
        _ => {
            eprintln!("stmt not Statement::Let. got={:?}", stmt);
            false
        }
    }
}

// ← NEW: Test for invalid input (should produce errors)
#[test]
fn test_let_statements_invalid() {
    let input = "ထား x 5။
ထား = 10။
ထား 838383။
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let _program = parser.parse_program();

    // This SHOULD have errors
    let errors = parser.return_errors();
    assert!(
        !errors.is_empty(),
        "expected parser errors for invalid input, got none"
    );

    eprintln!("Expected errors ({} total):", errors.len());
    for msg in errors {
        eprintln!("  {}", msg);
    }
}
