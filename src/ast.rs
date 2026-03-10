use crate::token::Token;

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// Identifier
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

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// ReturnStatement

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl ReturnStatement {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&self.token_literal());
        out.push(' ');
        if let Some(ref value) = self.return_value {
            out.push_str(&value.to_string());
        }
        out.push('။');
        write!(f, "{}", out)
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

impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&self.token_literal());
        out.push(' ');
        out.push_str(&self.name.value);
        out.push_str(" = ");
        if let Some(ref value) = self.value {
            out.push_str(&value.to_string());
        }
        out.push('။');
        write!(f, "{}", out)
    }
}

// Expression

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(i) => i.token_literal(),
            Expression::IntegerLiteral(il) => il.token_literal(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::IntegerLiteral(il) => write!(f, "{}", il),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl ExpressionStatement {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(expr) = &self.expression {
            write!(f, "{}", expr)
        } else {
            Ok(())
        }
    }
}

// Statement Enum
#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(ls) => ls.token_literal(),
            Statement::Return(rs) => rs.token_literal(),
            Statement::Expression(es) => es.token_literal(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(ls) => write!(f, "{}", ls),
            Statement::Return(rs) => write!(f, "{}", rs),
            Statement::Expression(es) => write!(f, "{}", es),
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

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // way cleaner than using ref
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}
