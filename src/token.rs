enum TokenType {
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
}

fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => return TokenType::FUNCTION,
        "let" => return TokenType::LET,
        _ => return TokenType::IDENT,
    }
}

struct Token {
    token_type: TokenType,
    literal: char,
}
