use std::any::Any;

use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
    fn get_as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn get_let(&self) -> Option<&LetStatement> {
        None
    }
    fn get_expression_stmt(&self) -> Option<&ExpressionStatement> {
        None
    }
    fn get_for_exp(&self) -> Option<&ForLoopExpression> {
        None
    }
}

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
    pub token: Token, //LET token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
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
pub struct Identifier {
    pub token: Token, //IDENT token
    pub value: String,
}

impl Expression for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        self.value.to_owned()
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExpressionStatement {
    pub token: Token, //first token of the expression
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {
    fn get_expression_stmt(&self) -> Option<&ExpressionStatement> {
        Some(self)
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
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
    pub token: Token, //return
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatemnt {}

impl Node for ReturnStatemnt {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push(' ');
        if let Some(return_value) = &self.return_value {
            out.push_str(&return_value.string());
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
        &self.token.literal
    }

    fn string(&self) -> String {
        self.token.literal.to_owned()
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
        &self.token.literal
    }
    fn string(&self) -> String {
        self.token.literal.to_owned()
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for BooleanLiteral {}

pub struct PrefixExpression {
    pub token: Token, //prefix token
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.operator);
        out.push_str(&self.right.as_ref().unwrap().string());
        out.push(')');

        out
    }
    fn get_as_any(&self) -> &dyn Any {
        self
    }
}

pub struct InfixExpression {
    pub token: Token, //infix token: '-', '+'
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push('(');
        out.push_str(&self.left.string());
        out.push(' ');
        out.push_str(&self.operator);
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
    pub token: Token, //IF
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpression {}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal());
        out.push_str(&self.condition.as_ref().unwrap().string());
        out.push(' ');
        out.push_str(&self.consequence.as_ref().unwrap().string());
        if let Some(alt) = &self.alternative {
            out.push_str(" else ");
            out.push_str(&alt.string())
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
        &self.token.literal
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
    pub parameters: Vec<Identifier>,
    pub body: Option<BlockStatement>,
}

impl Expression for FunctionLiteral {}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
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
    pub token: Token, //IDENT
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn string(&self) -> String {
        let mut out = String::new();
        let mut args = vec![];
        for a in &self.arguments {
            args.push(a.string());
        }
        out.push_str(&self.function.string());
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
    pub token: Token, //FOR
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
        &self.token.literal
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
    pub token: Token, // IN
    pub variable: Identifier,
    pub r#in: Option<Box<dyn Expression>>,
}

impl Expression for ConditionalIteratorExpression {}

impl Node for ConditionalIteratorExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
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
        &self.token.literal
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
    use crate::lexer::token::{Token, TokenType};

    use super::{Identifier, LetStatement, Node, Program};

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: "let".to_string(),
                    position: (0, 0),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                        position: (0, 0),
                    },
                    value: "myVar".to_string(),
                },
                value: Some(Box::new(Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                        position: (0, 0),
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
