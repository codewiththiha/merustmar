use crate::token::Token;

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

// Let Statement

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub value: Option<Expression>,
    pub name: Identifier,
}

impl LetStatement {
    pub fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

// Expression

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(i) => i.token_literal(),
        }
    }
}

// Statement Enum

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(ls) => ls.token_literal(),
        }
    }
}

// Program Main Struct of AST

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            return self.statements[0].token_literal();
        } else {
            ""
        }
    }
}
