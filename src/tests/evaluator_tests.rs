use crate::{evaluator, lexer::Lexer, object::Object, parser::Parser};

// ============================================================================
// Helper: test_eval
// ============================================================================

fn test_eval(input: &str) -> Option<Object> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();

    let errors = parser.return_errors();
    if !errors.is_empty() {
        eprintln!("Parser errors: {:?}", errors);
        return None;
    }

    evaluator::eval_program(&program)
}

// ============================================================================
// Helper: test_integer_object
// ============================================================================

fn test_integer_object(obj: &Option<Object>, expected: i64) -> bool {
    match obj {
        Some(Object::Integer(value)) => {
            if *value != expected {
                eprintln!("object has wrong value. got={}, want={}", value, expected);
                return false;
            }
            true
        }
        Some(other) => {
            eprintln!("object is not Integer. got={:?}", other);
            false
        }
        None => {
            eprintln!("object is None");
            false
        }
    }
}

// ============================================================================
// Helper: test_boolean_object
// ============================================================================

fn test_boolean_object(obj: &Option<Object>, expected: bool) -> bool {
    match obj {
        Some(Object::Boolean(value)) => {
            if *value != expected {
                eprintln!("object has wrong value. got={}, want={}", value, expected);
                return false;
            }
            true
        }
        Some(other) => {
            eprintln!("object is not Boolean. got={:?}", other);
            false
        }
        None => {
            eprintln!("object is None");
            false
        }
    }
}

// ============================================================================
// Test: Integer Expression (EXTENDED with arithmetic)
// ============================================================================

#[test]
fn test_eval_integer_expression() {
    struct Test {
        input: &'static str,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "5",
            expected: 5,
        },
        Test {
            input: "10",
            expected: 10,
        },
        Test {
            input: "-5",
            expected: -5,
        },
        Test {
            input: "-10",
            expected: -10,
        },
        Test {
            input: "5 + 5 + 5 + 5 - 10",
            expected: 10,
        },
        Test {
            input: "2 * 2 * 2 * 2 * 2",
            expected: 32,
        },
        Test {
            input: "-50 + 100 + -50",
            expected: 0,
        },
        Test {
            input: "5 * 2 + 10",
            expected: 20,
        },
        Test {
            input: "5 + 2 * 10",
            expected: 25,
        },
        Test {
            input: "20 + 2 * -10",
            expected: 0,
        },
        Test {
            input: "50 / 2 * 2 + 10",
            expected: 60,
        },
        Test {
            input: "2 * (5 + 10)",
            expected: 30,
        },
        Test {
            input: "3 * 3 * 3 + 10",
            expected: 37,
        },
        Test {
            input: "3 * (3 * 3) + 10",
            expected: 37,
        },
        Test {
            input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
            expected: 50,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        if !test_integer_object(&evaluated, tt.expected) {
            panic!("test[{}] failed for input '{}'", i, tt.input);
        }
    }
}

// ============================================================================
// Test: Boolean Expression (EXTENDED with comparisons)
// ============================================================================

#[test]
fn test_eval_boolean_expression() {
    struct Test {
        input: &'static str,
        expected: bool,
    }

    let tests = vec![
        // Boolean literals
        Test {
            input: "မှန်",
            expected: true,
        },
        Test {
            input: "မှား",
            expected: false,
        },
        // Comparison operators
        Test {
            input: "1 < 2",
            expected: true,
        },
        Test {
            input: "1 > 2",
            expected: false,
        },
        Test {
            input: "1 < 1",
            expected: false,
        },
        Test {
            input: "1 > 1",
            expected: false,
        },
        Test {
            input: "1 == 1",
            expected: true,
        },
        Test {
            input: "1 != 1",
            expected: false,
        },
        Test {
            input: "1 == 2",
            expected: false,
        },
        Test {
            input: "1 != 2",
            expected: true,
        },
        // Boolean equality
        Test {
            input: "မှန် == မှန်",
            expected: true,
        },
        Test {
            input: "မှား == မှား",
            expected: true,
        },
        Test {
            input: "မှန် == မှား",
            expected: false,
        },
        Test {
            input: "မှန် != မှား",
            expected: true,
        },
        Test {
            input: "မှား != မှန်",
            expected: true,
        },
        // Comparison with boolean
        Test {
            input: "(1 < 2) == မှန်",
            expected: true,
        },
        Test {
            input: "(1 < 2) == မှား",
            expected: false,
        },
        Test {
            input: "(1 > 2) == မှန်",
            expected: false,
        },
        Test {
            input: "(1 > 2) == မှား",
            expected: true,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        if !test_boolean_object(&evaluated, tt.expected) {
            panic!("test[{}] failed for input '{}'", i, tt.input);
        }
    }
}

// ============================================================================
// Test: Bang Operator (!)
// ============================================================================

#[test]
fn test_bang_operator() {
    struct Test {
        input: &'static str,
        expected: bool,
    }

    let tests = vec![
        Test {
            input: "!မှန်",
            expected: false,
        },
        Test {
            input: "!မှား",
            expected: true,
        },
        Test {
            input: "!5",
            expected: false,
        },
        Test {
            input: "!!မှန်",
            expected: true,
        },
        Test {
            input: "!!မှား",
            expected: false,
        },
        Test {
            input: "!!5",
            expected: true,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        if !test_boolean_object(&evaluated, tt.expected) {
            panic!("test[{}] failed for input '{}'", i, tt.input);
        }
    }
}

// ============================================================================
// Test: Minus Operator (-)
// ============================================================================

#[test]
fn test_minus_operator() {
    struct Test {
        input: &'static str,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "-5",
            expected: -5,
        },
        Test {
            input: "-10",
            expected: -10,
        },
        Test {
            input: "--5",
            expected: 5,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        if !test_integer_object(&evaluated, tt.expected) {
            panic!("test[{}] failed for input '{}'", i, tt.input);
        }
    }
}
