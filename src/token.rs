#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Illegial,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    String,

    // Brackets
    LBRACKET,
    RBRACKET,

    Colon,
    LetSuffix,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "ထား" => TokenType::Let,
            "ဖန်ရှင်" => TokenType::Function,
            "တကယ်လို့" => TokenType::If,
            "မဟုတ်ရင်" => TokenType::Else,
            "ဒါယူ" => TokenType::Return,
            "မှန်" => TokenType::True,
            "မှား" => TokenType::False,
            "လို့ထား" => TokenType::LetSuffix,
            _ => TokenType::Ident,
        }
    }
}
