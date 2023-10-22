use std::any::Any;

use crate::core::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
    fn get_as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub trait Iterators: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            return self.statements.get(0).unwrap().token_literal();
        }
        ""
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(&s.string());
        }
        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct LetStatement {
    pub token: Token, //let token
    pub name: IdentIfier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        out.push_str(&self.name.string());
        out.push_str(" = ");
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }
        out.push(';');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
pub struct IdentIfier {
    pub token: Token, //Ident token
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExpressionStatement {
    pub token: Token, //first token of the expression
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        if let Some(value) = &self.expression {
            return value.string();
        }

        "".to_owned()
    }

    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ReturnStatemnt {
    pub token: Token, //Return
    pub Return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatemnt {}

impl Node for ReturnStatemnt {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        if let Some(Return_value) = &self.Return_value {
            out.push_str(&Return_value.string());
        }
        out.push(';');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        todo!()
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        todo!()
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for BooleanLiteral {}

pub struct PrefixExpression {
    pub token: Token, //prefix token
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&format!("{}", self.token));
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push(')');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InfixExpression {
    pub token: Token, //Infix token: '-', '+'
    pub left: Box<dyn Expression>,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.left.string());
        out.push(' ');
        out.push_str(&format!("{}", self.token));
        out.push(' ');
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push(')');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IfExpression {
    pub token: Token, //If
    pub condition: Option<Box<dyn Expression>>,
    pub consEquence: Option<BlockStatement>,
    pub aLternative: Option<BlockStatement>,
}

impl Expression for IfExpression {}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal());
        out.push_str(&self.condition.as_ref().unwrap().string());
        out.push(' ');
        out.push_str(&self.consEquence.as_ref().unwrap().string());
        if let Some(aLt) = &self.aLternative {
            out.push_str(" else ");
            out.push_str(&aLt.string())
        }

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BlockStatement {
    pub token: Token, //{
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for s in &self.statements {
            out.push_str(&s.string());
        }

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct FunctionLiteral {
    pub token: Token, //FN
    pub parameters: Vec<IdentIfier>,
    pub body: Option<BlockStatement>,
}

impl Expression for FunctionLiteral {}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = vec![];

        for p in &self.parameters {
            params.push(p.string());
        }
        out.push_str(self.token_literal());
        out.push('(');
        out.push_str(params.join(", ").as_str());
        out.push_str(") ");
        if let Some(body) = &self.body {
            out.push_str(&body.string());
        }

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    pub token: Token, //Ident
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut args = vec![];
        for a in &self.arguments {
            args.push(a.string());
        }
        out.push_str(&self.Function.string());
        out.push('(');
        out.push_str(&args.join(", "));
        out.push(')');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ForLoopExpression {
    pub token: Token, //For
    pub condition: Option<ForLoopCondition>,
    pub body: Option<BlockStatement>,
}

pub enum ForLoopCondition {
    Loop,
    ForIn(Box<dyn Expression>),
    For(Box<dyn Expression>),
}

impl ForLoopCondition {
    fn string(&self) -> String {
        match self {
            ForLoopCondition::Loop => "".to_owned(),
            ForLoopCondition::ForIn(exp) | ForLoopCondition::For(exp) => exp.string(),
        }
    }
}

impl Statement for ForLoopExpression {}

impl Node for ForLoopExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out: String = String::new();
        out.push_str(self.token_literal());
        out.push(' ');
        if let Some(cond) = &self.condition {
            out.push_str(&cond.string())
        }
        if let Some(body) = &self.body {
            out.push_str(&body.string());
        }

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ConditionalIteratorExpression {
    pub token: Token, // In
    pub variable: IdentIfier,
    pub r#in: Option<Box<dyn Expression>>,
}

impl Expression for ConditionalIteratorExpression {}

impl Node for ConditionalIteratorExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.variable.string());
        out.push(' ');
        out.push_str(self.token_literal());
        out.push(' ');
        if let Some(r#in) = &self.r#in {
            out.push_str(&r#in.string())
        }
        out.push(')');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IteratorLiteral {
    pub token: Token, // .. SPREED
    pub start: Box<dyn Expression>,
    pub end: Option<Box<dyn Expression>>,
}

impl Iterators for IteratorLiteral {}

impl Expression for IteratorLiteral {}

impl Node for IteratorLiteral {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.start.string());
        out.push_str(self.token_literal());
        out.push_str(&self.end.as_ref().unwrap().string());

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::core::lexer::token::{Position, Token, TokenType};

    use super::{IdentIfier, LetStatement, Node, Program};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_type: TokenType::Let,
                    position: Position(0, 0),
                },
                name: IdentIfier {
                    token: Token {
                        token_type: TokenType::Ident("myVar".to_string()),
                        position: Position(0, 0),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Box::new(IdentIfier {
                    token: Token {
                        token_type: TokenType::Ident("anotherVar".to_string()),
                        position: Position(0, 0),
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
