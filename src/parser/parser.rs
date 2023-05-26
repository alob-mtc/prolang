use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};


use super::ast::{
    Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, PrefixExpression,
    Program, Statement,
};

const LOWEST: i32 = 1;
const EQUALS: i32 = 2; // ==
const LESSGREATER: i32 = 3; // > or <
const SUM: i32 = 4; // +
const PRODUCT: i32 = 5; // *
const PREFIX: i32 = 6; // -X or !X
const CALL: i32 = 7; // fn(x)

pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            errors: vec![],
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        // clear default tokens
        p.next_token();
        p.next_token();

        p
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

    fn parse_expression(&mut self, _precedence: i32) -> Option<Box<dyn Expression>> {
        let left_exp = parse_func(self);

        if let Some(left_exp) = left_exp {
            return Some(left_exp);
        }
        self.no_prefix_parse_fn_error();

        None
    }
}

// util functions
impl Parser {
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
            "no prefix parse function for {} found",
            self.cur_token.literal
        );
        self.errors.push(msg);
    }
}

// parse functions
fn parse_func(p: &mut Parser) -> Option<Box<dyn Expression>> {
    match p.cur_token.token_type {
        TokenType::IDENT => Some(parse_identifier(p)),
        TokenType::INT => Some(parse_integer_literal(p)),
        TokenType::BANG => parse_prefix_expression(p),
        TokenType::MINUS => parse_prefix_expression(p),
        _ => None,
    }
}

fn parse_prefix_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut expression = Box::new(PrefixExpression {
        token: p.cur_token.clone(),
        operator: p.cur_token.literal.clone(),
        right: None,
    });
    p.next_token();
    if let Some(right_exp) = p.parse_expression(PREFIX) {
        expression.right = Some(right_exp);
        return Some(expression);
    }

    None
}

fn parse_identifier(p: &Parser) -> Box<dyn Expression> {
    Box::new(Identifier {
        token: p.cur_token.clone(),
        value: p.cur_token.literal.clone(),
    })
}

fn parse_integer_literal(p: &Parser) -> Box<dyn Expression> {
    Box::new(IntegerLiteral {
        token: p.cur_token.clone(),
        value: p.cur_token.literal.parse().unwrap(),
    })
}
