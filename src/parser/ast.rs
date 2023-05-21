use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn get_let(&self) -> Option<&LetStatement> {
        None
    }
}

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
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.string());
        }
        out
    }
}

pub struct LetStatement {
    pub token: Token, //LET token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }
        out.push_str(";");

        out
    }
}

impl Statement for LetStatement {
    fn get_let(&self) -> Option<&LetStatement> {
        Some(self)
    }
}

#[derive(Default)]
pub struct Identifier {
    pub token: Token, //IDENT token
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn string(&self) -> String {
        self.value.to_owned()
    }
}

impl Expression for Identifier {}

pub struct ExpressionStatement {
    token: Token, //first token of the expression
    expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn string(&self) -> String {
        if let Some(value) = &self.expression {
            return value.string();
        }

        "".to_owned()
    }
}

impl Statement for ExpressionStatement {}

pub struct ReturnStatemnt {
    token: Token,
    return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatemnt {}

impl Node for ReturnStatemnt {
    fn token_literal(&self) -> String {
        self.token.literal.to_owned()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        if let Some(return_value) = &self.return_value {
            out.push_str(&return_value.string());
        }
        out.push_str(";");

        out
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::token::{Token, TokenType};

    use super::{Identifier, LetStatement, Node, Program};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: "let".to_string(),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };
        assert_eq!(
            program.string(),
            "let myVar = anotherVar;",
            "program.string() wrong, got={}",
            program.string()
        )
    }
}
