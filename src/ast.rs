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
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    HashLiteral(HashLiteral),
    LoopExpression(LoopExpression),
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
            Expression::StringLiteral(sl) => sl.token_literal(),
            Expression::ArrayLiteral(al) => al.token_literal(),
            Expression::IndexExpression(ie) => ie.token_literal(),
            Expression::HashLiteral(hl) => hl.token_literal(),
            Expression::LoopExpression(le) => le.token_literal(),
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
            Expression::StringLiteral(sl) => write!(f, "{}", sl),
            Expression::ArrayLiteral(al) => write!(f, "{}", al),
            Expression::IndexExpression(ie) => write!(f, "{}", ie),
            Expression::HashLiteral(hl) => write!(f, "{}", hl),
            Expression::LoopExpression(le) => write!(f, "{}", le),
        }
    }
}
// Loop Expression
#[derive(PartialEq, Debug, Clone)]
pub struct LoopExpression {
    pub token: Token, // Either MyanmarInt ('5') or Loop ('ပတ်')
    pub count: Option<i64>,
    pub condition: Option<Box<Expression>>,
    pub body: Option<BlockStatement>,
}

impl LoopExpression {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for LoopExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.count {
            write!(f, "{} ခါပတ် ", c)?;
        } else {
            write!(f, "ပတ် ")?;
            if let Some(ref cond) = self.condition {
                write!(f, "{} ", cond)?;
            }
        }
        if let Some(ref b) = self.body {
            write!(f, "{}", b)?;
        }
        Ok(())
    }
}

// Main Expression

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
    MultiLet(MultiLetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
    Reassign(ReassignStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        match self {
            Statement::Let(ls) => ls.token_literal(),
            Statement::Return(rs) => rs.token_literal(),
            Statement::Expression(es) => es.token_literal(),
            Statement::Block(bs) => bs.token_literal(),
            Statement::MultiLet(mls) => mls.token_literal(),
            Statement::Reassign(rs) => rs.token_literal(),
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
            Statement::MultiLet(mls) => write!(f, "{}", mls),
            Statement::Reassign(rs) => write!(f, "{}", rs),
        }
    }
}
// MultiLetStatement

#[derive(Debug, PartialEq, Clone)]
pub struct MultiLetStatement {
    pub token: Token, // The first identifier token
    pub declarations: Vec<(Identifier, Expression)>,
}

impl MultiLetStatement {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for MultiLetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut decl_strings = Vec::new();
        for (name, value) in &self.declarations {
            decl_strings.push(format!("{} = {}", name.value, value));
        }
        write!(f, "{} လို့ထား။", decl_strings.join(", "))
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

// StringLiteral

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl StringLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal)
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

// ArrayLiteral

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Option<Vec<Expression>>,
}

impl ArrayLiteral {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if let Some(elements) = &self.elements {
            let mut iter = elements.iter();
            // TODO note this down
            if let Some(first) = iter.next() {
                write!(f, "{}", first)?;
                for al in iter {
                    write!(f, ", {}", al)?;
                }
            }
        }
        write!(f, "]")
    }
}

// IndexExpression
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub index: Option<Box<Expression>>,
}

impl IndexExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // (something[]) we are creating this template
        write!(f, "(")?;
        if let Some(left) = &self.left {
            write!(f, "{}", left)?;
        }
        write!(f, "[")?;
        if let Some(index) = &self.index {
            write!(f, "{}", index)?;
        }
        write!(f, "])")
    }
}

// HashLiterals

#[derive(Debug, PartialEq, Clone)]
pub struct HashLiteral {
    pub token: Token, // the '{' token
    pub pairs: Vec<(Expression, Expression)>,
}

impl HashLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for HashLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.pairs.iter();
        if let Some((key, value)) = iter.next() {
            write!(f, "{}:{}", key, value)?;
            for (key, value) in iter {
                write!(f, ", {}:{}", key, value)?;
            }
        }
        write!(f, "}}")
    }
}

// ReassignStatement: x = newvalue။
#[derive(Debug, PartialEq, Clone)]
pub struct ReassignStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl ReassignStatement {
    pub fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl std::fmt::Display for ReassignStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name.value, self.value)
    }
}
