#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
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
    EQ,
    NotEq,

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

#[derive(Default, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
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

impl Token {
    pub fn new(token_type: TokenType, ch: String) -> Self {
        Self {
            token_type,
            literal: ch,
        }
    }
}
