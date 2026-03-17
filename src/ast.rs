use crate::token::Token;

// Expression(Root)

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
}

impl Expression {
    pub fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(i) => i.token_literal(),
            Expression::IntegerLiteral(il) => il.token_literal(),
            Expression::PrefixExpression(pe) => pe.token_literal(),
            Expression::InfixExpression(ie) => ie.token_literal(),
            Expression::Boolean(be) => be.token_literal(),
            Expression::IfExpression(ie) => ie.token_literal(),
            Expression::FunctionLiteral(fl) => fl.token_literal(),
            Expression::CallExpression(ce) => ce.token_literal(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::IntegerLiteral(il) => write!(f, "{}", il),
            Expression::PrefixExpression(pe) => write!(f, "{}", pe),
            Expression::InfixExpression(ie) => write!(f, "{}", ie),
            Expression::Boolean(be) => write!(f, "{}", be),
            Expression::IfExpression(ie) => write!(f, "{}", ie),
            Expression::FunctionLiteral(fl) => write!(f, "{}", fl),
            Expression::CallExpression(ce) => write!(f, "{}", ce),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(ls) => ls.token_literal(),
            Statement::Return(rs) => rs.token_literal(),
            Statement::Expression(es) => es.token_literal(),
            Statement::Block(bs) => bs.token_literal(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(ls) => write!(f, "{}", ls),
            Statement::Return(rs) => write!(f, "{}", rs),
            Statement::Expression(es) => write!(f, "{}", es),
            Statement::Block(bs) => write!(f, "{}", bs),
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
            // we'll see later
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

// Boolean Expression
#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Boolean {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

// InfixExpression

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub right: Option<Box<Expression>>,
    pub left: Option<Box<Expression>>,
    pub operator: String,
}

impl InfixExpression {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        if let Some(ref expr) = self.left {
            write!(f, "{}", expr)?;
        };
        write!(f, " {} ", self.operator)?;
        if let Some(ref expr) = self.right {
            write!(f, "{}", expr)?;
        }
        write!(f, ")")
    }
}

// PrefixExpression

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub right: Option<Box<Expression>>,
    pub operator: String,
}

impl PrefixExpression {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

/// This was only works if i didn't use Box
// impl std::fmt::Display for PrefixExpression {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "({}{})",
//             self.operator,
//             self.right.as_ref().map_or("", |r| r.token_literal())
//         )
//     }
// }

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(f, "{}", self.operator)?;

        if let Some(ref expr) = self.right {
            // Rust automatically "reaches into" the Box to find
            // the Display implementation for Expression
            write!(f, "{}", expr)?;
        }

        write!(f, ")")
    }
}

// IntegerLiteral
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone)]
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

// Block Statement
#[derive(PartialEq, Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Option<Vec<Statement>>,
}

impl BlockStatement {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        if let Some(statements) = &self.statements {
            for (_, s) in statements.iter().enumerate() {
                out.push_str(&s.to_string());
            }
        }

        write!(f, "{}", out)
    }
}

// If expression

#[derive(PartialEq, Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Option<Box<Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if")?;

        if let Some(ref condition) = self.condition {
            write!(f, "({})", condition)?; // condition is &Box<Expression>, auto-derefs
        }

        write!(f, " ")?;

        if let Some(ref consequence) = self.consequence {
            write!(f, "{}", consequence)?;
        }

        if self.alternative.is_some() {
            write!(f, "else")?;
            if let Some(ref alternative) = self.alternative {
                write!(f, "{}", alternative)?;
            }
        }

        Ok(())
    }
}

// Function Literal

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub body: Option<BlockStatement>,
    pub parameters: Option<Vec<Identifier>>,
}

impl FunctionLiteral {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //// This cost heap allocations multiple times
        // let mut statements = Vec::new();
        // for (_, s) in self.parameters.iter().enumerate() {
        //     if let Some(i) = s {
        //         statements.push(i.to_string());
        //     }
        // }

        write!(f, "{}", self.token_literal())?;
        write!(f, "(")?;
        // Just to print statements
        // This is the most optimized pattern in rust so take notes!!
        if let Some(params) = &self.parameters {
            let mut params_iter = params.iter();

            // take notes params_iter need mut cuz .next basically modifing (moving)
            if let Some(first) = params_iter.next() {
                write!(f, "{}", first.value)?;
                for param in params_iter {
                    write!(f, ", {}", param.value)?;
                }
            }
        }

        write!(f, ") ")?;
        if let Some(b) = &self.body {
            write!(f, "{}", b)?;
        }
        Ok(())
    }
}

// CallExpression

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub arguments: Option<Vec<Expression>>,
    pub function: Option<Box<Expression>>,
    pub token: Token,
}

impl CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(function) = &self.function {
            write!(f, "{}", function)?;
        }
        write! {f , "("}?;
        if let Some(args) = &self.arguments {
            let mut args_iterator = args.iter();
            if let Some(first) = args_iterator.next() {
                write!(f, "{}", first)?;
            }
            for (_, arg) in args_iterator.enumerate() {
                write!(f, ", {}", arg)?;
            }
        }
        write!(f, ")")
    }
}

// type FunctionLiteral
