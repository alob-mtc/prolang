use crate::core::lexer::{lexer::Lexer, token::TokenType};

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
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::Function,
            expected_literal: "fn".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftParan,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::Comma,
            expected_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightParan,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftBrace,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "x".to_string(),
        },
        TestCase {
            expected_type: TokenType::Plus,
            expected_literal: "+".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "y".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightBrace,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "resuLt".to_string(),
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "add".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftParan,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "five".to_string(),
        },
        TestCase {
            expected_type: TokenType::Comma,
            expected_literal: ",".to_string(),
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "ten".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightParan,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Bang,
            expected_literal: "!".to_string(),
        },
        TestCase {
            expected_type: TokenType::MInus,
            expected_literal: "-".to_string(),
        },
        TestCase {
            expected_type: TokenType::Asterisk,
            expected_literal: "*".to_string(),
        },
        TestCase {
            expected_type: TokenType::Slash,
            expected_literal: "/".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::Lt,
            expected_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::Gt,
            expected_literal: ">".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::If,
            expected_literal: "If".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftParan,
            expected_literal: "(".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
        },
        TestCase {
            expected_type: TokenType::Lt,
            expected_literal: "<".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightParan,
            expected_literal: ")".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftBrace,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::Return,
            expected_literal: "Return".to_string(),
        },
        TestCase {
            expected_type: TokenType::True,
            expected_literal: "True".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightBrace,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::ELSE,
            expected_literal: "else".to_string(),
        },
        TestCase {
            expected_type: TokenType::LeftBrace,
            expected_literal: "{".to_string(),
        },
        TestCase {
            expected_type: TokenType::Return,
            expected_literal: "Return".to_string(),
        },
        TestCase {
            expected_type: TokenType::False,
            expected_literal: "False".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::RightBrace,
            expected_literal: "}".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::Eq,
            expected_literal: "==".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
        },
        TestCase {
            expected_type: TokenType::Neq,
            expected_literal: "!=".to_string(),
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "9".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Dot,
            expected_literal: ".".to_string(),
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
        },
        TestCase {
            expected_type: TokenType::Spreed,
            expected_literal: "..".to_string(),
        },
        TestCase {
            expected_type: TokenType::Eof,
            expected_literal: "".to_string(),
        },
    ];

    let mut l = Lexer::new(input);
    for tt in tests {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt.expected_type,
            "test - tokentype wrong. extected={:?}, got={:?} - literal_e: {}",
            tok.token_type, tt.expected_type, tok,
        );
        assert_eq!(
            tok.literal, tt.expected_literal,
            "test - literal wrong. extected={}, got={}",
            tok.literal, tt.expected_literal,
        )
    }
}

#[test]
fn test_line_and_column() {
    let input = "
let five = 5;
// this is comment
let a = 10;
        "
    .to_string();
    struct TestCase {
        expected_type: TokenType,
        expected_literal: String,
        expected_column: usize,
        expected_line: usize,
    }

    let tests = [
        TestCase {
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
            expected_column: 3,
            expected_line: 2,
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "five".to_string(),
            expected_column: 8,
            expected_line: 2,
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
            expected_column: 10,
            expected_line: 2,
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "5".to_string(),
            expected_column: 12,
            expected_line: 2,
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
            expected_column: 13,
            expected_line: 2,
        },
        TestCase {
            expected_type: TokenType::Let,
            expected_literal: "Let".to_string(),
            expected_column: 3,
            expected_line: 4,
        },
        TestCase {
            expected_type: TokenType::Ident,
            expected_literal: "a".to_string(),
            expected_column: 5,
            expected_line: 4,
        },
        TestCase {
            expected_type: TokenType::Assign,
            expected_literal: "=".to_string(),
            expected_column: 7,
            expected_line: 4,
        },
        TestCase {
            expected_type: TokenType::Int,
            expected_literal: "10".to_string(),
            expected_column: 10,
            expected_line: 4,
        },
        TestCase {
            expected_type: TokenType::SemiColon,
            expected_literal: ";".to_string(),
            expected_column: 11,
            expected_line: 4,
        },
        TestCase {
            expected_type: TokenType::Eof,
            expected_literal: "".to_string(),
            expected_column: 0,
            expected_line: 0,
        },
    ];

    let mut l = Lexer::new(input);
    for tt in tests {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt.expected_type,
            "test - tokentype wrong. extected={:?}, got={:?} - literal_e: {}",
            tok.token_type, tt.expected_type, tok,
        );
        assert_eq!(
            tok, tt.expected_literal,
            "test - literal wrong. extected={}, got={}",
            tok.literal, tt.expected_literal,
        );

        let position = (tt.expected_line, tt.expected_column);
        assert_eq!(
            tok.position, position,
            "test - wrong position. expected={}, got={}",
            tok.position, position
        );
    }
}
