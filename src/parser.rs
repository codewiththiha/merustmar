use std::collections::HashMap;

use crate::{
    ast::{
        ArrayLiteral, BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement,
        FunctionLiteral, HashLiteral, Identifier, IfExpression, IndexExpression, InfixExpression,
        IntegerLiteral, LetStatement, LoopExpression, MultiLetStatement, PrefixExpression, Program,
        ReassignStatement, ReturnStatement, Statement, StringLiteral,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

// Precedence constants
#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,      // ==
    LessGreater = 2, // > or <
    Sum = 3,         // +
    Product = 4,     // *
    Prefix = 5,      // -X or !X
    Call = 6,        // myFunction(X)
    Index = 7,
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
            TokenType::LBRACKET => Precedence::Index,
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

        // PrefixFns
        parser.register_prefix(TokenType::Ident, Parser::parse_identifier);
        parser.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::True, Parser::parse_boolean);
        parser.register_prefix(TokenType::False, Parser::parse_boolean);
        parser.register_prefix(TokenType::LParen, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::If, Parser::parse_if_expression);
        parser.register_prefix(TokenType::Function, Parser::parse_function_literal);
        parser.register_prefix(TokenType::String, Parser::parse_string_literal);
        parser.register_prefix(TokenType::LBRACKET, Parser::parse_array_literal);
        parser.register_prefix(TokenType::LBrace, Parser::parse_hash_literal);
        parser.register_prefix(TokenType::MyanmarInt, Parser::parse_n_times_loop);
        parser.register_prefix(TokenType::Loop, Parser::parse_while_or_inf_loop);

        // InfixFns
        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Eq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Gt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LParen, Parser::parse_call_expression);
        parser.register_infix(TokenType::LBRACKET, Parser::parse_index_expression);

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
            // If it's an Identifier AND the next token is '=', it is NOT a standard expression.
            TokenType::Ident if self.peek_token_is(TokenType::Assign) => {
                self.parse_ident_assign_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_n_times_loop(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let count = token.literal.parse::<i64>().ok()?;

        if !self.expect_peek(TokenType::TimesLoop) {
            return None;
        }
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::LoopExpression(LoopExpression {
            token,
            count: Some(count),
            condition: None,
            body,
        }))
    }

    pub fn parse_while_or_inf_loop(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let mut condition = None;

        if self.peek_token_is(TokenType::LBrace) {
            // Infinite loop syntax: ပတ် {}
            self.next_token();
        } else {
            // While loop syntax: ပတ် condition {}
            self.next_token();
            condition = self.parse_expression(Precedence::Lowest).map(Box::new);
            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }
        }

        let body = self.parse_block_statement();

        Some(Expression::LoopExpression(LoopExpression {
            token,
            count: None,
            condition,
            body,
        }))
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
        let mut left_exp = prefix_fn.unwrap()(self);

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            // notice in this code we used peek_token to get infix function
            let infix_fn = self
                .infix_parse_fns
                .get(&self.peek_token.token_type)
                .copied();
            if infix_fn.is_none() {
                return left_exp;
            }
            self.next_token();
            if let Some(le) = left_exp {
                left_exp = infix_fn.unwrap()(self, le);
            }
        }

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

    // Infix parser
    pub fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let left = Some(left).map(Box::new);
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).map(Box::new);

        Some(Expression::InfixExpression(InfixExpression {
            token,
            operator,
            left,
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

    // StringLiteral parser
    pub fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
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

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest);

        // fixed potential infinite loop
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(LetStatement { token, name, value }))
    }

    // ReturnStatement Parser
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(ReturnStatement {
            token,
            return_value,
        }));
    }

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let expresssion = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        expresssion
    }
    //// Get Complicated
    // pub fn parse_if_expression(&self) -> Option<Expression> {
    //     // capture "if" token
    //     let cur_token = self.cur_token;
    //     let altr = None;
    //     let cond_expr = ;
    //
    //     if !self.expect_peek(TokenType::LParen) {
    //         return None;
    //     }
    //
    //     self.next_token();
    //     if let Some(e) = self.parse_expression(Precedence::Lowest) {
    //         cond_expr = Box::new(Some(e));
    //     }
    //
    //     if !self.expect_peek(TokenType::RParen) {
    //         return None;
    //     }
    //
    //     let cons_expr = self.parse_block_statement();
    //
    //     if self.peek_token_is(TokenType::Else) {
    //         self.next_token();
    //
    //         if !self.expect_peek(TokenType::LBrace) {
    //             return None;
    //         }
    //
    //         altr = self.parse_block_statement();
    //     }
    //
    //     Some(Expression::IfExpression(IfExpression {
    //         token: cur_token,
    //         condition: cond_expr,
    //         consequence: cons_expr,
    //         alternative: altr,
    //     }))
    // }

    pub fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();

        // We use the `?` operator to safely extract the expression,
        // or return None early if it fails to parse.
        let condition = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }

            alternative = self.parse_block_statement();
        }

        Some(Expression::IfExpression(IfExpression {
            token,
            condition: condition.map(Box::new), // Box it here, not up top!
            consequence,
            alternative,
        }))
    }

    pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        // capture { lbrace
        let cur_token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();

        while !self.cur_token_is(TokenType::RBrace) && !self.peek_token_is(TokenType::Eof) {
            if let Some(s) = self.parse_statement() {
                statements.push(s);
            }
            self.next_token();
        }
        let pack_s = Some(statements);
        Some(BlockStatement {
            token: cur_token,
            statements: pack_s,
        })
    }

    pub fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    pub fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }
        // this skip the lparen and get grab on the first real param
        self.next_token();
        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    // ParseCallExpression
    pub fn parse_call_expression(
        &mut self,
        // in case you wonder, the function parameter get from the leftexp in infix_parse_fn
        // (leftexp become function arg for this case)
        function: Expression,
    ) -> Option<Expression> {
        Some(Expression::CallExpression(CallExpression {
            token: self.cur_token.clone(),
            function: Some(Box::new(function)),
            arguments: self.parse_expression_list(TokenType::RParen),
        }))
    }

    // function call args (ends with RParen) and array elements (ends with RBracket)
    pub fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut args = Vec::new();
        // for the function call without arguments like doSomething();
        if self.peek_token_is(end) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
            args.push(expr);
        }
        // for multiple args
        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                args.push(expr);
            }
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(args)
    }

    //Parse ArrayLiteral
    // Parse array literal: [1, 2, 3]
    pub fn parse_array_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let elements = self.parse_expression_list(TokenType::RBRACKET);
        Some(Expression::ArrayLiteral(ArrayLiteral { token, elements }))
    }

    // Parse index expression: myArray[0]
    // This is an infix parser — [ is treated as the "operator"
    // left operand = myArray, right operand = 0
    pub fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::RBRACKET) {
            return None;
        }

        Some(Expression::IndexExpression(IndexExpression {
            token,
            left: Some(Box::new(left)),
            index: index.map(Box::new),
        }))
    }

    /// Parse hash literal: {"key": value, "key2": value2}
    /// The `{` is already consumed as cur_token.
    pub fn parse_hash_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let mut pairs = Vec::new();

        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.push((key, value));

            // After each pair, expect either } or ,
            if !self.peek_token_is(TokenType::RBrace) && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peek(TokenType::RBrace) {
            return None;
        }

        Some(Expression::HashLiteral(HashLiteral { token, pairs }))
    }

    /// Parses `ident = expr` and then disambiguates:
    ///   - peek is `။` (Semicolon) or EOF  →  Reassignment
    ///   - peek is `,` or `လို့ထား` (LetSuffix)  →  Multi-let declaration
    pub fn parse_ident_assign_statement(&mut self) -> Option<Statement> {
        let start_token = self.cur_token.clone();

        // Capture the first identifier
        let first_name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        // Expect '='
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // Move past '=' and parse the expression
        self.next_token();
        let first_value = self.parse_expression(Precedence::Lowest)?;

        // Disambiguate based on what follows
        if self.peek_token_is(TokenType::Comma) || self.peek_token_is(TokenType::LetSuffix) {
            // ── It's a multi-let declaration ──
            let mut declarations = vec![(first_name, first_value)];

            // Collect additional comma-separated pairs
            while self.peek_token_is(TokenType::Comma) {
                self.next_token(); // consume ','
                if !self.expect_peek(TokenType::Ident) {
                    return None;
                }

                let name = Identifier {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                };

                if !self.expect_peek(TokenType::Assign) {
                    return None;
                }

                self.next_token(); // move past '='
                let value = self.parse_expression(Precedence::Lowest)?;
                declarations.push((name, value));
            }

            // Expect closing 'လို့ထား'
            if !self.expect_peek(TokenType::LetSuffix) {
                return None;
            }

            // Optional semicolon after လို့ထား
            if self.peek_token_is(TokenType::Semicolon) {
                self.next_token();
            }

            Some(Statement::MultiLet(MultiLetStatement {
                token: start_token,
                declarations,
            }))
        } else {
            // ── It's a reassignment: x = value။ ──
            if self.peek_token_is(TokenType::Semicolon) {
                self.next_token(); // consume ။
            }

            Some(Statement::Reassign(ReassignStatement {
                token: start_token,
                name: first_name,
                value: first_value,
            }))
        }
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

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::from_token_type(self.peek_token.token_type)
    }

    pub fn cur_precedence(&self) -> Precedence {
        Precedence::from_token_type(self.cur_token.token_type)
    }
}
