pub use crate::{lexer::Lexer, token::TokenType};

#[test]
fn test_next_token_myanmar_full() {
    let input = "ထား five = 5။
ထား ten = 10။
ထား add = ဖန်ရှင်(x, y) {
x + y။
}။
ထား result = add(five, ten)။
!-/*5။
5 < 10 > 5။
တကယ်လို့ (5 < 10) {
ဒါယူ မှန်။
} မဟုတ်ရင် {
ဒါယူ မှား။
}
10 == 10။
10 != 9။
  \"foobar\"
\"Hello World\"
  [1, 2]။
  {\"foo\": \"bar\"}
";

    let tests = vec![
        (TokenType::Let, "ထား"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, "။"),
        (TokenType::Let, "ထား"),
        (TokenType::Ident, "ten"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, "။"),
        (TokenType::Let, "ထား"),
        (TokenType::Ident, "add"),
        (TokenType::Assign, "="),
        (TokenType::Function, "ဖန်ရှင်"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, "။"),
        (TokenType::RBrace, "}"),
        (TokenType::Semicolon, "။"),
        (TokenType::Let, "ထား"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::RParen, ")"),
        (TokenType::Semicolon, "။"),
        (TokenType::Bang, "!"),
        (TokenType::Minus, "-"),
        (TokenType::Slash, "/"),
        (TokenType::Asterisk, "*"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, "။"),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::Gt, ">"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, "။"),
        (TokenType::If, "တကယ်လို့"),
        (TokenType::LParen, "("),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "ဒါယူ"),
        (TokenType::True, "မှန်"),
        (TokenType::Semicolon, "။"),
        (TokenType::RBrace, "}"),
        (TokenType::Else, "မဟုတ်ရင်"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "ဒါယူ"),
        (TokenType::False, "မှား"),
        (TokenType::Semicolon, "။"),
        (TokenType::RBrace, "}"),
        (TokenType::Int, "10"),
        (TokenType::Eq, "=="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, "။"),
        (TokenType::Int, "10"),
        (TokenType::NotEq, "!="),
        (TokenType::Int, "9"),
        (TokenType::Semicolon, "။"),
        (TokenType::String, "foobar"),
        (TokenType::String, "Hello World"),
        (TokenType::LBRACKET, "["),
        (TokenType::Int, "1"),
        (TokenType::Comma, ","),
        (TokenType::Int, "2"),
        (TokenType::RBRACKET, "]"),
        (TokenType::Semicolon, "။"),
        (TokenType::LBrace, "{"),
        (TokenType::String, "foo"),
        (TokenType::Colon, ":"),
        (TokenType::String, "bar"),
        (TokenType::RBrace, "}"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input);

    for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
        let tok = lexer.next_token();

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
