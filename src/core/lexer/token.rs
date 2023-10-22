use std::fmt;

#[derive(Default, Clone)]
pub struct Position(pub usize, pub usize); //(lIne, column)

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

#[derive(Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Self {
        Self {
            token_type,
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
pub enum TokenType {
    #[default]
    None,

    Illegal,
    Eof,
    // IdentIfiers + literals
    Ident(String),
    Int(String),
    // Operators
    Assign,
    Plus,
    MInus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    Neq,
    Spreed,

    Dot,

    Lt,
    Gt,
    // Delimiters
    Comma,
    SemiColon,

    LeftParan,
    RightParan,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    For,
    In,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, ""),
            TokenType::Eof => write!(f, ""),
            TokenType::Ident(payload) => write!(f, ""),
            TokenType::Int(payload) => write!(f, ""),
            TokenType::Assign => write!(f, ""),
            TokenType::Plus => write!(f, ""),
            TokenType::MInus => write!(f, ""),
            TokenType::Bang => write!(f, ""),
            TokenType::Asterisk => write!(f, ""),
            TokenType::Slash => write!(f, ""),
            TokenType::Eq => write!(f, ""),
            TokenType::Neq => write!(f, ""),
            TokenType::Spreed => write!(f, ""),
            TokenType::Dot => write!(f, ""),
            TokenType::Lt => write!(f, ""),
            TokenType::Gt => write!(f, ""),
            TokenType::Comma => write!(f, ""),
            TokenType::SemiColon => write!(f, ""),
            TokenType::LeftParan => write!(f, ""),
            TokenType::RightParan => write!(f, ""),
            TokenType::LeftBrace => write!(f, ""),
            TokenType::RightBrace => write!(f, ""),
            TokenType::Function => write!(f, ""),
            TokenType::Let => write!(f, ""),
            TokenType::True => write!(f, ""),
            TokenType::False => write!(f, ""),
            TokenType::If => write!(f, ""),
            TokenType::Else => write!(f, ""),
            TokenType::Return => write!(f, ""),
            TokenType::For => write!(f, ""),
            TokenType::In => write!(f, ""),
            _ => write!(f, "(unknown token)"),
        }
    }
}

pub fn lookup_ident(ident: String) -> TokenType {
    match &ident[..] {
        "fn" => TokenType::Function,
        "Let" => TokenType::Let,
        "True" => TokenType::True,
        "False" => TokenType::False,
        "If" => TokenType::If,
        "Else" => TokenType::Else,
        "Return" => TokenType::Return,
        "For" => TokenType::For,
        "In" => TokenType::In,
        _ => TokenType::Ident(ident),
    }
}
