pub use crate::{
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
        // ← NEW: Function call precedence tests (from Go)
        PrecedenceTest {
            input: "a + add(b * c) + d",
            expected: "((a + add((b * c))) + d)",
        },
        PrecedenceTest {
            input: "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            expected: "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        },
        PrecedenceTest {
            input: "add(a + b + c * d / f + g)",
            expected: "add((((a + b) + ((c * d) / f)) + g))",
        },
        PrecedenceTest {
            input: "a * [1, 2, 3, 4][b * c] * d",
            expected: "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        },
        PrecedenceTest {
            input: "add(a * b[2], b[1], 2 * [1, 2][1])",
            expected: "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
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

// ============================================================================
// Function Literal Parsing Test (UPDATED for Option<Vec<Identifier>>)
// ============================================================================

#[test]
fn test_function_literal_parsing() {
    // Myanmar localized: ဖန်ရှင်(x, y) { x + y။ }
    let input = "ဖန်ရှင်(x, y) { x + y။ }";

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

    // Check it's an ExpressionStatement
    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    };

    // Check it's a FunctionLiteral
    let function = match &stmt.expression {
        Some(Expression::FunctionLiteral(func)) => func,
        _ => panic!(
            "stmt.Expression is not FunctionLiteral. got={:?}",
            stmt.expression
        ),
    };

    // Check parameters count
    assert!(
        function.parameters.is_some(),
        "function parameters should be Some"
    );

    let params = function.parameters.as_ref().unwrap();
    assert_eq!(
        params.len(),
        2,
        "function literal parameters wrong. want 2, got={}",
        params.len()
    );

    // Check first parameter is "x"
    assert_eq!(
        params[0].value, "x",
        "function parameter[0] is not 'x'. got={}",
        params[0].value
    );
    assert_eq!(
        params[0].token_literal(),
        "x",
        "function parameter[0].TokenLiteral is not 'x'. got={}",
        params[0].token_literal()
    );

    // Check second parameter is "y"
    assert_eq!(
        params[1].value, "y",
        "function parameter[1] is not 'y'. got={}",
        params[1].value
    );
    assert_eq!(
        params[1].token_literal(),
        "y",
        "function parameter[1].TokenLiteral is not 'y'. got={}",
        params[1].token_literal()
    );

    // Check body has 1 statement
    assert!(function.body.is_some(), "function body should be Some");

    let body = function.body.as_ref().unwrap();
    assert!(body.statements.is_some(), "body.statements should be Some");

    let statements = body.statements.as_ref().unwrap();
    assert_eq!(
        statements.len(),
        1,
        "function.Body.Statements has not 1 statements. got={}",
        statements.len()
    );

    // Check body statement is ExpressionStatement
    let body_stmt = match &statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "function body stmt is not ExpressionStatement. got={:?}",
            statements[0]
        ),
    };

    // Check body expression is infix (x + y)
    match &body_stmt.expression {
        Some(Expression::InfixExpression(infix)) => {
            // Check operator
            assert_eq!(
                infix.operator, "+",
                "body expression operator is not '+'. got={}",
                infix.operator
            );

            // Check left is identifier "x"
            match &infix.left {
                Some(left_expr) => match left_expr.as_ref() {
                    Expression::Identifier(ident) => {
                        assert_eq!(
                            ident.value, "x",
                            "left expression is not 'x'. got={}",
                            ident.value
                        );
                    }
                    _ => panic!("left expression is not Identifier"),
                },
                None => panic!("left expression is None"),
            }

            // Check right is identifier "y"
            match &infix.right {
                Some(right_expr) => match right_expr.as_ref() {
                    Expression::Identifier(ident) => {
                        assert_eq!(
                            ident.value, "y",
                            "right expression is not 'y'. got={}",
                            ident.value
                        );
                    }
                    _ => panic!("right expression is not Identifier"),
                },
                None => panic!("right expression is None"),
            }
        }
        _ => panic!("body expression is not InfixExpression"),
    }
}

// ============================================================================
// Additional Test: Function with No Parameters
// ============================================================================

