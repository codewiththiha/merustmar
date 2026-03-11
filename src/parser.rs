use std::collections::HashMap;

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

// Precedence constants
pub enum Precedence {
    Lowest = 0,
    Equals = 1,      // ==
    LessGreater = 2, // > or <
    Sum = 3,         // +
    Product = 4,     // *
    Prefix = 5,      // -X or !X
    Call = 6,        // myFunction(X)
}

impl Precedence {
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::Lt => Precedence::LessGreater,
            TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

// New Encounter (we can define fn types too in rust)

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Option<Expression>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Option<Expression>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    pub cur_token: Token,
    pub peek_token: Token,
    errors: Vec<String>,
    // Pass 'a into the types here
    pub prefix_parse_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::new(TokenType::Illegial, "".to_string()),
            peek_token: Token::new(TokenType::Illegial, "".to_string()),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        parser.register_prefix(TokenType::Ident, Parser::parse_identifier);
        parser.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(ExpressionStatement {
            token,
            expression,
        }))
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // this is the part that filters out the correct function
        let prefix_fn = self
            .prefix_parse_fns
            .get(&self.cur_token.token_type)
            .copied();

        if prefix_fn.is_none() {
            self.no_prefix_parse_fn_error();
            return None;
        }
        // this is a bit confusing , cause we cast the parse_identifier into PrefixParseFn
        // and how it works is if the deifined fn's parameter and return type is matched it get
        // casted itself , at least that's what gemini said :))
        let left_exp = prefix_fn.unwrap()(self);
        left_exp
    }

    pub fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    // Prefix parser
    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self
            .parse_expression(Precedence::Prefix)
            .map(|x| Box::new(x));

        Some(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    // IntegerLiteral parser
    pub fn parse_integer_literal(&mut self) -> Option<Expression> {
        let value = self.cur_token.literal.parse::<i64>().ok();
        match value {
            Some(v) => Some(Expression::IntegerLiteral(IntegerLiteral {
                token: self.cur_token.clone(),
                value: v,
            })),
            None => {
                let msg = format!("could not parse {:?} as integer", self.cur_token.literal);
                self.errors.push(msg);
                None
            }
        }
    }

    // LetStatement parser
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        // don't get confused if expect_peek is correct the next_token get run
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while self.cur_token.token_type != TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Let(LetStatement {
            token,
            name,
            value: None,
        }))
    }

    // ReturnStatement Parser
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        self.next_token();

        while !self.cur_token_is(TokenType::Semicolon) || !self.cur_token_is(TokenType::Eof) {
            self.next_token();
        }

        return Some(Statement::Return(ReturnStatement {
            token,
            return_value: None,
        }));
    }

    // Helpers
    pub fn next_token(&mut self) -> () {
        // in this part don't get confused, the replace isn't actually replacing it's actually
        // acting like moving since when rust move it left the part that execute no var at all
        // might cause panic error so we have two option clone instead of move or this one since
        // this one is more effecient i used this , but doing self.peek_token.clon() is much
        // cleaner if you want other cleaner option!!
        self.cur_token = std::mem::replace(
            &mut self.peek_token,
            Token::new(TokenType::Illegial, "".to_string()),
        );
        self.peek_token = self.lexer.next_token();
    }

    // notice how in Vec we need to add & explictly
    pub fn return_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?} got {:?} instead.",
            t, self.cur_token.token_type
        );
        self.errors.push(msg);
    }

    fn register_prefix(&mut self, token_type: TokenType, fn_ptr: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(token_type, fn_ptr);
    }

    fn register_infix(&mut self, token_type: TokenType, fn_ptr: InfixParseFn<'a>) {
        self.infix_parse_fns.insert(token_type, fn_ptr);
    }

    pub fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    pub fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    // note expect_peek do increse the token
    pub fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
    pub fn no_prefix_parse_fn_error(&mut self) {
        let msg = format!(
            "no prefix parse function for {:?} found",
            self.cur_token.token_type
        );
        self.errors.push(msg);
    }
}
