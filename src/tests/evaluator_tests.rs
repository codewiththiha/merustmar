use crate::{environment::Environment, evaluator, lexer::Lexer, object::Object, parser::Parser};

// ============================================================================
// Helper: test_eval
// ============================================================================

pub fn test_eval(input: &str) -> Option<Object> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let env = Environment::new(); // returns Rc<RefCell<Environment>>
    let program = parser.parse_program();

    let errors = parser.return_errors();
    if !errors.is_empty() {
        eprintln!("Parser errors: {:?}", errors);
        return None;
    }

    evaluator::eval_program(&program, &env) // pass &Rc<...>
}

// ============================================================================
// Helper: test_integer_object
// ============================================================================

pub fn test_integer_object(obj: &Option<Object>, expected: i64) -> bool {
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

pub fn test_boolean_object(obj: &Option<Object>, expected: bool) -> bool {
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

// ============================================================================
// Test: If-Else Expressions
// ============================================================================

#[test]
fn test_return_statements() {
    struct Test {
        input: &'static str,
        expected: ExpectedValue,
    }

    enum ExpectedValue {
        Integer(i64),
        Boolean(bool),
    }

    let tests = vec![
        Test {
            input: "ဒါယူ 10။",
            expected: ExpectedValue::Integer(10),
        },
        Test {
            input: "ဒါယူ 5 + 5။",
            expected: ExpectedValue::Integer(10),
        },
        Test {
            input: "ဒါယူ 2 * 5။",
            expected: ExpectedValue::Integer(10),
        },
        Test {
            input: "ဒါယူ 10။\nဒါယူ 20။",
            expected: ExpectedValue::Integer(10),
        },
        Test {
            input: "ဒါယူ 2 > 1။",
            expected: ExpectedValue::Boolean(true),
        },
        // Nested if with return
        Test {
            input: "တကယ်လို့ (10 > 1) {
                တကယ်လို့ (10 > 1) {
                    ဒါယူ 10။
                }
                ဒါယူ 1။
            }",
            expected: ExpectedValue::Integer(10),
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        match &tt.expected {
            ExpectedValue::Integer(expected_val) => match evaluated {
                Some(Object::Integer(value)) => {
                    if value != *expected_val {
                        panic!(
                            "test[{}] - wrong value. got={}, want={}",
                            i, value, expected_val
                        );
                    }
                }
                other => {
                    panic!("test[{}] - expected Integer, got={:?}", i, other);
                }
            },
            ExpectedValue::Boolean(expected_val) => match evaluated {
                Some(Object::Boolean(value)) => {
                    if value != *expected_val {
                        panic!(
                            "test[{}] - wrong value. got={}, want={}",
                            i, value, expected_val
                        );
                    }
                }
                other => {
                    panic!("test[{}] - expected Boolean, got={:?}", i, other);
                }
            },
        }
    }
}

pub fn test_error_object(obj: &Option<Object>, expected_message: &str) -> bool {
    match obj {
        Some(Object::ErrorObj(message)) => {
            if message != expected_message {
                eprintln!(
                    "error message mismatch. got={}, want={}",
                    message, expected_message
                );
                return false;
            }
            true
        }
        Some(other) => {
            eprintln!("object is not ErrorObj. got={:?}", other);
            false
        }
        None => {
            eprintln!("object is None, expected ErrorObj");
            false
        }
    }
}

// ============================================================================
// Test: Error Handling
// ============================================================================

#[test]
fn test_error_handling() {
    struct Test {
        input: &'static str,
        expected_message: &'static str,
    }

    let tests = vec![
        Test {
            input: "5 + မှန်။",
            expected_message: "type mismatch: INTEGER + BOOLEAN",
        },
        Test {
            input: "5 + မှန်။ 5။",
            expected_message: "type mismatch: INTEGER + BOOLEAN",
        },
        Test {
            input: "-မှန်။",
            expected_message: "unknown operator: -BOOLEAN",
        },
        Test {
            input: "မှန် + မှား။",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN",
        },
        Test {
            input: "5။ မှန် + မှား။ 5။",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN",
        },
        Test {
            input: "တကယ်လို့ (10 > 1) { မှန် + မှား။ }",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN",
        },
        Test {
            input: "တကယ်လို့ (10 > 1) {
                တကယ်လို့ (10 > 1) {
                    ဒါယူ မှန် + မှား။
                }
                ဒါယူ 1။
            }",
            expected_message: "unknown operator: BOOLEAN + BOOLEAN",
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        if !test_error_object(&evaluated, tt.expected_message) {
            panic!("test[{}] failed for input '{}'", i, tt.input);
        }
    }
}

#[test]
fn test_function_object() {
    let input = "ဖန်ရှင်(x) { x + 2။ }။";
    let evaluated = test_eval(input);

    match evaluated {
        Some(Object::Function(func)) => {
            assert_eq!(
                func.parameters.len(),
                1,
                "function has wrong number of parameters. got={}",
                func.parameters.len()
            );

            assert_eq!(
                func.parameters[0].to_string(),
                "x",
                "parameter is not 'x'. got={}",
                func.parameters[0]
            );

            let expected_body = "(x + 2)";
            assert_eq!(
                func.body.to_string(),
                expected_body,
                "body is not '{}'. got={}",
                expected_body,
                func.body
            );
        }
        other => panic!("object is not Function. got={:?}", other),
    }
}

#[test]
fn test_function_application() {
    struct Test {
        input: &'static str,
        expected: i64,
    }

    let tests = vec![
        Test {
            input: "ထား identity = ဖန်ရှင်(x) { x။ }။ identity(5)။",
            expected: 5,
        },
        Test {
            input: "ထား identity = ဖန်ရှင်(x) { ဒါယူ x။ }။ identity(5)။",
            expected: 5,
        },
        Test {
            input: "ထား double = ဖန်ရှင်(x) { x * 2။ }။ double(5)။",
            expected: 10,
        },
        Test {
            input: "ထား add = ဖန်ရှင်(x, y) { x + y။ }။ add(5, 5)။",
            expected: 10,
        },
        Test {
            input: "ထား add = ဖန်ရှင်(x, y) { x + y။ }။ add(5 + 5, add(5, 5))။",
            expected: 20,
        },
        Test {
            input: "ဖန်ရှင်(x) { x။ }(5)",
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

// String Concatenation test

#[test]
fn test_string_concatenation() {
    let input = "\"Hello\" + \" \" + \"World\"";
    let evaluated = test_eval(input);

    match evaluated {
        Some(Object::String(value)) => {
            assert_eq!(value, "Hello World");
        }
        other => panic!("object is not String. got={:?}", other),
    }
}

// ============================================================================
// Test: Built-in Functions
// ============================================================================

#[test]
fn test_builtin_functions() {
    enum Expected {
        Int(i64),
        Err(String),
    }

    struct Test {
        input: &'static str,
        expected: Expected,
    }

    let tests = vec![
        Test {
            input: r#"len("")"#,
            expected: Expected::Int(0),
        },
        Test {
            input: r#"len("four")"#,
            expected: Expected::Int(4),
        },
        Test {
            input: r#"len("hello world")"#,
            expected: Expected::Int(11),
        },
        Test {
            input: r#"len(1)"#,
            expected: Expected::Err("argument to `len` not supported, got INTEGER".to_string()),
        },
        Test {
            input: r#"len("one", "two")"#,
            expected: Expected::Err("wrong number of arguments. got=2, want=1".to_string()),
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        match &tt.expected {
            Expected::Int(expected_val) => {
                if !test_integer_object(&evaluated, *expected_val) {
                    panic!("test[{}] failed for input '{}'", i, tt.input);
                }
            }
            Expected::Err(expected_msg) => {
                if !test_error_object(&evaluated, expected_msg) {
                    panic!("test[{}] failed for input '{}'", i, tt.input);
                }
            }
        }
    }
}

// ============================================================================
// Helper: test_null_object
// ============================================================================

pub fn test_null_object(obj: &Option<Object>) -> bool {
    match obj {
        Some(Object::Null) => true,
        Some(other) => {
            eprintln!("object is not Null. got={:?}", other);
            false
        }
        None => {
            eprintln!("object is None, expected Null");
            false
        }
    }
}

// ============================================================================
// Test: Array Literals
// ============================================================================

#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";
    let evaluated = test_eval(input);

    match evaluated {
        Some(Object::Array(elements)) => {
            assert_eq!(
                elements.len(),
                3,
                "array has wrong num of elements. got={}",
                elements.len()
            );
            assert!(test_integer_object(&Some(elements[0].clone()), 1));
            assert!(test_integer_object(&Some(elements[1].clone()), 4));
            assert!(test_integer_object(&Some(elements[2].clone()), 6));
        }
        other => panic!("object is not Array. got={:?}", other),
    }
}

// ============================================================================
// Test: Array Index Expressions
// ============================================================================

#[test]
fn test_array_index_expressions() {
    enum Expected {
        Int(i64),
        Null,
    }

    struct Test {
        input: &'static str,
        expected: Expected,
    }

    let tests = vec![
        Test {
            input: "[1, 2, 3][0]",
            expected: Expected::Int(1),
        },
        Test {
            input: "[1, 2, 3][1]",
            expected: Expected::Int(2),
        },
        Test {
            input: "[1, 2, 3][2]",
            expected: Expected::Int(3),
        },
        Test {
            input: "ထား i = 0။ [1][i]။",
            expected: Expected::Int(1),
        },
        Test {
            input: "[1, 2, 3][1 + 1]။",
            expected: Expected::Int(3),
        },
        Test {
            input: "ထား myArray = [1, 2, 3]။ myArray[2]။",
            expected: Expected::Int(3),
        },
        Test {
            input: "ထား myArray = [1, 2, 3]။ myArray[0] + myArray[1] + myArray[2]။",
            expected: Expected::Int(6),
        },
        Test {
            input: "ထား myArray = [1, 2, 3]။ ထား i = myArray[0]။ myArray[i]။",
            expected: Expected::Int(2),
        },
        Test {
            input: "[1, 2, 3][3]",
            expected: Expected::Null,
        },
        Test {
            input: "[1, 2, 3][-1]",
            expected: Expected::Null,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);

        match &tt.expected {
            Expected::Int(expected_val) => {
                if !test_integer_object(&evaluated, *expected_val) {
                    panic!("test[{}] failed for input '{}'", i, tt.input);
                }
            }
            Expected::Null => {
                if !test_null_object(&evaluated) {
                    panic!("test[{}] failed for input '{}'", i, tt.input);
                }
            }
        }
    }
}
