use crate::lexer::{lexer::Lexer, token::TokenType};

#[test]
fn test_next_token() {
    let input = "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);

        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10; 10 != 9;
        ";
    struct TestCase {
        expected_type: TokenType,
        expecred_literal: String,
    }

    let tests = [
        TestCase {
            expected_type: TokenType::LET,
            expecred_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expecred_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expecred_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expecred_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expecred_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expecred_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::FUNCTION,
            expecred_literal: "fn".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expecred_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::COMMA,
            expecred_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expecred_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expecred_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::PLUS,
            expecred_literal: "+".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expecred_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expecred_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "result".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expecred_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expecred_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::COMMA,
            expecred_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expecred_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expecred_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::BANG,
            expecred_literal: "!".to_string(),
        },
        TestCase {
            expected_type: TokenType::MINUS,
            expecred_literal: "-".to_string(),
        },
        TestCase {
            expected_type: TokenType::SLASH,
            expecred_literal: "/".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASTERISK,
            expecred_literal: "*".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::LT,
            expecred_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::GT,
            expecred_literal: ">".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::IF,
            expecred_literal: "if".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expecred_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::LT,
            expecred_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expecred_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expecred_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::RETURN,
            expecred_literal: "return".to_string(),
        },
        TestCase {
            expected_type: TokenType::TRUE,
            expecred_literal: "true".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expecred_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::ELSE,
            expecred_literal: "else".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expecred_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::RETURN,
            expecred_literal: "return".to_string(),
        },
        TestCase {
            expected_type: TokenType::FALSE,
            expecred_literal: "false".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expecred_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::EQ,
            expecred_literal: "==".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::NotEq,
            expecred_literal: "!=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expecred_literal: "9".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expecred_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::EOF,
            expecred_literal: "".to_string(),
        },
    ];

    let mut l = Lexer::new(input.to_owned());
    for tt in tests {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt.expected_type,
            "test - tokentype wrong. extected={:?}, got={:?} - literal_e: {}",
            tok.token_type, tt.expected_type, tok.literal,
        );
        assert_eq!(
            tok.literal, tt.expecred_literal,
            "test - literal wrong. extected={}, got={}",
            tok.literal, tt.expecred_literal,
        )
    }
}
