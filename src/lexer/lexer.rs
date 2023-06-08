use super::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();

        l
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut literal = String::from(ch);
                    literal.push(self.ch);
                    tok = Token::new(TokenType::EQ, literal);
                } else {
                    tok = Token::new(TokenType::ASSIGN, self.ch.to_string());
                }
            }
            '+' => tok = Token::new(TokenType::PLUS, self.ch.to_string()),
            '-' => tok = Token::new(TokenType::MINUS, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut literal = String::from(ch);
                    literal.push(self.ch);
                    tok = Token::new(TokenType::NotEq, literal);
                } else {
                    tok = Token::new(TokenType::BANG, self.ch.to_string());
                }
            }
            '/' => tok = Token::new(TokenType::SLASH, self.ch.to_string()),
            '*' => tok = Token::new(TokenType::ASTERISK, self.ch.to_string()),
            '<' => tok = Token::new(TokenType::LT, self.ch.to_string()),
            '>' => tok = Token::new(TokenType::GT, self.ch.to_string()),
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            ',' => tok = Token::new(TokenType::COMMA, self.ch.to_string()),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch.to_string()),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch.to_string()),
            '.' => {
                if self.peek_char() == '.' {
                    let ch = self.ch;
                    self.read_char();
                    let mut literal = String::from(ch);
                    literal.push(ch);
                    tok = Token::new(TokenType::Spreed, literal)
                } else {
                    tok = Token::new(TokenType::Dot, self.ch.to_string())
                }
            }
            '\0' => tok.token_type = TokenType::EOF,
            _ => {
                if is_letter(self.ch) {
                    tok.literal = self.read_indentifier();
                    tok.token_type = lookup_ident(&tok.literal);
                    return tok;
                } else if is_digit(self.ch) {
                    tok.literal = self.read_number();
                    tok.token_type = TokenType::INT;
                    return tok;
                } else {
                    tok = Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        }

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while ['\t', '\n', '\r', ' '].contains(&self.ch) {
            self.read_char()
        }
    }

    fn read_indentifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
