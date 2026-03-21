use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
    // rust's char have built in unicode support and we packed in Option because we want to have
    // None , or Some obviously None is what we want actually
    pub ch: Option<char>,
    // usize is unsigned integer but it's smarter its limit byte is built on the client CPU's
    // architecture
    pub read_position: usize,
    pub position: usize,
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

    pub fn read_number(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.ch {
            if is_digit(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    pub fn read_string(&mut self) -> String {
        let start_position = self.position.saturating_add(1);
        loop {
            self.read_char();
            if self.ch.unwrap() == '"' || self.ch == None {
                break;
            }
        }
        self.input[start_position..self.position].to_string()
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

    pub fn read_identifier(&mut self) -> String {
        let start_position = self.position;

        while let Some(ch) = self.ch {
            if is_letter(ch) {
                self.read_char();
            } else {
                break;
            }
        }

        return self.input[start_position..self.position].to_string();
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            self.input[self.read_position..].chars().next()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, "=".to_string())
                }
            }
            Some('။') => Token::new(TokenType::Semicolon, "။".to_string()),
            Some('(') => Token::new(TokenType::LParen, "(".to_string()),
            Some(')') => Token::new(TokenType::RParen, ")".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some('{') => Token::new(TokenType::LBrace, "{".to_string()),
            Some('}') => Token::new(TokenType::RBrace, "}".to_string()),
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            Some('-') => Token::new(TokenType::Minus, "-".to_string()),
            Some('/') => Token::new(TokenType::Slash, "/".to_string()),
            Some('*') => Token::new(TokenType::Asterisk, "*".to_string()),
            Some('>') => Token::new(TokenType::Gt, ">".to_string()),
            Some('<') => Token::new(TokenType::Lt, "<".to_string()),
            Some('"') => Token::new(TokenType::String, self.read_string()),
            // NOTICE how in this case we used explict return , cuz if we don't do that we might
            // end up scrolling to the end and execute self.read_char() which will increase the
            // read_position and cause conflicts and bugs (::D just experienced that for 1 hr)
            // as i thought implict and explict return act like same so REMEMBER this
            // implict return in most of the case actually all of the case except this match case
            // only able to do at the end so no problem , so this confusion is unique
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                // TODO noticee how rust handled literal(String) and cast to &str (the lookup_ident
                // only accept &str not &string) research about that!
                let token_type = Token::lookup_ident(&literal);
                return Token::new(token_type, literal);
            }
            Some(ch) if is_digit(ch) => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, literal);
            }

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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    if ch == '။' {
        return false;
    }
    // ASCII: letters,underscore
    if ch.is_ascii() {
        return ch.is_ascii_alphabetic() || ch == '_';
    }

    // Myanmar Unicode block (U+1000..=U+109F)
    // This includes consonants, vowels, digits, AND combining marks
    // All of these should be valid in Myanmar identifiers/keywords
    if ('\u{1000}'..='\u{109F}').contains(&ch) {
        return true;
    }

    // Fallback: other scripts (Greek, Arabic, etc.)
    ch.is_alphabetic() || ch == '_'
}

pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
