pub use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::{Token, TokenType},
};

#[test]
fn test_ast_string() {
    // 1. Manually build the AST nodes (no lexer or parser involved).
    let program = Program {
        statements: vec![Statement::Let(LetStatement {
            token: Token::dummy(TokenType::Let, "ထား".to_string()),
            name: Identifier {
                token: Token::dummy(TokenType::Ident, "x".to_string()),
                value: "x".to_string(),
            },
            // Bind `x` to an identifier expression referencing `y`.
            value: Some(Expression::Identifier(Identifier {
                token: Token::dummy(TokenType::Ident, "y".to_string()),
                value: "y".to_string(),
            })),
        })],
    };

    // 2. Render the program to a string via the Display impl.
    let output = program.to_string();

    // 3. The Display impl should reproduce the original source form.
    assert_eq!(output, "ထား x = y။");
}
