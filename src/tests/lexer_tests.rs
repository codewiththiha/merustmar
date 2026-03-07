pub use crate::lexer::Lexer;
pub use crate::token::TokenType;

#[test]
fn test_next_token() {
    let input = "=+(){},။";

    // Define expected tokens as a vector of tuples
    let tests = vec![
        (TokenType::Assign, "="),
        (TokenType::Plus, "+"),
        (TokenType::LParen, "("),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::RBrace, "}"),
        (TokenType::Comma, ","),
        (TokenType::Semicolon, "။"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input);

    for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
        let tok = lexer.next_token();

        // Rust's assert_eq! is cleaner than t.Fatalf
        // It automatically prints both values on failure
        assert_eq!(
            tok.token_type, *expected_type,
            "tests[{}] - token_type wrong. expected={:?}, got={:?}",
            i, expected_type, tok.token_type
        );

        assert_eq!(
            tok.literal, *expected_literal,
            "tests[{}] - literal wrong. expected={:?}, got={:?}",
            i, expected_literal, tok.literal
        );
    }
}
