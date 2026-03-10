use crate::{
    ast::{Expression, Statement},
    lexer::Lexer,
    parser::Parser,
    token::TokenType,
};

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

// ← NEW: Return statement test
#[test]
fn test_return_statements() {
    let input = "ဒါယူ 5။
ဒါယူ 10။
ဒါယူ 993322။
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    // Check for parser errors
    check_parser_errors(&parser);

    // Check statement count
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    // Check each statement is a Return statement
    for (i, stmt) in program.statements.iter().enumerate() {
        if !test_return_statement(stmt, i) {
            return;
        }
    }
}

// ← NEW: Helper to validate return statements
fn test_return_statement(stmt: &Statement, index: usize) -> bool {
    // Check TokenLiteral()
    if stmt.token_literal() != "ဒါယူ" {
        eprintln!(
            "stmt[{}].token_literal() not 'ဒါယူ'. got={:?}",
            index,
            stmt.token_literal()
        );
        return false;
    }

    // Pattern match instead of type assertion (Go: stmt.(*ast.ReturnStatement))
    match stmt {
        Statement::Return(return_stmt) => {
            // TokenLiteral check (redundant but matches Go test)
            if return_stmt.token_literal() != "ဒါယူ" {
                eprintln!(
                    "return_stmt[{}].token_literal() not 'ဒါယူ'. got={}",
                    index,
                    return_stmt.token_literal()
                );
                return false;
            }
            true
        }
        _ => {
            eprintln!("stmt[{}] not Statement::Return. got={:?}", index, stmt);
            false
        }
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar။";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got={}",
        program.statements.len()
    );

    match &program.statements[0] {
        Statement::Expression(expr_stmt) => match &expr_stmt.expression {
            Some(Expression::Identifier(ident)) => {
                assert_eq!(
                    ident.value, "foobar",
                    "ident.Value not foobar. got={}",
                    ident.value
                );
                assert_eq!(
                    ident.token_literal(),
                    "foobar",
                    "ident.TokenLiteral not foobar. got={}",
                    ident.token_literal()
                );
            }
            _ => panic!(
                "stmt.Expression is not Identifier. got={:?}",
                expr_stmt.expression
            ),
        },
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    }
}

#[test]
fn test_parser_initialization() {
    let input = "foobar။";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);

    // Debug: Check what tokens we have
    eprintln!(
        "cur_token: {:?} = {:?}",
        parser.cur_token.token_type, parser.cur_token.literal
    );
    eprintln!(
        "peek_token: {:?} = {:?}",
        parser.peek_token.token_type, parser.peek_token.literal
    );

    // Check if prefix_parse_fns has Ident registered
    eprintln!(
        "prefix_parse_fns has Ident: {}",
        parser.prefix_parse_fns.contains_key(&TokenType::Ident)
    );

    let program = parser.parse_program();

    eprintln!("Parsed {} statements", program.statements.len());

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_integer_literal_expression() {
    let input = "5။";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program has not enough statements. got={}",
        program.statements.len()
    );

    match &program.statements[0] {
        Statement::Expression(expr_stmt) => {
            // Check the expression is an IntegerLiteral
            match &expr_stmt.expression {
                Some(Expression::IntegerLiteral(int_lit)) => {
                    assert_eq!(
                        int_lit.value, 5,
                        "literal.Value not 5. got={}",
                        int_lit.value
                    );
                    assert_eq!(
                        int_lit.token_literal(),
                        "5",
                        "literal.TokenLiteral not 5. got={}",
                        int_lit.token_literal()
                    );
                }
                _ => panic!(
                    "stmt.Expression is not IntegerLiteral. got={:?}",
                    expr_stmt.expression
                ),
            }
        }
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    }
}
