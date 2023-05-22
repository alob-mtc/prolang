use std::collections::HashMap;
use std::fmt::format;

use crate::lexer::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

use super::ast::{
    Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program, Statement,
};

const LOWEST: i32 = 1;
const EQUALS: i32 = 2; // ==
const LESSGREATER: i32 = 3; // > or <
const SUM: i32 = 4; // +
const PRODUCT: i32 = 5; // *
const PREFIX: i32 = 6; // -X or !X
const CALL: i32 = 7; // fn(x)

type PrefixParsefn = fn(&Parser) -> Box<dyn Expression>;
type InfixParsefn = fn(&Parser, dyn Expression) -> Box<dyn Expression>;

struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,

    prefix_parsefns: HashMap<TokenType, PrefixParsefn>,
    infix_parsefns: HashMap<TokenType, InfixParsefn>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            errors: vec![],
            cur_token: Token::default(),
            peek_token: Token::default(),
            prefix_parsefns: HashMap::new(),
            infix_parsefns: HashMap::new(),
        };

        // clear default tokens
        p.next_token();
        p.next_token();

        p.prefix_parsefns.insert(TokenType::IDENT, parse_identifier);
        p.prefix_parsefns
            .insert(TokenType::INT, parse_integer_literal);

        return p;
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParsefn) {
        self.prefix_parsefns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: TokenType, func: InfixParsefn) {
        self.infix_parsefns.insert(token_type, func);
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

    fn parse_expression(&self, precedence: i32) -> Option<Box<dyn Expression>> {
        let prefix = self.prefix_parsefns.get(&self.cur_token.token_type);
        if prefix.is_none() {
            return None;
        }

        let left_exp = prefix.unwrap()(self);

        Some(left_exp)
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
        "
        .to_string();

        let l = Lexer::new(input);
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
        "
        .to_string();

        let l = Lexer::new(input);
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return none");

        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let stmt = program
            .statements
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_expression_stmt()
            .expect("program.Statements[0] is not ast.ExpressionStatement");

        let ident = match &stmt.expression {
            Some(ident) => match ident.get_ident() {
                Some(ident) => ident,
                _ => panic!("exp is not Identifier"),
            },
            _ => panic!("exp is none"),
        };

        assert_eq!(
            ident.value, "foobar",
            "ident.value not {}. got={}",
            "foobar", ident.value
        );

        assert_eq!(
            ident.token_literal(),
            "foobar",
            "ident.token_literal not {}. got={}",
            "foobar",
            ident.token_literal()
        )
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return none");

        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let stmt = program
            .statements
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_expression_stmt()
            .expect("program.Statements[0] is not ast.ExpressionStatement");

        let ident = match &stmt.expression {
            Some(ident) => match ident.get_int_literal() {
                Some(ident) => ident,
                _ => panic!("exp is not Identifier"),
            },
            _ => panic!("exp is none"),
        };

        assert_eq!(
            ident.value, 5,
            "ident.value not {}. got={}",
            "foobar", ident.value
        );

        assert_eq!(
            ident.token_literal(),
            "5",
            "ident.token_literal not {}. got={}",
            "foobar",
            ident.token_literal()
        )
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
