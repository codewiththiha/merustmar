use crate::{
    ast::{Expression, Statement},
    lexer::Lexer,
    parser::Parser,
};

// ============================================================================
// Helper Functions
// ============================================================================

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

fn test_integer_literal(right: &Option<Box<Expression>>, expected_value: i64) -> bool {
    let expr = match right {
        Some(boxed) => boxed.as_ref(),
        None => {
            eprintln!("il is None, expected IntegerLiteral");
            return false;
        }
    };

    let int_lit = match expr {
        Expression::IntegerLiteral(il) => il,
        _ => {
            eprintln!("il not IntegerLiteral. got={:?}", expr);
            return false;
        }
    };

    if int_lit.value != expected_value {
        eprintln!("integ.Value not {}. got={}", expected_value, int_lit.value);
        return false;
    }

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

fn test_identifier(exp: &Expression, value: &str) -> bool {
    match exp {
        Expression::Identifier(ident) => {
            if ident.value != value {
                eprintln!("ident.Value not {}. got={}", value, ident.value);
                return false;
            }
            if ident.token_literal() != value {
                eprintln!(
                    "ident.TokenLiteral not {}. got={}",
                    value,
                    ident.token_literal()
                );
                return false;
            }
            true
        }
        _ => {
            eprintln!("exp not Identifier. got={:?}", exp);
            false
        }
    }
}

fn test_boolean_literal(exp: &Expression, value: bool) -> bool {
    match exp {
        Expression::Boolean(bo) => {
            if bo.value != value {
                eprintln!("bo.Value not {}. got={}", value, bo.value);
                return false;
            }
            let expected_literal = if value {
                "မှန်"
            } else {
                "မှား"
            };
            if bo.token_literal() != expected_literal {
                eprintln!(
                    "bo.TokenLiteral not {}. got={}",
                    expected_literal,
                    bo.token_literal()
                );
                return false;
            }
            true
        }
        _ => {
            eprintln!("exp not Boolean. got={:?}", exp);
            false
        }
    }
}

fn test_literal_expression(exp: &Expression, expected: &LiteralExpected) -> bool {
    match expected {
        LiteralExpected::Int(v) => test_integer_literal(&Some(Box::new(exp.clone())), *v),
        LiteralExpected::String(s) => test_identifier(exp, s),
        LiteralExpected::Bool(b) => test_boolean_literal(exp, *b),
    }
}

enum LiteralExpected {
    Int(i64),
    String(String),
    Bool(bool),
}

fn test_infix_expression(
    exp: &Expression,
    left: &LiteralExpected,
    operator: &str,
    right: &LiteralExpected,
) -> bool {
    match exp {
        Expression::InfixExpression(op_exp) => {
            if let Some(ref left_expr) = op_exp.left {
                if !test_literal_expression(left_expr.as_ref(), left) {
                    return false;
                }
            } else {
                eprintln!("opExp.Left is None");
                return false;
            }
            if op_exp.operator != operator {
                eprintln!(
                    "exp.Operator is not '{}'. got={}",
                    operator, op_exp.operator
                );
                return false;
            }
            if let Some(ref right_expr) = op_exp.right {
                if !test_literal_expression(right_expr.as_ref(), right) {
                    return false;
                }
            } else {
                eprintln!("opExp.Right is None");
                return false;
            }
            true
        }
        _ => {
            eprintln!("exp is not InfixExpression. got={:?}", exp);
            false
        }
    }
}

// ============================================================================
// Let Statements Test
// ============================================================================

#[test]
fn test_let_statements() {
    let input = "ထား x = 5။
ထား y = 10။
ထား foobar = 838383။
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    assert!(
        !program.statements.is_empty(),
        "parse_program() returned empty program"
    );

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let tests = vec!["x", "y", "foobar"];

    for (i, expected_identifier) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        if !test_let_statement(stmt, expected_identifier) {
            return;
        }
    }
}

