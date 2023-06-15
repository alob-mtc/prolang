use std::collections::HashMap;

use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

use super::ast::{
    BlockStatement, ConditionalIteratorExpression, Expression, ExpressionStatement,
    ForLoopCondition, ForLoopExpression, Identifier, InfixExpression, LetStatement, Program,
    ReturnStatemnt, Statement,
};
use super::is_of_type;
use super::parse_func::{parse_infix_func, parse_prefix_func};

pub(crate) const LOWEST: i32 = 1;
const EQUALS: i32 = 2; // ==
const LESSGREATER: i32 = 3; // > or <
const SPREED: i32 = 4;
const IN: i32 = 5;
const SUM: i32 = 6; // +
const PRODUCT: i32 = 7; // *
pub(crate) const PREFIX: i32 = 8; // -X or !X
const CALL: i32 = 9; // fn(x)

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
            TokenType::FOR => self.parse_for_expression(),
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

        self.next_token();
        stmt.value = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = ReturnStatemnt {
            token: self.cur_token.clone(),
            return_value: None,
        };

        self.next_token();

        stmt.return_value = self.parse_expression(LOWEST);

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
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

    pub(crate) fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let mut block = BlockStatement {
            token: self.cur_token.clone(),
            statements: vec![],
        };

        self.next_token();

        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token()
        }

        Some(block)
    }

    pub(crate) fn parse_fn_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = vec![];
        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.to_owned(),
        });

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.to_owned(),
            })
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }

        Some(identifiers)
    }

    pub(crate) fn parse_call_argument(&mut self) -> Vec<Box<dyn Expression>> {
        let mut args = vec![];

        if self.peek_token_is(&TokenType::RPAREN) {
            self.next_token();
            return args;
        }

        self.next_token();
        args.push(self.parse_expression(LOWEST).unwrap());

        while self.peek_token_is(&TokenType::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(LOWEST).unwrap());
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return vec![];
        }

        args
    }

    fn parse_for_expression(&mut self) -> Option<Box<dyn Statement>> {
        let mut expression = ForLoopExpression {
            token: self.cur_token.clone(),
            condition: None,
            body: None,
        };

        if !self.peek_token_is(&TokenType::LBRACE) && !self.expect_peek(TokenType::LPAREN) {
            return None;
        }

        if self.peek_token_is(&TokenType::LBRACE) {
            self.next_token();
            expression.condition = Some(ForLoopCondition::Loop);
        } else {
            self.next_token();
            if let Some(condition) = self.parse_expression(LOWEST) {
                if is_of_type::<InfixExpression>(condition.get_as_any()) {
                    expression.condition = Some(ForLoopCondition::For(condition));
                    self.next_token();
                } else if is_of_type::<ConditionalIteratorExpression>(condition.get_as_any()) {
                    expression.condition = Some(ForLoopCondition::ForIn(condition))
                }
                if !self.expect_peek(TokenType::LBRACE) {
                    return None;
                }
            }
        }

        expression.body = self.parse_block_statement();

        if self.peek_token_is(&TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(expression))
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
        self.precedences.insert(TokenType::LPAREN, CALL);
        self.precedences.insert(TokenType::Spreed, SPREED);
        self.precedences.insert(TokenType::IN, IN);
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

    pub(crate) fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }
    pub(crate) fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }
    pub(crate) fn expect_peek(&mut self, t: TokenType) -> bool {
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
            "main.pr:{}:{} \n expected next token to be {:?}, got {:?} instead",
            self.cur_token.position.0, self.cur_token.position.1, t, self.peek_token.token_type,
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
