#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,
    Float,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Percent,

    Lt,
    Gt,
    LtEq,
    GtEq,

    Eq,
    NotEq,

    And,
    Or,

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

    // Loops
    Loop,
    TimesLoop,  // ခါပတ် — N-times loop marker
    FromMarker, // ကနေ — "from" marker for for-each / range loops
    UntilLoop,  // ထိပတ် — "until" loop marker
    Break,      // ရပ် — break out of the enclosing loop
    Continue,   // ကျော် — skip to the next iteration of the enclosing loop
    MyanmarInt,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }

    /// Helper for creating placeholder tokens (e.g. before the first `next_token`
    /// call). Line/column are 0 so any accidental use in an error message is
    /// visually obvious.
    pub fn dummy(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
            line: 0,
            column: 0,
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
            "ပတ်" => TokenType::Loop,
            "ခါပတ်" => TokenType::TimesLoop,
            "ကနေ" => TokenType::FromMarker,
            "ထိပတ်" => TokenType::UntilLoop,
            "ရပ်" => TokenType::Break,
            "ကျော်" => TokenType::Continue,
            _ => TokenType::Ident,
        }
    }
}
