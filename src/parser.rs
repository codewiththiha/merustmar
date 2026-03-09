use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::new(TokenType::Illegial, "".to_string()),
            peek_token: Token::new(TokenType::Illegial, "".to_string()),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
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
            _ => None,
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

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(ReturnStatement {
            token,
            return_value: None,
        }));
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
}

