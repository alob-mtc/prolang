use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 1,
            ch: '\0',
        };

        l.read_char();

        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch {
            '=' => tok = Token::new(TokenType::ASSIGN, self.ch),
            '+' => tok = Token::new(TokenType::PLUS, self.ch),
            '-' => tok = Token::new(TokenType::MINUS, self.ch),
            '!' => tok = Token::new(TokenType::BANG, self.ch),
            '/' => tok = Token::new(TokenType::SLASH, self.ch),
            '*' => tok = Token::new(TokenType::ASTERISK, self.ch),
            '<' => tok = Token::new(TokenType::LT, self.ch),
            '>' => tok = Token::new(TokenType::GT, self.ch),
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch),
            ',' => tok = Token::new(TokenType::COMMA, self.ch),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch),
            '\0' => tok.token_type = TokenType::EOF,
            _ => {
                if is_letter(self.ch) {
                    tok.literal = self.read_indentifier();
                    tok.token_type = Token::lookup_ident(&tok.literal)
                } else if is_digit(self.ch) {
                    tok.literal = self.read_number();
                    tok.token_type = TokenType::INT
                } else {
                    tok = Token::new(TokenType::ILLEGAL, self.ch)
                }
            }
        }

        return tok;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == '\0' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn read_indentifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        (&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        (&self.input[position..self.position]).to_string()
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        assert_eq!(1, 1)
    }
}
