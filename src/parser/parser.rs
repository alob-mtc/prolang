use std::collections::HashMap;

use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

use super::ast::{Expression, ExpressionStatement, Identifier, LetStatement, Program, Statement};
use super::parse_func::{parse_infix_func, parse_prefix_func};

const LOWEST: i32 = 1;
const EQUALS: i32 = 2; // ==
const LESSGREATER: i32 = 3; // > or <
const SUM: i32 = 4; // +
const PRODUCT: i32 = 5; // *
pub(crate) const PREFIX: i32 = 6; // -X or !X
const CALL: i32 = 7; // fn(x)

pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    pub(crate) cur_token: Token,
    peek_token: Token,
    precedences: HashMap<TokenType, i32>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            errors: vec![],
            cur_token: Token::default(),
            peek_token: Token::default(),
            precedences: HashMap::default(),
        };
        p.register_precedences();
        // clear default tokens
        p.next_token();
        p.next_token();

        p
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
            TokenType::None => None,
            _ => self.parse_expression_statment(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            value: None,
            name: Identifier::default(),
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: String::new(),
        };
        stmt.name.value.push_str(&self.cur_token.literal);

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
            value: None,
            name: Identifier::default(),
        };

        self.next_token();

        // TODO: skipping the expressions until we encounter ;
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statment(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression: self.parse_expression(LOWEST),
        };

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token()
        }

        Some(Box::new(stmt))
    }

    pub(crate) fn parse_expression(&mut self, precedence: i32) -> Option<Box<dyn Expression>> {
        match parse_prefix_func(self) {
            Some(mut left_exp) => {
                while !self.peek_token_is(&TokenType::SEMICOLON)
                    && precedence < self.peek_precedence()
                {
                    // TODO: think about this implementation
                    self.next_token();
                    left_exp = parse_infix_func(self, left_exp)?;
                }
                Some(left_exp)
            }
            _ => {
                self.no_prefix_parse_fn_error();
                return None;
            }
        }
    }
}

// util functions
impl Parser {
    fn register_precedences(&mut self) {
        self.precedences.insert(TokenType::EQ, EQUALS);
        self.precedences.insert(TokenType::NotEq, EQUALS);
        self.precedences.insert(TokenType::LT, LESSGREATER);
        self.precedences.insert(TokenType::GT, LESSGREATER);
        self.precedences.insert(TokenType::PLUS, SUM);
        self.precedences.insert(TokenType::MINUS, SUM);
        self.precedences.insert(TokenType::SLASH, PRODUCT);
        self.precedences.insert(TokenType::ASTERISK, PRODUCT);
    }

    fn peek_precedence(&self) -> i32 {
        if let Some(&p) = self.precedences.get(&self.peek_token.token_type) {
            return p;
        }
        LOWEST
    }

    pub(crate) fn cur_precedence(&self) -> i32 {
        if let Some(&p) = self.precedences.get(&self.cur_token.token_type) {
            return p;
        }
        LOWEST
    }

    pub(crate) fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = self.l.next_token();
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
        false
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type,
        );
        self.errors.push(msg)
    }
    fn no_prefix_parse_fn_error(&mut self) {
        let msg = format!(
            "no prefix parse function for {:?} found",
            self.cur_token.literal
        );
        self.errors.push(msg);
    }
}