#[test]
fn test_function_literal_no_parameters() {
    // Myanmar localized: ဖန်ရှင်() { 5။ }
    let input = "ဖန်ရှင်() { 5။ }";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("Not ExpressionStatement"),
    };

    let function = match &stmt.expression {
        Some(Expression::FunctionLiteral(func)) => func,
        _ => panic!("Not FunctionLiteral"),
    };

    // Depending on your parser implementation
    if let Some(params) = &function.parameters {
        assert_eq!(
            params.len(),
            0,
            "function with no parameters should have empty Vec"
        );
    }
    // Or if your parser returns None for no parameters:
    // assert!(function.parameters.is_none(), "parameters should be None");
}

// ============================================================================
// Function Parameter Parsing Test (NEW!)
// ============================================================================

#[test]
fn test_function_parameter_parsing() {
    // Table-driven test data (Myanmar localized)
    struct ParamTest {
        input: &'static str,
        expected_params: Vec<&'static str>,
    }

    let tests = vec![
        ParamTest {
            input: "ဖန်ရှင်() { }",
            expected_params: vec![],
        },
        ParamTest {
            input: "ဖန်ရှင်(x) { }",
            expected_params: vec!["x"],
        },
        ParamTest {
            input: "ဖန်ရှင်(x, y, z) { }",
            expected_params: vec!["x", "y", "z"],
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
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

        // Check it's a FunctionLiteral
        let function = match &stmt.expression {
            Some(Expression::FunctionLiteral(func)) => func,
            _ => panic!(
                "test[{}] - stmt.Expression is not FunctionLiteral. got={:?}",
                i, stmt.expression
            ),
        };

        // Check parameters count
        let actual_params = if let Some(ref params) = function.parameters {
            params.len()
        } else {
            0
        };

        assert_eq!(
            actual_params,
            tt.expected_params.len(),
            "test[{}] - length parameters wrong. want {}, got={}",
            i,
            tt.expected_params.len(),
            actual_params
        );

        // Check each parameter name
        if let Some(ref params) = function.parameters {
            for (j, expected_ident) in tt.expected_params.iter().enumerate() {
                if j < params.len() {
                    // Check parameter value
                    assert_eq!(
                        params[j].value, *expected_ident,
                        "test[{}] - parameter[{}] not '{}'. got='{}'",
                        i, j, expected_ident, params[j].value
                    );
                    // Check parameter token literal
                    assert_eq!(
                        params[j].token_literal(),
                        *expected_ident,
                        "test[{}] - parameter[{}].TokenLiteral not '{}'. got='{}'",
                        i,
                        j,
                        expected_ident,
                        params[j].token_literal()
                    );
                }
            }
        }
    }
}

// ============================================================================
// Call Expression Parsing Test (NEW!)
// ============================================================================

#[test]
fn test_call_expression_parsing() {
    // Myanmar localized: add(1, 2 * 3, 4 + 5)။
    let input = "add(1, 2 * 3, 4 + 5)။";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    // Check statement count
    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    // Check it's an ExpressionStatement
    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "stmt is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    };

    // Check it's a CallExpression
    let call_exp = match &stmt.expression {
        Some(Expression::CallExpression(call)) => call,
        _ => panic!(
            "stmt.Expression is not CallExpression. got={:?}",
            stmt.expression
        ),
    };

    // Check function is identifier "add"
    match &call_exp.function {
        Some(func_expr) => {
            if !test_identifier(func_expr.as_ref(), "add") {
                panic!("function is not identifier 'add'");
            }
        }
        None => panic!("function is None"),
    }

    // Check arguments count
    assert!(call_exp.arguments.is_some(), "arguments should be Some");

    let args = call_exp.arguments.as_ref().unwrap();
    assert_eq!(
        args.len(),
        3,
        "wrong length of arguments. got={}",
        args.len()
    );

    // Check first argument is integer literal 1
    assert!(
        test_literal_expression(&args[0], &LiteralExpected::Int(1)),
        "argument[0] is not integer literal 1"
    );

    // Check second argument is infix expression (2 * 3)
    assert!(
        test_infix_expression(
            &args[1],
            &LiteralExpected::Int(2),
            "*",
            &LiteralExpected::Int(3)
        ),
        "argument[1] is not infix expression 2 * 3"
    );

    // Check third argument is infix expression (4 + 5)
    assert!(
        test_infix_expression(
            &args[2],
            &LiteralExpected::Int(4),
            "+",
            &LiteralExpected::Int(5)
        ),
        "argument[2] is not infix expression 4 + 5"
    );
}

// ============================================================================
// Let Statements with Values Test (NEW!)
// ============================================================================

#[test]
fn test_let_statements_with_values() {
    // Table-driven test data (Myanmar localized)
    struct LetTest {
        input: &'static str,
        expected_identifier: &'static str,
        expected_value: LiteralExpected,
    }

    let tests = vec![
        LetTest {
            input: "ထား x = 5။",
            expected_identifier: "x",
            expected_value: LiteralExpected::Int(5),
        },
        LetTest {
            input: "ထား y = မှန်။",
            expected_identifier: "y",
            expected_value: LiteralExpected::Bool(true),
        },
        LetTest {
            input: "ထား foobar = y။",
            expected_identifier: "foobar",
            expected_value: LiteralExpected::String("y".to_string()),
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
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

        let stmt = &program.statements[0];

        // Check let statement identifier
        if !test_let_statement(stmt, tt.expected_identifier) {
            panic!("test[{}] - let statement test failed", i);
        }

        // Check the value expression
        if let Statement::Let(let_stmt) = stmt {
            if let Some(ref value) = let_stmt.value {
                if !test_literal_expression(value, &tt.expected_value) {
                    panic!("test[{}] - value expression test failed", i);
                }
            } else {
                panic!("test[{}] - let statement value is None", i);
            }
        }
    }
}

// ============================================================================
// Array Literal Parsing Test
// ============================================================================

#[test]
fn test_parsing_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "program.Statements[0] is not ExpressionStatement. got={:?}",
            program.statements[0]
        ),
    };

    let array = match &stmt.expression {
        Some(Expression::ArrayLiteral(al)) => al,
        _ => panic!("exp not ArrayLiteral. got={:?}", stmt.expression),
    };

    let elements = array.elements.as_ref().expect("elements should be Some");

    assert_eq!(
        elements.len(),
        3,
        "len(array.Elements) not 3. got={}",
        elements.len()
    );

    assert!(
        test_literal_expression(&elements[0], &LiteralExpected::Int(1)),
        "elements[0] failed"
    );
    assert!(
        test_infix_expression(
            &elements[1],
            &LiteralExpected::Int(2),
            "*",
            &LiteralExpected::Int(2)
        ),
        "elements[1] failed"
    );
    assert!(
        test_infix_expression(
            &elements[2],
            &LiteralExpected::Int(3),
            "+",
            &LiteralExpected::Int(3)
        ),
        "elements[2] failed"
    );
}

// ============================================================================
// Index Expression Parsing Test
// ============================================================================

#[test]
fn test_parsing_index_expressions() {
    let input = "myArray[1 + 1]";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    check_parser_errors(&parser);

    let stmt = match &program.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("Not ExpressionStatement"),
    };

    let index_exp = match &stmt.expression {
        Some(Expression::IndexExpression(ie)) => ie,
        _ => panic!("exp not IndexExpression. got={:?}", stmt.expression),
    };

    if let Some(ref left) = index_exp.left {
        assert!(
            test_identifier(left.as_ref(), "myArray"),
            "left is not identifier 'myArray'"
        );
    } else {
        panic!("left is None");
    }

    if let Some(ref index) = index_exp.index {
        assert!(
            test_infix_expression(
                index.as_ref(),
                &LiteralExpected::Int(1),
                "+",
                &LiteralExpected::Int(1),
            ),
            "index is not infix 1 + 1"
        );
    } else {
        panic!("index is None");
    }
}
