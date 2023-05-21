use std::fmt::format;

use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

use super::ast::{ExpressionDefault, Identifier, LetStatement, Program, Statement};

struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let p = Parser {
            l,
            errors: vec![],
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        return p;
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };
        while !self.cur_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token()
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            value: Box::new(ExpressionDefault {}),
            name: Identifier::default(),
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: "".to_string(),
        };
        self.cur_token.literal.clone_into(&mut stmt.name.value);

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: skipping the expressions until we encounter ;
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = LetStatement {
            token: self.cur_token.clone(),
            value: Box::new(ExpressionDefault {}),
            name: Identifier::default(),
        };

        self.next_token();

        // TODO: skipping the expressions until we encounter ;
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }

        Some(Box::new(stmt))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }
    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }
    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        }
        self.peek_error(t);
        return false;
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format(format_args!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type,
        ));
        self.errors.push(msg)
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{
        lexer::lexer::Lexer,
        parser::ast::{Node, Statement},
    };

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program().expect("parse_program() return none");
        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        struct TestCase {
            expected_identifier: String,
        }

        let tests = vec![
            TestCase {
                expected_identifier: "x".to_string(),
            },
            TestCase {
                expected_identifier: "y".to_string(),
            },
            TestCase {
                expected_identifier: "foobar".to_string(),
            },
        ];

        let mut i = 0;
        for tt in tests {
            let stmt = program.statements.get(i).unwrap();
            let_statemnt(stmt, tt.expected_identifier);
            i += 1;
        }
    }

    #[test]
    fn test_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993322;
        ";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program().expect("parse_program() return none");
        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        for stmt in program.statements {
            assert_eq!(
                stmt.token_literal(),
                "return",
                "return_stmt.token_literal not 'return', got {}",
                stmt.token_literal()
            )
        }
    }

    fn let_statemnt(s: &Box<dyn Statement>, name: String) {
        assert_eq!(
            s.token_literal(),
            "let",
            "s.token_literal not 'let' got={}",
            s.token_literal()
        );
        let let_stmt = s.get_let().unwrap();
        assert_eq!(
            let_stmt.name.value, name,
            "letStmt.Name.Value not '{}'. got={}",
            name, let_stmt.name.value
        );
        assert_eq!(
            let_stmt.name.token_literal(),
            name,
            "letStmt.Name.Value not '{}'. got={}",
            name,
            let_stmt.name.value
        )
    }

    fn chack_parser_errors(p: &Parser) -> bool {
        let errs: &Vec<String> = p.errors();
        if errs.len() == 0 {
            return false;
        }
        println!("parser has errors: {}", errs.len());
        for err in errs {
            println!("parser error: {}", err)
        }

        return true;
    }
}
