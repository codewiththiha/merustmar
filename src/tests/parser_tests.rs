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

#[test]
fn test_parsing_prefix_expressions() {
    // Table-driven test data (Go's slice of structs → Rust Vec of structs)
    struct PrefixTest {
        input: &'static str,
        operator: &'static str,
        integer_value: i64,
    }

    let prefix_tests = vec![
        PrefixTest {
            input: "!5။",
            operator: "!",
            integer_value: 5,
        },
        PrefixTest {
            input: "-15။",
            operator: "-",
            integer_value: 15,
        },
    ];

    for (i, tt) in prefix_tests.iter().enumerate() {
        let mut lexer = Lexer::new(tt.input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        // Check statement count
        assert_eq!(
            program.statements.len(),
            1,
            "test[{}] - program.Statements does not contain 1 statements. got={}",
            i,
            program.statements.len()
        );

        // Check it's an ExpressionStatement (Go: type assertion)
        let stmt = match &program.statements[0] {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!(
                "test[{}] - program.Statements[0] is not ExpressionStatement. got={:?}",
                i, program.statements[0]
            ),
        };

        // Check it's a PrefixExpression (Go: type assertion)
        let prefix_exp = match &stmt.expression {
            Some(Expression::PrefixExpression(prefix)) => prefix,
            _ => panic!(
                "test[{}] - stmt.Expression is not PrefixExpression. got={:?}",
                i, stmt.expression
            ),
        };

        // Check operator
        assert_eq!(
            prefix_exp.operator, tt.operator,
            "test[{}] - exp.Operator is not '{}'. got={}",
            i, tt.operator, prefix_exp.operator
        );

        // Check right expression is IntegerLiteral with correct value
        assert!(
            test_integer_literal(&prefix_exp.right, tt.integer_value),
            "test[{}] - Right expression is not correct IntegerLiteral",
            i
        );
    }
}

// fn test_integer_literal(right: &Option<Box<Expression>>, expected_value: i64) -> bool {
//     // Unbox the Expression
//     let expr = match right {
//         Some(boxed) => boxed.as_ref(),
//         None => {
//             eprintln!("il is None, expected IntegerLiteral");
//             return false;
//         }
//     };
//
//     // Check it's an IntegerLiteral
//     let int_lit = match expr {
//         Expression::IntegerLiteral(il) => il,
//         _ => {
//             eprintln!("il not IntegerLiteral. got={:?}", expr);
//             return false;
//         }
//     };
//
//     // Check value
//     if int_lit.value != expected_value {
//         eprintln!("integ.Value not {}. got={}", expected_value, int_lit.value);
//         return false;
//     }
//
//     // Check TokenLiteral
//     let expected_literal = expected_value.to_string();
//     if int_lit.token_literal() != expected_literal {
//         eprintln!(
//             "integ.TokenLiteral not {}. got={}",
//             expected_value,
//             int_lit.token_literal()
//         );
//         return false;
//     }
//
//     true
// }
//
#[test]
fn test_parsing_infix_expressions() {
    // Table-driven test data
    struct InfixTest {
        input: &'static str,
        left_value: i64,
        operator: &'static str,
        right_value: i64,
    }

    let infix_tests = vec![
        InfixTest {
            input: "5 + 5။",
            left_value: 5,
            operator: "+",
            right_value: 5,
        },
        InfixTest {
            input: "5 - 5။",
            left_value: 5,
            operator: "-",
            right_value: 5,
        },
        InfixTest {
            input: "5 * 5။",
            left_value: 5,
            operator: "*",
            right_value: 5,
        },
        InfixTest {
            input: "5 / 5။",
            left_value: 5,
            operator: "/",
            right_value: 5,
        },
        InfixTest {
            input: "5 > 5။",
            left_value: 5,
            operator: ">",
            right_value: 5,
        },
        InfixTest {
            input: "5 < 5။",
            left_value: 5,
            operator: "<",
            right_value: 5,
        },
        InfixTest {
            input: "5 == 5။",
            left_value: 5,
            operator: "==",
            right_value: 5,
        },
        InfixTest {
            input: "5 != 5။",
            left_value: 5,
            operator: "!=",
            right_value: 5,
        },
    ];

    for (i, tt) in infix_tests.iter().enumerate() {
        let mut lexer = Lexer::new(tt.input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        // Check statement count
        assert_eq!(
            program.statements.len(),
            1,
            "test[{}] - program.Statements does not contain 1 statements. got={}",
            i,
            program.statements.len()
        );

        // Check it's an ExpressionStatement
        let stmt = match &program.statements[0] {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!(
                "test[{}] - program.Statements[0] is not ExpressionStatement. got={:?}",
                i, program.statements[0]
            ),
        };

        // Check it's an InfixExpression
        let infix_exp = match &stmt.expression {
            Some(Expression::InfixExpression(infix)) => infix,
            _ => panic!(
                "test[{}] - stmt.Expression is not InfixExpression. got={:?}",
                i, stmt.expression
            ),
        };

        // Check left expression is IntegerLiteral with correct value
        assert!(
            test_integer_literal(&infix_exp.left, tt.left_value),
            "test[{}] - Left expression is not correct IntegerLiteral",
            i
        );

        // Check operator
        assert_eq!(
            infix_exp.operator, tt.operator,
            "test[{}] - exp.Operator is not '{}'. got={}",
            i, tt.operator, infix_exp.operator
        );

        // Check right expression is IntegerLiteral with correct value
        assert!(
            test_integer_literal(&infix_exp.right, tt.right_value),
            "test[{}] - Right expression is not correct IntegerLiteral",
            i
        );
    }
}

fn test_integer_literal(right: &Option<Box<Expression>>, expected_value: i64) -> bool {
    // Unbox the Expression
    let expr = match right {
        Some(boxed) => boxed.as_ref(),
        None => {
            eprintln!("il is None, expected IntegerLiteral");
            return false;
        }
    };

    // Check it's an IntegerLiteral
    let int_lit = match expr {
        Expression::IntegerLiteral(il) => il,
        _ => {
            eprintln!("il not IntegerLiteral. got={:?}", expr);
            return false;
        }
    };

    // Check value
    if int_lit.value != expected_value {
        eprintln!("integ.Value not {}. got={}", expected_value, int_lit.value);
        return false;
    }

    // Check TokenLiteral
    let expected_literal = expected_value.to_string();
    if int_lit.token_literal() != expected_literal {
        eprintln!(
            "integ.TokenLiteral not {}. got={}",
            expected_value,
            int_lit.token_literal()
        );
        return false;
    }

    true
}

#[test]
fn test_operator_precedence_parsing() {
    struct PrecedenceTest {
        input: &'static str,
        expected: &'static str,
    }

    let tests = vec![
        // Basic prefix expressions
        PrecedenceTest {
            input: "-a * b",
            expected: "((-a) * b)",
        },
        PrecedenceTest {
            input: "!-a",
            expected: "(!(-a))",
        },
        // Left associativity
        PrecedenceTest {
            input: "a + b + c",
            expected: "((a + b) + c)",
        },
        PrecedenceTest {
            input: "a + b - c",
            expected: "((a + b) - c)",
        },
        PrecedenceTest {
            input: "a * b * c",
            expected: "((a * b) * c)",
        },
        PrecedenceTest {
            input: "a * b / c",
            expected: "((a * b) / c)",
        },
        // Precedence (* and / before + and -)
        PrecedenceTest {
            input: "a + b / c",
            expected: "(a + (b / c))",
        },
        PrecedenceTest {
            input: "a + b * c + d / e - f",
            expected: "(((a + (b * c)) + (d / e)) - f)",
        },
        // Multiple statements (USE MYANMAR SEMICOLON `။` NOT `;`)
        PrecedenceTest {
            input: "3 + 4။\n-5 * 5",
            expected: "(3 + 4)((-5) * 5)",
        },
        // Comparison operators
        PrecedenceTest {
            input: "5 > 4 == 3 < 4",
            expected: "((5 > 4) == (3 < 4))",
        },
        PrecedenceTest {
            input: "5 < 4 != 3 > 4",
            expected: "((5 < 4) != (3 > 4))",
        },
        // Complex precedence
        PrecedenceTest {
            input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
            expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        },
        // ⚠️ COMMENT OUT THESE UNTIL YOU IMPLEMENT THEM:
        // Booleans (need parse_boolean)
        // PrecedenceTest { input: "true", expected: "true" },
        // PrecedenceTest { input: "false", expected: "false" },
        // PrecedenceTest { input: "3 > 5 == false", expected: "((3 > 5) == false)" },
        // PrecedenceTest { input: "3 < 5 == true", expected: "((3 < 5) == true)" },

        // Grouped expressions (need LParen prefix parser)
        // PrecedenceTest { input: "1 + (2 + 3) + 4", expected: "((1 + (2 + 3)) + 4)" },
        // PrecedenceTest { input: "(5 + 5) * 2", expected: "((5 + 5) * 2)" },
        // PrecedenceTest { input: "2 / (5 + 5)", expected: "(2 / (5 + 5))" },
        // PrecedenceTest { input: "-(5 + 5)", expected: "(-(5 + 5))" },
        // PrecedenceTest { input: "!(true == true)", expected: "(!(true == true))" },

        // Function calls (need CallExpression parser)
        // PrecedenceTest { input: "a + add(b * c) + d", expected: "((a + add((b * c))) + d)" },
        // PrecedenceTest { input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))" },
        // PrecedenceTest { input: "add(a + b + c * d / f + g)", expected: "add((((a + b) + ((c * d) / f)) + g))" },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let mut lexer = Lexer::new(tt.input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        let actual = program.to_string();

        assert_eq!(
            actual, tt.expected,
            "test[{}] - expected={:?}, got={:?}",
            i, tt.expected, actual
        );
    }
}
