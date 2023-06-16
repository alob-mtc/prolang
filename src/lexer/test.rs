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

        !-*/5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10; 10 != 9;
        .;
        ..
        "
    .to_string();
    struct TestCase {
        expected_type: TokenType,
        expected_literal: String,
    }

    let tests = [
        TestCase {
            expected_type: TokenType::LET,
            expected_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expected_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expected_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::FUNCTION,
            expected_literal: "fn".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::COMMA,
            expected_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::PLUS,
            expected_literal: "+".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::LET,
            expected_literal: "let".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "result".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASSIGN,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::COMMA,
            expected_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::IDENT,
            expected_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::BANG,
            expected_literal: "!".to_string(),
        },
        TestCase {
            expected_type: TokenType::MINUS,
            expected_literal: "-".to_string(),
        },
        TestCase {
            expected_type: TokenType::ASTERISK,
            expected_literal: "*".to_string(),
        },
        TestCase {
            expected_type: TokenType::SLASH,
            expected_literal: "/".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::LT,
            expected_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::GT,
            expected_literal: ">".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::IF,
            expected_literal: "if".to_string(),
        },
        TestCase {
            expected_type: TokenType::LPAREN,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::LT,
            expected_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::RPAREN,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::RETURN,
            expected_literal: "return".to_string(),
        },
        TestCase {
            expected_type: TokenType::TRUE,
            expected_literal: "true".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::ELSE,
            expected_literal: "else".to_string(),
        },
        TestCase {
            expected_type: TokenType::LBRACE,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::RETURN,
            expected_literal: "return".to_string(),
        },
        TestCase {
            expected_type: TokenType::FALSE,
            expected_literal: "false".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RBRACE,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::EQ,
            expected_literal: "==".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::NotEq,
            expected_literal: "!=".to_string(),
        },
        TestCase {
            expected_type: TokenType::INT,
            expected_literal: "9".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Dot,
            expected_literal: ".".to_string(),
        },
        TestCase {
            expected_type: TokenType::SEMICOLON,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Spreed,
            expected_literal: "..".to_string(),
        },
        TestCase {
            expected_type: TokenType::EOF,
            expected_literal: "".to_string(),
        },
    ];

    let mut l = Lexer::new(input);
    for tt in tests {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt.expected_type,
            "test - tokentype wrong. extected={:?}, got={:?} - literal_e: {}",
            tok.token_type, tt.expected_type, tok.literal,
        );
        assert_eq!(
            tok.literal, tt.expected_literal,
            "test - literal wrong. extected={}, got={}",
            tok.literal, tt.expected_literal,
        )
    }
}
