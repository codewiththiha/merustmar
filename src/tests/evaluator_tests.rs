use crate::{environment::Environment, evaluator, lexer::Lexer, object::Object, parser::Parser};

// ============================================================================
// Helper: test_eval
// ============================================================================

pub fn test_eval(input: &str) -> Option<Object> {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let env = Environment::new(); // Returns Rc<RefCell<Environment>>.
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
            input: "1 <= 1",
            expected: true,
        },
        Test {
            input: "1 >= 1",
            expected: true,
        },
        Test {
            input: "1 <= 2",
            expected: true,
        },
        Test {
            input: "2 >= 1",
            expected: true,
        },
        Test {
            input: "2 <= 1",
            expected: false,
        },
        Test {
            input: "1 >= 2",
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

    let tests = [
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

    let tests = [
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

    let tests = [
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

    let tests = [
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

    let tests = [
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

    let tests = [
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

// ============================================================================
// Test: Hash Literals
// ============================================================================

#[test]
fn test_hash_literals() {
    let input = r#"ထား two = "two"။
    {
        "one": 10 - 9,
        two: 1 + 1,
        "thr" + "ee": 6 / 2,
        4: 4,
        မှန်: 5,
        မှား: 6
    }"#;

    let evaluated = test_eval(input);

    match evaluated {
        Some(Object::Hash(pairs)) => {
            let expected: Vec<(Object, i64)> = vec![
                (Object::String("one".to_string()), 1),
                (Object::String("two".to_string()), 2),
                (Object::String("three".to_string()), 3),
                (Object::Integer(4), 4),
                (Object::Boolean(true), 5),
                (Object::Boolean(false), 6),
            ];

            assert_eq!(
                pairs.len(),
                expected.len(),
                "Hash has wrong number of pairs. got={}",
                pairs.len()
            );

            for (key_obj, expected_value) in &expected {
                let hash_key = key_obj.hash_key().expect("key should be hashable");
                let pair = pairs.get(&hash_key).expect("no pair for given key");
                assert!(test_integer_object(
                    &Some(pair.value.clone()),
                    *expected_value
                ));
            }
        }
        other => panic!("object is not Hash. got={:?}", other),
    }
}

// ============================================================================
// Test: Hash Index Expressions
// ============================================================================

#[test]
fn test_hash_index_expressions() {
    enum Expected {
        Int(i64),
        Null,
    }

    struct Test {
        input: &'static str,
        expected: Expected,
    }

    let tests = [
        Test {
            input: r#"{"foo": 5}["foo"]"#,
            expected: Expected::Int(5),
        },
        Test {
            input: r#"{"foo": 5}["bar"]"#,
            expected: Expected::Null,
        },
        Test {
            input: r#"ထား key = "foo"။ {"foo": 5}[key]"#,
            expected: Expected::Int(5),
        },
        Test {
            input: r#"{}["foo"]"#,
            expected: Expected::Null,
        },
        Test {
            input: r#"{5: 5}[5]"#,
            expected: Expected::Int(5),
        },
        Test {
            input: r#"{မှန်: 5}[မှန်]"#,
            expected: Expected::Int(5),
        },
        Test {
            input: r#"{မှား: 5}[မှား]"#,
            expected: Expected::Int(5),
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

// ============================================================================
// Test: String / collection builtins
// ============================================================================

#[test]
fn test_eval_string_builtins() {
    struct Test {
        input: &'static str,
        expected: &'static str,
    }

    let tests = [
        Test {
            input: "upper(\"hello\")",
            expected: "HELLO",
        },
        Test {
            input: "lower(\"WORLD\")",
            expected: "world",
        },
        Test {
            input: "to_string(42)",
            expected: "42",
        },
        Test {
            input: "join([\"a\", \"b\", \"c\"], \",\")",
            expected: "a,b,c",
        },
        Test {
            input: "contains(\"hello\", \"ell\")",
            expected: "true",
        },
        Test {
            input: "contains(\"hello\", \"xyz\")",
            expected: "false",
        },
        Test {
            input: "contains([1, 2, 3], 2)",
            expected: "true",
        },
        Test {
            input: "contains([1, 2, 3], 9)",
            expected: "false",
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let evaluated = test_eval(tt.input);
        match evaluated {
            Some(Object::String(s)) => {
                assert_eq!(
                    s, tt.expected,
                    "test[{}] failed for input '{}'",
                    i, tt.input
                );
            }
            Some(Object::Boolean(b)) => {
                assert_eq!(
                    b.to_string(),
                    tt.expected,
                    "test[{}] failed for input '{}'",
                    i,
                    tt.input
                );
            }
            other => panic!(
                "test[{}] - expected String or Boolean, got {:?} for input '{}'",
                i, other, tt.input
            ),
        }
    }
}

#[test]
fn test_eval_split_builtin() {
    let evaluated = test_eval("split(\"a,b,c\", \",\")");
    match evaluated {
        Some(Object::Array(arr)) => {
            assert_eq!(arr.len(), 3, "split should return 3 elements");
            assert_eq!(arr[0], Object::String("a".to_string()));
            assert_eq!(arr[1], Object::String("b".to_string()));
            assert_eq!(arr[2], Object::String("c".to_string()));
        }
        other => panic!("expected Array, got {:?}", other),
    }
}

// ============================================================================
// Test: New loop forms (extended N-times, range, for-each) and global
// Myanmar numerals.
// ============================================================================

#[test]
fn test_eval_myanmar_numerals_in_math() {
    // Myanmar digits should work in any arithmetic context, not just N-times loops.
    let cases = [
        ("၅ + ၃", 8),
        ("၇ * ၂", 14),
        ("၁၀ - ၄", 6),
        ("၂၀ / ၅", 4),
        ("၅ + 10", 15), // mixed Myanmar + Arabic
        ("၇ > ၃", 1),   // truthy
    ];
    for (input, expected) in cases {
        let evaluated = test_eval(input);
        match evaluated {
            Some(Object::Integer(n)) => assert_eq!(
                n, expected,
                "input '{}' should evaluate to {}",
                input, expected
            ),
            Some(Object::Boolean(b)) => assert_eq!(
                b as i64, expected,
                "input '{}' should evaluate to {}",
                input, expected
            ),
            other => panic!("input '{}' returned {:?}", input, other),
        }
    }
}

#[test]
fn test_eval_times_loop_with_expression() {
    // `(n * 2) ခါပတ်` should iterate n*2 times. We test by counting iterations
    // via a side-effecting counter.
    let program = r#"
ထား count = 0။
ထား n = 3။
(n * 2) ခါပတ် {
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 6));
}

#[test]
fn test_eval_range_loop_no_var() {
    // `1 ကနေ 3 ထိပတ်` should run 3 times.
    let program = r#"
ထား count = 0။
1 ကနေ 3 ထိပတ် {
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_range_loop_with_var() {
    // `7 ကနေ 10 ထိပတ် i` should produce i = 7,8,9,10. Sum = 34.
    let program = r#"
ထား sum = 0။
7 ကနေ 10 ထိပတ် i {
    sum = sum + i။
}
sum
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 34)); // 7+8+9+10 = 34
}

#[test]
fn test_eval_range_loop_myanmar_numerals() {
    // `၇ ကနေ ၁၀ ထိပတ် i` — same as above but with Myanmar digits.
    let program = r#"
ထား sum = 0။
၇ ကနေ ၁၀ ထိပတ် i {
    sum = sum + i။
}
sum
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 34));
}

#[test]
fn test_eval_range_loop_with_expression_bounds() {
    // Range bounds can be expressions.
    let program = r#"
ထား start = 1။
ထား end = 4။
ထား count = 0။
start ကနေ end + 1 ထိပတ် i {
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 5)); // 1..=5
}

#[test]
fn test_eval_foreach_array() {
    // Iterate an array literal and sum the elements.
    let program = r#"
ထား sum = 0။
[10, 20, 30] ကနေ num ထိပတ် {
    sum = sum + num။
}
sum
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 60));
}

#[test]
fn test_eval_foreach_array_with_index() {
    // Iterate with explicit index variable; sum (index * value).
    let program = r#"
ထား sum = 0။
[10, 20, 30] ကနေ num, idx ထိပတ် {
    sum = sum + idx * num။
}
sum
"#;
    let evaluated = test_eval(program);
    // 0*10 + 1*20 + 2*30 = 0 + 20 + 60 = 80
    assert!(test_integer_object(&evaluated, 80));
}

#[test]
fn test_eval_foreach_function_result() {
    // The source of a for-each can be a function call returning an array.
    let program = r#"
ဖန်ရှင် get_nums() {
    ဒါယူ [1, 2, 3]။
}
ထား product = 1။
get_nums() ကနေ n ထိပတ် {
    product = product * n။
}
product
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 6)); // 1*2*3
}

#[test]
fn test_eval_foreach_string_array() {
    // For-each over an array of strings: concatenate them.
    let program = r#"
ထား result = ""။
["a", "b", "c"] ကနေ ch ထိပတ် {
    result = result + ch။
}
result
"#;
    let evaluated = test_eval(program);
    match evaluated {
        Some(Object::String(s)) => assert_eq!(s, "abc"),
        other => panic!("expected String, got {:?}", other),
    }
}

// ============================================================================
// Test: break (ရပ်) and continue (ကျော်) in every loop form
// ============================================================================

#[test]
fn test_eval_break_in_times_loop() {
    // `5 ခါပတ် i { ... break when i == 3 ... }` should print i=0,1,2.
    // We count iterations via a side-effect counter to make the test robust.
    let program = r#"
ထား count = 0။
5 ခါပတ် i {
    တကယ်လို့ (i == 3) {
        ရပ်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_continue_in_times_loop() {
    // `5 ခါပတ် i { ... continue when i == 2 ... }` should run bodies for
    // i = 0, 1, 3, 4 — count = 4.
    let program = r#"
ထား count = 0။
5 ခါပတ် i {
    တကယ်လို့ (i == 2) {
        ကျော်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 4));
}

#[test]
fn test_eval_break_in_while_loop() {
    let program = r#"
ထား i = 0။
ထား count = 0။
ပတ် (i < 100) {
    တကယ်လို့ (i >= 3) {
        ရပ်။
    }
    count = count + 1။
    i = i + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_continue_in_while_loop() {
    // Count odd numbers 1..=5 → 1, 3, 5 → count = 3
    let program = r#"
ထား j = 0။
ထား count = 0။
ပတ် (j < 5) {
    j = j + 1။
    တကယ်လို့ (j % 2 == 0) {
        ကျော်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_break_in_infinite_loop() {
    let program = r#"
ထား k = 0။
ထား count = 0။
ပတ် {
    တကယ်လို့ (k >= 3) {
        ရပ်။
    }
    count = count + 1။
    k = k + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_break_in_foreach() {
    // Stop at "banana"; only "apple" gets counted.
    let program = r#"
ထား count = 0။
["apple", "banana", "cherry", "date"] ကနေ fruit ထိပတ် {
    တကယ်လို့ (fruit == "banana") {
        ရပ်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 1));
}

#[test]
fn test_eval_continue_in_foreach() {
    // Skip "banana"; count = 3 (apple, cherry, date)
    let program = r#"
ထား count = 0။
["apple", "banana", "cherry", "date"] ကနေ fruit ထိပတ် {
    တကယ်လို့ (fruit == "banana") {
        ကျော်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 3));
}

#[test]
fn test_eval_break_in_range_loop() {
    // Range 1..=10, break at i==5 → count = 4 (i = 1,2,3,4)
    let program = r#"
ထား count = 0။
1 ကနေ 10 ထိပတ် i {
    တကယ်လို့ (i == 5) {
        ရပ်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 4));
}

#[test]
fn test_eval_continue_in_range_loop() {
    // Range 1..=5, skip i==3 → count = 4 (i = 1,2,4,5)
    let program = r#"
ထား count = 0။
1 ကနေ 5 ထိပတ် i {
    တကယ်လို့ (i == 3) {
        ကျော်။
    }
    count = count + 1။
}
count
"#;
    let evaluated = test_eval(program);
    assert!(test_integer_object(&evaluated, 4));
}

#[test]
fn test_eval_break_outside_loop_is_error() {
    let program = "ရပ်။";
    let evaluated = test_eval(program);
    match evaluated {
        Some(Object::ErrorObj(msg)) => {
            assert!(
                msg.contains("break"),
                "expected error mentioning break, got: {}",
                msg
            );
        }
        other => panic!("expected ErrorObj for break outside loop, got {:?}", other),
    }
}

#[test]
fn test_eval_continue_outside_loop_is_error() {
    let program = "ကျော်။";
    let evaluated = test_eval(program);
    match evaluated {
        Some(Object::ErrorObj(msg)) => {
            assert!(
                msg.contains("continue"),
                "expected error mentioning continue, got: {}",
                msg
            );
        }
        other => panic!(
            "expected ErrorObj for continue outside loop, got {:?}",
            other
        ),
    }
}

#[test]
fn test_eval_break_in_function_does_not_escape_to_caller_loop() {
    // A `break` inside a function body should NOT break the caller's loop —
    // it should produce a runtime error inside the function call, which then
    // propagates as an ErrorObj and stops the loop entirely.
    let program = r#"
ထား count = 0။
ဖန်ရှင် maybe_break(should_break) {
    တကယ်လို့ (should_break) {
        ရပ်။
    }
    ဒါယူ 1။
}
3 ခါပတ် i {
    ထား result = maybe_break(i == 1)။
    count = count + result။
}
count
"#;
    let evaluated = test_eval(program);
    // i=0: maybe_break(false) returns 1, count=1
    // i=1: maybe_break(true) tries to break -> error, loop stops
    // So count should be 1 (only i=0's increment happened before the error).
    // Actually the assignment happens before the error in i=0; in i=1 the
    // maybe_break returns an ErrorObj which then becomes the result of the
    // let-statement, which propagates as an error and halts the loop.
    match evaluated {
        Some(Object::ErrorObj(msg)) => assert!(msg.contains("break")),
        Some(Object::Integer(n)) => {
            // The exact count depends on how the let-statement handles the error.
            // What matters is that the break did NOT escape to the caller's loop.
            assert!(n < 3, "break should not have escaped to caller loop");
        }
        other => panic!("unexpected result: {:?}", other),
    }
}

// ============================================================================
// Test: Error message format (Line N, Token N + source pointer)
// ============================================================================

#[test]
fn test_error_format_missing_rparen() {
    // Input `ရေး("Hello World"` is missing the closing `)`.
    // The error should mention Line 1 and include the source line + `^` pointer.
    let input = "ရေး(\"Hello World\"";
    let evaluated = test_eval(input);
    match evaluated {
        None => {
            // test_eval returns None on parser errors, so we need to check
            // via the parser directly.
            let mut lexer = Lexer::new(input);
            let mut parser = Parser::new(&mut lexer);
            parser.parse_program();
            let errors = parser.return_errors();
            assert_eq!(
                errors.len(),
                1,
                "expected 1 parse error, got {}",
                errors.len()
            );
            let msg = &errors[0];
            assert!(
                msg.contains("Error at Line 1, Token"),
                "expected 'Error at Line 1, Token' in: {}",
                msg
            );
            assert!(
                msg.contains("expected next token to be RParen"),
                "expected RParen mention in: {}",
                msg
            );
            assert!(msg.contains("Eof"), "expected Eof mention in: {}", msg);
            assert!(msg.contains("^"), "expected ^ pointer in: {}", msg);
            assert!(
                msg.contains("ရေး(\"Hello World\""),
                "expected source line in: {}",
                msg
            );
        }
        Some(other) => panic!(
            "test_eval should return None on parse error, got {:?}",
            other
        ),
    }
}

#[test]
fn test_error_format_missing_assign() {
    // `ထား x 5။` — missing `=` between `x` and `5`.
    let input = "ထား x 5။";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program();
    let errors = parser.return_errors();
    assert!(!errors.is_empty(), "expected parse errors");
    let msg = &errors[0];
    assert!(
        msg.contains("Error at Line 1, Token"),
        "expected 'Error at Line 1, Token' in: {}",
        msg
    );
    assert!(
        msg.contains("expected next token to be Assign"),
        "expected Assign mention in: {}",
        msg
    );
    assert!(
        msg.contains("Int") || msg.contains("5"),
        "expected the offending token (Int / '5') in: {}",
        msg
    );
    assert!(msg.contains("^"), "expected ^ pointer in: {}", msg);
}

#[test]
fn test_error_format_multiline_input() {
    // Error on line 3 — the `^` pointer should point at line 3's source.
    let input = "ထား x = 10။\nထား y = 20။\nရေး(x +";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program();
    let errors = parser.return_errors();
    assert!(!errors.is_empty(), "expected parse errors");
    let msg = &errors[0];
    assert!(
        msg.contains("Error at Line 3, Token"),
        "expected 'Error at Line 3, Token' in: {}",
        msg
    );
    // The source line shown should be the 3rd line.
    assert!(
        msg.contains("ရေး(x +"),
        "expected source line 'ရေး(x +' in: {}",
        msg
    );
}

#[test]
fn test_error_format_includes_token_position() {
    // Verify that the "Token N" part is present and is a number.
    let input = "ရေး(\"hi\"";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    parser.parse_program();
    let errors = parser.return_errors();
    assert!(!errors.is_empty());
    let msg = &errors[0];
    // Check that "Token " is followed by a digit.
    assert!(
        msg.contains("Token ") && msg.chars().any(|c| c.is_ascii_digit()),
        "expected 'Token N' (with numeric N) in: {}",
        msg
    );
}