fn test_let_statement(stmt: &Statement, name: &str) -> bool {
    if stmt.token_literal() != "ထား" {
        eprintln!(
            "stmt.token_literal() not 'ထား'. got={:?}",
            stmt.token_literal()
        );
        return false;
    }

    match stmt {
        Statement::Let(let_stmt) => {
            if let_stmt.name.value != name {
                eprintln!(
                    "let_stmt.name.value not '{}'. got={}",
                    name, let_stmt.name.value
                );
                return false;
            }
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

// ============================================================================
// Return Statements Test
// ============================================================================

#[test]
fn test_return_statements() {
    let input = "ဒါယူ 5။
ဒါယူ 10။
ဒါယူ 993322။
";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for (i, stmt) in program.statements.iter().enumerate() {
        if !test_return_statement(stmt, i) {
            return;
        }
    }
}

fn test_return_statement(stmt: &Statement, index: usize) -> bool {
    if stmt.token_literal() != "ဒါယူ" {
        eprintln!(
            "stmt[{}].token_literal() not 'ဒါယူ'. got={:?}",
            index,
            stmt.token_literal()
        );
        return false;
    }

    match stmt {
        Statement::Return(return_stmt) => {
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

// ============================================================================
// Identifier Expression Test
// ============================================================================

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

// ============================================================================
// Integer Literal Expression Test
// ============================================================================

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
        Statement::Expression(expr_stmt) => match &expr_stmt.expression {
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
        },
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    }
}

// ============================================================================
// Prefix Expressions Test
// ============================================================================

#[test]
fn test_parsing_prefix_expressions() {
    struct PrefixTest {
        input: &'static str,
        operator: &'static str,
        value: LiteralExpected,
    }

    let prefix_tests = vec![
        PrefixTest {
            input: "!5။",
            operator: "!",
            value: LiteralExpected::Int(5),
        },
        PrefixTest {
            input: "-15။",
            operator: "-",
            value: LiteralExpected::Int(15),
        },
        PrefixTest {
            input: "!မှန်။",
            operator: "!",
            value: LiteralExpected::Bool(true),
        },
        PrefixTest {
            input: "!မှား။",
            operator: "!",
            value: LiteralExpected::Bool(false),
        },
    ];

    for (i, tt) in prefix_tests.iter().enumerate() {
        let mut lexer = Lexer::new(tt.input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "test[{}] - program.Statements does not contain 1 statements. got={}",
            i,
            program.statements.len()
        );

        let stmt = match &program.statements[0] {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!(
                "test[{}] - program.Statements[0] is not ExpressionStatement. got={:?}",
                i, program.statements[0]
            ),
        };

        let prefix_exp = match &stmt.expression {
            Some(Expression::PrefixExpression(prefix)) => prefix,
            _ => panic!(
                "test[{}] - stmt.Expression is not PrefixExpression. got={:?}",
                i, stmt.expression
            ),
        };

        assert_eq!(
            prefix_exp.operator, tt.operator,
            "test[{}] - exp.Operator is not '{}'. got={}",
            i, tt.operator, prefix_exp.operator
        );

        if let Some(ref right) = prefix_exp.right {
            if !test_literal_expression(right.as_ref(), &tt.value) {
                panic!("test[{}] - Right expression is not correct", i);
            }
        } else {
            panic!("test[{}] - Right expression is None", i);
        }
    }
}

// ============================================================================
// Infix Expressions Test
// ============================================================================

#[test]
fn test_parsing_infix_expressions() {
    struct InfixTest {
        input: &'static str,
        left: LiteralExpected,
        operator: &'static str,
        right: LiteralExpected,
    }

    let infix_tests = vec![
        InfixTest {
            input: "5 + 5။",
            left: LiteralExpected::Int(5),
            operator: "+",
            right: LiteralExpected::Int(5),
        },
        InfixTest {
            input: "မှန် == မှန်",
            left: LiteralExpected::Bool(true),
            operator: "==",
            right: LiteralExpected::Bool(true),
        },
        InfixTest {
            input: "မှန် != မှား",
            left: LiteralExpected::Bool(true),
            operator: "!=",
            right: LiteralExpected::Bool(false),
        },
        InfixTest {
            input: "မှား == မှား",
            left: LiteralExpected::Bool(false),
            operator: "==",
            right: LiteralExpected::Bool(false),
        },
    ];

    for (i, tt) in infix_tests.iter().enumerate() {
        let mut lexer = Lexer::new(tt.input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "test[{}] - program.Statements does not contain 1 statements. got={}",
            i,
            program.statements.len()
        );

        let stmt = match &program.statements[0] {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!(
                "test[{}] - program.Statements[0] is not ExpressionStatement. got={:?}",
                i, program.statements[0]
            ),
        };

        if let Some(ref exp) = stmt.expression {
            if !test_infix_expression(exp, &tt.left, tt.operator, &tt.right) {
                panic!("test[{}] - Infix expression test failed", i);
            }
        } else {
            panic!("test[{}] - Expression is None", i);
        }
    }
}

// ============================================================================
// Operator Precedence Test
// ============================================================================

#[test]
fn test_operator_precedence_parsing() {
    struct PrecedenceTest {
        input: &'static str,
        expected: &'static str,
    }

    let tests = vec![
        PrecedenceTest {
            input: "မှန်",
            expected: "မှန်",
        },
        PrecedenceTest {
            input: "မှား",
            expected: "မှား",
        },
        PrecedenceTest {
            input: "3 > 5 == မှား",
            expected: "((3 > 5) == မှား)",
        },
        PrecedenceTest {
            input: "3 < 5 == မှန်",
            expected: "((3 < 5) == မှန်)",
        },
        PrecedenceTest {
            input: "1 + (2 + 3) + 4",
            expected: "((1 + (2 + 3)) + 4)",
        },
        PrecedenceTest {
            input: "!(မှန် == မှန်)",
            expected: "(!(မှန် == မှန်))",
        },
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

// ============================================================================
// If Expression Test
// ============================================================================

#[test]
fn test_if_expression() {
    let input = "တကယ်လို့ (x < y) { x }";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    };

    let exp = match &stmt.expression {
        Some(Expression::IfExpression(if_exp)) => if_exp,
        _ => panic!(
            "stmt.Expression is not IfExpression. got={:?}",
            stmt.expression
        ),
    };

    // Check condition is infix expression (x < y)
    if let Some(ref condition) = exp.condition {
        if !test_infix_expression(
            condition,
            &LiteralExpected::String("x".to_string()),
            "<",
            &LiteralExpected::String("y".to_string()),
        ) {
            panic!("Condition is not correct infix expression");
        }
    } else {
        panic!("Condition is None");
    }

    // Check consequence has 1 statement
    if let Some(ref consequence) = exp.consequence {
        if let Some(ref statements) = consequence.statements {
            assert_eq!(
                statements.len(),
                1,
                "consequence is not 1 statements. got={}",
                statements.len()
            );

            if let Some(ref stmt) = statements.get(0) {
                match stmt {
                    Statement::Expression(expr_stmt) => {
                        if let Some(ref expr) = expr_stmt.expression {
                            if !test_identifier(expr, "x") {
                                panic!("Consequence expression is not identifier 'x'");
                            }
                        }
                    }
                    _ => panic!("Consequence statement is not ExpressionStatement"),
                }
            }
        }
    }

    // Check alternative is None
    assert!(
        exp.alternative.is_none(),
        "exp.Alternative was not nil. got={:?}",
        exp.alternative
    );
}
