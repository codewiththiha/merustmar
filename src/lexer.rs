use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    pub input: &'a str,
    // Rust's `char` is a full Unicode scalar value. We wrap it in Option so
    // `None` cleanly represents "past the end of input".
    pub ch: Option<char>,
    // usize is an unsigned integer whose size matches the host CPU's pointer width.
    pub read_position: usize,
    pub position: usize,
    // 1-based line number and column for the *current* `ch` mostly to track "^" position
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            ch: None,
            read_position: 0,
            position: 0,
            line: 1,
            column: 1, // 1-based: the first char of the input is at line 1, column 1.
        };
        lexer.read_char();
        lexer
    }

    pub fn read_myanmar_number(&mut self) -> String {
        let mut val: i64 = 0;
        while let Some(ch) = self.ch {
            if is_myanmar_digit(ch) {
                let digit = ch as u32 - '\u{1040}' as u32;
                val = val * 10 + digit as i64;
                self.read_char();
            } else {
                break;
            }
        }
        val.to_string() // Converts ၅ back to "5" for the parser
    }

    pub fn read_number(&mut self) -> (String, bool) {
        let start = self.position;
        let mut is_float = false;

        while let Some(ch) = self.ch {
            if is_digit(ch) {
                self.read_char();
            } else if ch == '.' && !is_float {
                // Check if the next character is also a digit
                if let Some(next_ch) = self.peek_char()
                    && is_digit(next_ch)
                {
                    is_float = true;
                    self.read_char();
                    continue;
                }
                break;
            } else {
                break;
            }
        }
        (self.input[start..self.position].to_string(), is_float)
    }

    pub fn read_string(&mut self) -> String {
        let mut string = String::new();
        loop {
            self.read_char();
            match self.ch {
                Some('"') | None => break,
                Some('\\') => {
                    self.read_char();
                    match self.ch {
                        Some('n') => string.push('\n'),
                        Some('t') => string.push('\t'),
                        Some('"') => string.push('"'),
                        Some('\\') => string.push('\\'),
                        Some(ch) => {
                            string.push('\\');
                            string.push(ch);
                        }
                        None => break,
                    }
                }
                Some(ch) => string.push(ch),
            }
        }
        string
    }

    pub fn read_char(&mut self) {
        // Update line/column based on the character we are *leaving behind*.
        // The new `ch` will be the next character in the stream; its position
        // is therefore (line, column) after the update.
        if let Some(ch) = self.ch {
            if ch == '\n' {
                self.line += 1;
                self.column = 1; // Reset to 1-based for the new line.
            } else {
                self.column += 1;
            }
        }

        if self.read_position >= self.input.len() {
            self.ch = None;
            self.position = self.read_position;
        } else if let Some(ch) = self.input[self.read_position..].chars().next() {
            self.ch = Some(ch);
            self.position = self.read_position;
            self.read_position += ch.len_utf8();
        } else {
            self.ch = None;
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let start_position = self.position;

        while let Some(ch) = self.ch {
            if is_letter(ch) || is_digit(ch) || is_myanmar_digit(ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[start_position..self.position].to_string()
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input[self.read_position..].chars().next()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        // Capture the start line/column of the current token BEFORE we consume
        // any characters — these coordinates point at `self.ch`.
        let tok_line = self.line;
        let tok_column = self.column;

        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Assign, "=".to_string(), tok_line, tok_column)
                }
            }
            Some(ch) if is_myanmar_digit(ch) => {
                // Myanmar digits (၀..၉) are lexed as a regular Int token whose
                // literal is the equivalent Arabic-digit string. This means
                // `၅ + ၃` works anywhere `5 + 3` would, with no special casing.
                let literal = self.read_myanmar_number();
                return Token::new(TokenType::Int, literal, tok_line, tok_column);
            }
            Some('။') => Token::new(TokenType::Semicolon, "။".to_string(), tok_line, tok_column),
            Some('(') => Token::new(TokenType::LParen, "(".to_string(), tok_line, tok_column),
            Some(')') => Token::new(TokenType::RParen, ")".to_string(), tok_line, tok_column),
            Some(',') => Token::new(TokenType::Comma, ",".to_string(), tok_line, tok_column),
            Some('+') => Token::new(TokenType::Plus, "+".to_string(), tok_line, tok_column),
            Some('{') => Token::new(TokenType::LBrace, "{".to_string(), tok_line, tok_column),
            Some('}') => Token::new(TokenType::RBrace, "}".to_string(), tok_line, tok_column),
            Some('[') => Token::new(TokenType::LBRACKET, "[".to_string(), tok_line, tok_column),
            Some(']') => Token::new(TokenType::RBRACKET, "]".to_string(), tok_line, tok_column),
            Some(':') => Token::new(TokenType::Colon, ":".to_string(), tok_line, tok_column),
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Bang, "!".to_string(), tok_line, tok_column)
                }
            }
            Some('-') => Token::new(TokenType::Minus, "-".to_string(), tok_line, tok_column),
            Some('/') => Token::new(TokenType::Slash, "/".to_string(), tok_line, tok_column),
            Some('*') => Token::new(TokenType::Asterisk, "*".to_string(), tok_line, tok_column),
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::GtEq, ">=".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Gt, ">".to_string(), tok_line, tok_column)
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::new(TokenType::LtEq, "<=".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Lt, "<".to_string(), tok_line, tok_column)
                }
            }
            Some('"') => Token::new(TokenType::String, self.read_string(), tok_line, tok_column),
            Some('%') => Token::new(TokenType::Percent, "%".to_string(), tok_line, tok_column),
            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char();
                    Token::new(TokenType::And, "&&".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Illegal, "&".to_string(), tok_line, tok_column)
                }
            }
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char();
                    Token::new(TokenType::Or, "||".to_string(), tok_line, tok_column)
                } else {
                    Token::new(TokenType::Illegal, "|".to_string(), tok_line, tok_column)
                }
            }
            // We use an explicit `return` here. Without it, control would fall
            // through to the `self.read_char()` call at the end of `next_token`,
            // which would advance the position past the identifier we just read
            // and corrupt the next token. Implicit returns only work at the end
            // of a function body, so this match arm needs an explicit return.
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                // `lookup_ident` takes `&str`, and `&String` auto-derefs to `&str`.
                let token_type = Token::lookup_ident(&literal);
                return Token::new(token_type, literal, tok_line, tok_column);
            }
            Some(ch) if is_digit(ch) => {
                let (literal, is_float) = self.read_number();
                if is_float {
                    return Token::new(TokenType::Float, literal, tok_line, tok_column);
                } else {
                    return Token::new(TokenType::Int, literal, tok_line, tok_column);
                }
            }
            None => Token::new(TokenType::Eof, String::new(), tok_line, tok_column),
            _ => Token::new(
                TokenType::Illegal,
                self.ch.map(|c| c.to_string()).unwrap_or_default(),
                tok_line,
                tok_column,
            ),
        };
        self.read_char();
        token
    }

    // Skips whitespace and comments. Comment forms supported:
    //   // line comment
    //   #  hash comment
    //   /* block comment */
    fn skip_whitespace(&mut self) {
        loop {
            // Spaces, tabs, newlines, and invisible Unicode formatters.
            while let Some(ch) = self.ch {
                if ch.is_whitespace() || ch == '\u{200B}' || ch == '\u{200C}' || ch == '\u{200D}' {
                    self.read_char();
                } else {
                    break;
                }
            }

            // `//` line comment
            if self.ch == Some('/') && self.peek_char() == Some('/') {
                self.read_char();
                self.read_char();
                while let Some(ch) = self.ch {
                    if ch == '\n' {
                        break;
                    }
                    self.read_char();
                }
                continue;
            }

            // /* block comment */
            if self.ch == Some('/') && self.peek_char() == Some('*') {
                self.read_char();
                self.read_char();
                while self.ch.is_some() {
                    if self.ch == Some('*') && self.peek_char() == Some('/') {
                        self.read_char();
                        self.read_char();
                        break;
                    }
                    self.read_char();
                }
                continue;
            }

            // `#` hash comment
            if self.ch == Some('#') {
                self.read_char();
                while let Some(ch) = self.ch {
                    if ch == '\n' {
                        break;
                    }
                    self.read_char();
                }
                continue;
            }

            break;
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    if ch == '။' {
        return false;
    }
    if ch.is_ascii() {
        return ch.is_ascii_alphabetic() || ch == '_';
    }

    if ('\u{1000}'..='\u{109F}').contains(&ch) {
        // Exclude Myanmar digits so they are tokenized as numbers instead,
        // matching how we treat `။` as a non-letter above.
        if is_myanmar_digit(ch) {
            return false;
        }
        return true;
    }
    ch.is_alphabetic() || ch == '_'
}

pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

pub fn is_myanmar_digit(ch: char) -> bool {
    ('\u{1040}'..='\u{1049}').contains(&ch)
}
