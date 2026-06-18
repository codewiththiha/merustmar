use std::collections::HashMap;

use crate::{
    ast::{
        ArrayLiteral, BlockStatement, Boolean, BreakStatement, CallExpression, ContinueStatement,
        Expression, ExpressionStatement, FloatLiteral, FunctionLiteral, HashLiteral, Identifier,
        IfExpression, IndexExpression, InfixExpression, IntegerLiteral, LetStatement,
        LoopExpression, LoopKind, MultiLetStatement, PrefixExpression, Program, ReassignStatement,
        ReturnStatement, Statement, StringLiteral,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

// Precedence constants
#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    Lowest = 0,
    Or = 1,
    And = 2,
    Equals = 3,      // ==
    LessGreater = 4, // > or <
    Sum = 5,         // +
    Product = 6,     // *
    Prefix = 7,      // -X or !X
    Call = 8,        // myFunction(X)
    Index = 9,
}

impl Precedence {
    pub fn from_token_type(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Or => Precedence::Or,
            TokenType::And => Precedence::And,
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::Lt => Precedence::LessGreater,
            TokenType::Gt => Precedence::LessGreater,
            TokenType::LtEq => Precedence::LessGreater,
            TokenType::GtEq => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            TokenType::Percent => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBRACKET => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}

// Rust lets us define type aliases for function signatures.
type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Option<Expression>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Option<Expression>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    pub cur_token: Token,
    pub peek_token: Token,
    errors: Vec<String>,
    // 1-based position of `cur_token` in the token stream. Used for error
    // messages of the form `Error at Line L, Token N: ...`.
    token_position: usize,
    // Pass 'a into the types here
    pub prefix_parse_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::dummy(TokenType::Illegal, String::new()),
            peek_token: Token::dummy(TokenType::Illegal, String::new()),
            errors: Vec::new(),
            token_position: 0,
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
        parser.register_prefix(TokenType::Loop, Parser::parse_while_or_inf_loop);
        parser.register_prefix(TokenType::Float, Parser::parse_float_literal);

        // InfixFns
        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Eq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Gt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LtEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GtEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LParen, Parser::parse_call_expression);
        parser.register_infix(TokenType::LBRACKET, Parser::parse_index_expression);
        parser.register_infix(TokenType::And, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Or, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Percent, Parser::parse_infix_expression);

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
            TokenType::Function if self.peek_token_is(TokenType::Ident) => {
                self.parse_function_declaration()
            }
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            // If it's an Identifier AND the next token is '=', it is NOT a standard expression.
            TokenType::Ident if self.peek_token_is(TokenType::Assign) => {
                self.parse_ident_assign_statement()
            }
            _ => self.parse_expression_statement(),
        }
    }

    // `ရပ်။` — break out of the enclosing loop.
    pub fn parse_break_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        // Optional trailing `။` (semicolon's replacement :D ).
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::Break(BreakStatement { token }))
    }

    // `ကျော်။` — skip to the next iteration of the enclosing loop.
    pub fn parse_continue_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Statement::Continue(ContinueStatement { token }))
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        // After parsing the leading expression, we may be looking at one of the
        // expression-led loop forms:
        //   <expr> ခါပတ် { }                 — N-times loop
        //   <arr_expr> ကနေ <var> ထိပတ် { }  — array for-each
        //   <arr_expr> ကနေ <var>, <idx> ထိပတ် { }  — array for-each with index
        //   <start> ကနေ <end> ထိပတ် { }    — range loop (no var)
        //   <start> ကနေ <end> ထိပတ် <var> { } — range loop with var
        if self.peek_token_is(TokenType::TimesLoop) {
            return self.parse_times_loop_from_expr(token, expression);
        }
        if self.peek_token_is(TokenType::FromMarker) {
            return self.parse_from_loop_from_expr(token, expression);
        }

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(ExpressionStatement {
            token,
            expression: Some(expression),
        }))
    }

    // Builds a `Times(count_expr)` loop once `<count_expr> ခါပတ်` has been matched.
    // Supports an optional loop variable:
    //   <expr> ခါပတ် { }       — no var (just iterate N times)
    //   <expr> ခါပတ် i { }     — bind i to the 0-based iteration index
    fn parse_times_loop_from_expr(
        &mut self,
        token: Token,
        count_expr: Expression,
    ) -> Option<Statement> {
        // Consume `ခါပတ်`. After this, cur is `ခါပတ်` and peek is whatever
        // follows (either an identifier for the loop var, or `{`).
        self.next_token();

        // Optional loop variable: `ခါပတ် i { }` vs `ခါပတ် { }`.
        let var = if self.peek_token_is(TokenType::Ident) {
            self.next_token(); // advance cur to the identifier
            let id = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            Some(id)
        } else {
            None
        };

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        let expr = Expression::LoopExpression(LoopExpression {
            token,
            kind: LoopKind::Times {
                count: Box::new(count_expr),
                var,
            },
            body,
        });
        Some(Statement::Expression(ExpressionStatement {
            token: self.cur_token.clone(),
            expression: Some(expr),
        }))
    }

    // Builds one of the `ကနေ ... ထိပတ်` loop variants.
    //
    // We've already consumed `<source_expr>` (passed in as `source`). When this
    // function is entered, `cur_token` is the source's last token and
    // `peek_token` is `ကနေ`. Disambiguate after consuming `ကနေ`:
    //   - Ident followed by `ထိပတ်`         -> ForEach { source, var }
    //   - Ident followed by `,`              -> ForEachIndex { source, var, index }
    //   - otherwise                          -> Range; parse `<end>` expression,
    //                                            then expect `ထိပတ်`, optionally
    //                                            followed by a loop variable.
    fn parse_from_loop_from_expr(&mut self, token: Token, source: Expression) -> Option<Statement> {
        // Consume `ကနေ`. After this: cur is the token that followed `ကနေ`.
        if !self.expect_peek(TokenType::FromMarker) {
            return None;
        }
        self.next_token(); // advance past `ကနေ` to the token after it

        // cur is an identifier and peek is either ထိပတ် or Comma.
        if self.cur_token_is(TokenType::Ident)
            && (self.peek_token_is(TokenType::UntilLoop) || self.peek_token_is(TokenType::Comma))
        {
            let var = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };

            if self.peek_token_is(TokenType::Comma) {
                // ForEachIndex: <source> ကနေ <var>, <index> ထိပတ် { }
                self.next_token(); // consume var, now on `,`
                self.next_token(); // consume `,`, now on next ident
                if !self.cur_token_is(TokenType::Ident) {
                    let msg = format!(
                        "expected identifier for index after `,`, got {:?} ('{}')",
                        self.cur_token.token_type, self.cur_token.literal
                    );
                    let token = self.cur_token.clone();
                    let pos = self.token_position.saturating_sub(1);
                    self.emit_error(&token, pos, &msg);
                    return None;
                }
                let index = Identifier {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                };
                if !self.expect_peek(TokenType::UntilLoop) {
                    return None;
                }
                if !self.expect_peek(TokenType::LBrace) {
                    return None;
                }
                let body = self.parse_block_statement();
                let expr = Expression::LoopExpression(LoopExpression {
                    token,
                    kind: LoopKind::ForEachIndex {
                        source: Box::new(source),
                        var,
                        index,
                    },
                    body,
                });
                return Some(Statement::Expression(ExpressionStatement {
                    token: self.cur_token.clone(),
                    expression: Some(expr),
                }));
            } else {
                // ForEach: <source> ကနေ <var> ထိပတ် { }
                if !self.expect_peek(TokenType::UntilLoop) {
                    return None;
                }
                if !self.expect_peek(TokenType::LBrace) {
                    return None;
                }
                let body = self.parse_block_statement();
                let expr = Expression::LoopExpression(LoopExpression {
                    token,
                    kind: LoopKind::ForEach {
                        source: Box::new(source),
                        var,
                    },
                    body,
                });
                return Some(Statement::Expression(ExpressionStatement {
                    token: self.cur_token.clone(),
                    expression: Some(expr),
                }));
            }
        }

        // Range loop. Parse `<end>` as a full expression.
        // cur_token is the start of the end expression.
        let end_expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::UntilLoop) {
            return None;
        }

        // Optional loop variable: `ထိပတ် i { }` vs `ထိပတ် { }`.
        if self.peek_token_is(TokenType::Ident) {
            self.next_token(); // consume `ထိပတ်`, now on ident
            let var = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }
            let body = self.parse_block_statement();
            let expr = Expression::LoopExpression(LoopExpression {
                token,
                kind: LoopKind::RangeVar {
                    start: Box::new(source),
                    end: Box::new(end_expr),
                    var,
                },
                body,
            });
            return Some(Statement::Expression(ExpressionStatement {
                token: self.cur_token.clone(),
                expression: Some(expr),
            }));
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        let expr = Expression::LoopExpression(LoopExpression {
            token,
            kind: LoopKind::Range {
                start: Box::new(source),
                end: Box::new(end_expr),
            },
            body,
        });
        Some(Statement::Expression(ExpressionStatement {
            token: self.cur_token.clone(),
            expression: Some(expr),
        }))
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Look up the prefix parser for the current token.
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
            // The infix parser is looked up by the *peek* token, since the
            // peek token is what binds `left_exp` on its right side.
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

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let expresssion = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(TokenType::RParen) {
            return None;
        }
        expresssion
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

        let right = self.parse_expression(Precedence::Prefix).map(Box::new);

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
        let left = Some(Box::new(left));
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

    // LetStatement parser (singluar)
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }
        // If `expect_peek` succeeded, it has already advanced to the identifier token.
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest);

        // Consume the optional trailing semicolon (avoids a potential infinite loop).
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(LetStatement { token, name, value }))
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
                let token = self.cur_token.clone();
                let pos = self.token_position.saturating_sub(1);
                self.emit_error(&token, pos, &msg);
                None
            }
        }
    }

    // FloatLiteral parser
    pub fn parse_float_literal(&mut self) -> Option<Expression> {
        let value = self.cur_token.literal.parse::<f64>().ok();
        match value {
            Some(v) => Some(Expression::FloatLiteral(FloatLiteral {
                token: self.cur_token.clone(),
                value: v,
            })),
            None => {
                let msg = format!("could not parse {:?} as float", self.cur_token.literal);
                let token = self.cur_token.clone();
                let pos = self.token_position.saturating_sub(1);
                self.emit_error(&token, pos, &msg);
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

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    // parse functions in common way "fn something()"
    pub fn parse_function_declaration(&mut self) -> Option<Statement> {
        let fn_token = self.cur_token.clone(); // the ဖန်ရှင် token
        // Capture position up front so we can build the synthetic let_token after
        // `fn_token` has been moved into the FunctionLiteral below.
        let fn_line = fn_token.line;
        let fn_column = fn_token.column;

        // Next token is the function name
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        // Expect '('
        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        // Expect '{'
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        // Build the function literal expression
        let function_literal = Expression::FunctionLiteral(FunctionLiteral {
            token: fn_token,
            parameters,
            body,
        });

        // Desugar into a Let statement — preserve the function token's position
        // so any error reporting on this synthetic token still points at the
        // original `ဖန်ရှင်` keyword.
        let let_token = Token::new(TokenType::Let, "ထား".to_string(), fn_line, fn_column);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(LetStatement {
            token: let_token,
            name,
            value: Some(function_literal),
        }))
    }

    // FunctionLiteral parser ("let x = fn()" pattern)
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

    // Parses a comma-separated list of expressions ending with `end`
    // (RParen for call args, RBracket for array elements).
    pub fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut args = Vec::new();
        // Handle the empty case, e.g. `doSomething()`.
        if self.peek_token_is(end) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
            args.push(expr);
        }
        // Collect any additional comma-separated expressions.
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

    // ReturnStatement Parser
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(ReturnStatement {
            token,
            return_value,
        }))
    }

    // conditions parser
    pub fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();

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
            condition: condition.map(Box::new), // Box the condition when assembling the node.
            consequence,
            alternative,
        }))
    }

    pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        // `self.cur_token` is currently the opening `{`.
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

    // Parse array literal: [1, 2, 3]
    pub fn parse_array_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let elements = self.parse_expression_list(TokenType::RBRACKET);
        Some(Expression::ArrayLiteral(ArrayLiteral { token, elements }))
    }

    // Parse index expression: myArray[0].
    // This is an infix parser — `[` is the operator, with `left` as the
    // collection and the parsed expression as the index.
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

    // Parse hash literal: {"key": value, "key2": value2}.
    // The `{` is already consumed as cur_token.
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

    // Loop parsers
    // `ပတ် cond { }` or `ပတ် { }` (while / infinite loop).
    pub fn parse_while_or_inf_loop(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if self.peek_token_is(TokenType::LBrace) {
            // Infinite loop syntax: ပတ် {}
            self.next_token();
            let body = self.parse_block_statement();
            return Some(Expression::LoopExpression(LoopExpression {
                token,
                kind: LoopKind::Infinite,
                body,
            }));
        }

        // While loop syntax: ပတ် condition {}
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Expression::LoopExpression(LoopExpression {
            token,
            kind: LoopKind::While(Box::new(condition)),
            body,
        }))
    }

    // Helpers
    pub fn next_token(&mut self) {
        // `std::mem::replace` moves `peek_token` out into `cur_token` without
        // leaving `peek_token` in a moved (unusable) state. Cloning `peek_token`
        // would also work and read more cleanly, but is slightly less efficient.
        self.cur_token = std::mem::replace(
            &mut self.peek_token,
            Token::dummy(TokenType::Illegal, String::new()),
        );
        self.peek_token = self.lexer.next_token();
        self.token_position += 1;
    }

    // Note: returns a reference (`&Vec<String>`) rather than owning the errors.
    pub fn return_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} ('{}') instead.",
            t, self.peek_token.token_type, self.peek_token.literal
        );
        // The offending token is the peek token (the one we got instead of `t`).
        // `token_position` tracks cur_token's position + 1 (because Parser::new
        // calls next_token twice), so it already equals peek_token's actual
        // 1-based position in the stream.
        let token = self.peek_token.clone();
        let pos = self.token_position;
        self.emit_error(&token, pos, &msg);
    }

    /// Build a multi-line error message that includes:
    ///   - The line number and 1-based token position.
    ///   - The human-readable message.
    ///   - The offending source line, prefixed with ` {line_num} | `.
    ///   - A `^^^` pointer aligned to the offending token's column.
    ///
    /// `column` is still tracked on the Token (needed for the `^` pointer), but
    /// is deliberately NOT shown in the message header — the user asked for the
    /// format `Error at Line L, Token N: ...` instead.
    ///
    /// Special case: when the offending token is EOF (which sits one past the
    /// last line of input), we fall back to the last non-empty source line so
    /// the user still gets a visual pointer.
    fn emit_error(&mut self, token: &Token, token_position: usize, msg: &str) {
        let lines: Vec<&str> = self.lexer.input.lines().collect();
        // For EOF (empty literal, line past end of input) fall back to the last
        // non-empty line so the pointer is still useful.
        let is_eof = token.token_type == TokenType::Eof;
        let mut display_line = token.line;
        let mut display_column = token.column;
        if is_eof {
            // Walk backwards from the EOF's line to find a non-empty source line.
            // `token.line` is 1-based; `lines` is 0-based, so line N is at
            // `lines[N-1]`. Start at the EOF's line and go backwards.
            let mut idx = token.line;
            while idx > 0 {
                if let Some(l) = lines.get(idx - 1)
                    && !l.is_empty()
                {
                    display_line = idx;
                    // Point the caret just past the end of the line.
                    display_column = l.chars().count() + 1;
                    break;
                }
                idx -= 1;
            }
            if idx == 0 {
                // Couldn't find a non-empty line; just use line 1.
                display_line = 1;
                display_column = 1;
            }
        }

        // Use display_line in the header too, so EOF errors report the line
        // the user actually sees in the source pointer (not the empty line
        // the EOF token technically sits on).
        let mut formatted = format!(
            "Error at Line {}, Token {}: {}",
            display_line, token_position, msg
        );

        let line_idx = display_line.saturating_sub(1);
        if let Some(source_line) = lines.get(line_idx) {
            let line_num_str = display_line.to_string();
            formatted.push_str(&format!("\n {} | {}", line_num_str, source_line));

            // Build the padding so the `^` lines up with the token's column.
            // Layout: ` {line_num} | {source_line}\n` then spaces under the
            // prefix, then spaces/tabs matching the source line up to the token.
            let mut padding = String::new();
            padding.push_str(&" ".repeat(line_num_str.len() + 3));

            // Column is 1-based and counts characters (not bytes). Skip the
            // first `column - 1` chars of the source line, preserving tabs.
            for c in source_line.chars().take(display_column.saturating_sub(1)) {
                if c == '\t' {
                    padding.push('\t');
                } else {
                    padding.push(' ');
                }
            }

            // For EOF the literal is empty, so use at least one caret.
            let token_len = if is_eof {
                1
            } else {
                token.literal.chars().count().max(1)
            };
            let carets = "^".repeat(token_len);
            formatted.push_str(&format!("\n{}{}", padding, carets));
        }

        self.errors.push(formatted);
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

    // `expect_peek` advances to the next token if it matches `t`, otherwise
    // records a parse error.
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
            "no prefix parse function for {:?} ('{}') found",
            self.cur_token.token_type, self.cur_token.literal
        );
        // `token_position` is 1 ahead of cur_token's actual position (see
        // the note in peek_error), so subtract 1 to get cur_token's position.
        let token = self.cur_token.clone();
        let pos = self.token_position.saturating_sub(1);
        self.emit_error(&token, pos, &msg);
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::from_token_type(self.peek_token.token_type)
    }

    pub fn cur_precedence(&self) -> Precedence {
        Precedence::from_token_type(self.cur_token.token_type)
    }
}
