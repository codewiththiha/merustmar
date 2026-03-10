pub use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::{Token, TokenType},
};

#[test]
fn test_ast_string() {
    // 1. Manually build the AST nodes (No Lexer, No Parser!)
    let program = Program {
        statements: vec![Statement::Let(LetStatement {
            token: Token::new(TokenType::Let, "ထား".to_string()),
            name: Identifier {
                token: Token::new(TokenType::Ident, "x".to_string()),
                value: "x".to_string(),
            },
            // We manually insert the 'y' expression here
            value: Some(Expression::Identifier(Identifier {
                token: Token::new(TokenType::Ident, "y".to_string()),
                value: "y".to_string(),
            })),
        })],
    };

    // 2. Test your perfect string() method
    let output = program.to_string();

    // 3. This will now pass!
    assert_eq!(output, "ထား x = y။");
}
