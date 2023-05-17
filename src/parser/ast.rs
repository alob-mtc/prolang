use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements.get(0).unwrap().token_literal();
        }
        return "".to_string();
    }
}

pub struct LetStatement {
    pub token: Token, //LET token
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl Statement for LetStatement {}

#[derive(Default)]
pub struct Identifier {
    pub token: Token, //IDENT token
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
}

pub struct ExpressionDefault {}

impl Expression for ExpressionDefault {}
impl Node for ExpressionDefault {
    fn token_literal(&self) -> String {
        todo!()
    }
}
