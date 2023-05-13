#[derive(Debug, Default, PartialEq, Eq)]
pub enum TokenType {
    #[default]
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => return TokenType::FUNCTION,
        "let" => return TokenType::LET,
        "true" => return TokenType::TRUE,
        "false" => return TokenType::FALSE,
        "if" => return TokenType::IF,
        "else" => return TokenType::ELSE,
        "return" => return TokenType::RETURN,
        _ => return TokenType::IDENT,
    }
}

#[derive(Default)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        Token {
            token_type,
            literal: ch.to_string(),
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => return TokenType::FUNCTION,
            "let" => return TokenType::LET,
            "true" => return TokenType::TRUE,
            "false" => return TokenType::FALSE,
            "if" => return TokenType::IF,
            "else" => return TokenType::ELSE,
            "return" => return TokenType::RETURN,
            _ => return TokenType::IDENT,
        }
    }
}
