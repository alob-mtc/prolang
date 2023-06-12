#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub enum TokenType {
    #[default]
    None,

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
    Spreed,

    Dot,

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
    FOR,
    IN,
}

#[derive(Default, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: (usize, usize), //(line, column)
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "for" => TokenType::FOR,
        "in" => TokenType::IN,
        _ => TokenType::IDENT,
    }
}

impl Token {
    pub fn new(token_type: TokenType, ch: String, position: (usize, usize)) -> Self {
        Self {
            token_type,
            literal: ch,
            position,
        }
    }
    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
