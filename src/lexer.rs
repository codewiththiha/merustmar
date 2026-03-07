use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    // rust's char have built in unicode support and we packed in Option because we want to have
    // None , or Some obviously None is what we want actually
    ch: Option<char>,
    // usize is unsigned integer but it's smarter its limit byte is built on the client CPU's
    // architecture
    read_position: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            ch: None,
            read_position: 0,
            position: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
            self.position = self.read_position;
        } else {
            if let Some(ch) = self.input[self.read_position..].chars().next() {
                self.ch = Some(ch);
                self.position = self.read_position;
                self.read_position += ch.len_utf8();
            } else {
                self.ch = None;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token::new(TokenType::Assign, "=".to_string()),
            Some('။') => Token::new(TokenType::Semicolon, "။".to_string()),
            Some('(') => Token::new(TokenType::LParen, "(".to_string()),
            Some(')') => Token::new(TokenType::RParen, ")".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some('{') => Token::new(TokenType::LBrace, "{".to_string()),
            Some('}') => Token::new(TokenType::RBrace, "}".to_string()),
            // TODO none should not create a new token tho
            // index 0 having none can cause an error not a problem tho adding skip space can fix
            None => Token::new(TokenType::Eof, "".to_string()),
            _ => Token::new(
                TokenType::Illegial,
                // TODO study this pattern
                self.ch.map(|c| c.to_string()).unwrap_or_default(),
            ),
        };
        self.read_char();
        token
    }
}
