use crate::lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn string(&self) -> String;
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

pub trait Expression: Node {
    fn get_ident(&self) -> Option<&Identifier> {
        None
    }
    fn get_int_literal(&self) -> Option<&IntegerLiteral> {
        None
    }
    fn get_prefix_exp(&self) -> Option<&PrefixExpression> {
        None
    }
    fn get_infix_exp(&self) -> Option<&InfixExpression> {
        None
    }
    fn get_bool_literal(&self) -> Option<&BooleanLiteral> {
        None
    }
    fn get_if_exp(&self) -> Option<&IfExpression> {
        None
    }
    fn get_fn_literal(&self) -> Option<&FunctionLiteral> {
        None
    }
    fn get_call_exp(&self) -> Option<&CallExpression> {
        None
    }
    fn get_conditional_iter(&self) -> Option<&ConditionalIteratorExpression> {
        None
    }
    fn get_iter_literal(&self) -> Option<&IteratorLiteral> {
        None
    }
}

pub trait Iterators: Node {
    fn get_iter_literal(&self) -> Option<&IteratorLiteral> {
        None
    }
}

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
}

pub struct LetStatement {
    pub token: Token, //LET token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

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
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn string(&self) -> String {
        self.value.to_owned()
    }
}

impl Expression for Identifier {
    fn get_ident(&self) -> Option<&Identifier> {
        Some(self)
    }
}

pub struct ExpressionStatement {
    pub token: Token, //first token of the expression
    pub expression: Option<Box<dyn Expression>>,
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
}

impl Statement for ExpressionStatement {
    fn get_expression_stmt(&self) -> Option<&ExpressionStatement> {
        Some(self)
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
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn string(&self) -> String {
        self.token.literal.to_owned()
    }
}

impl Expression for IntegerLiteral {
    fn get_int_literal(&self) -> Option<&IntegerLiteral> {
        Some(self)
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
}

impl Expression for BooleanLiteral {
    fn get_bool_literal(&self) -> Option<&BooleanLiteral> {
        Some(self)
    }
}

pub struct PrefixExpression {
    pub token: Token, //prefix token
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {
    fn get_prefix_exp(&self) -> Option<&PrefixExpression> {
        Some(self)
    }
}

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
}

pub struct InfixExpression {
    pub token: Token, //infix token: '-', '+'
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {
    fn get_infix_exp(&self) -> Option<&InfixExpression> {
        Some(self)
    }
}

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
}

pub struct IfExpression {
    pub token: Token, //IF
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

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
}

impl Expression for IfExpression {
    fn get_if_exp(&self) -> Option<&IfExpression> {
        Some(self)
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
}

pub struct FunctionLiteral {
    pub token: Token, //FN
    pub parameters: Vec<Identifier>,
    pub body: Option<BlockStatement>,
}

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
}

impl Expression for FunctionLiteral {
    fn get_fn_literal(&self) -> Option<&FunctionLiteral> {
        Some(self)
    }
}

pub struct CallExpression {
    pub token: Token, //IDENT
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

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
}

impl Expression for CallExpression {
    fn get_call_exp(&self) -> Option<&CallExpression> {
        Some(self)
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

impl Node for ForLoopExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn string(&self) -> String {
        let out = String::new();

        out
    }
}

impl Statement for ForLoopExpression {
    fn get_for_exp(&self) -> Option<&ForLoopExpression> {
        Some(self)
    }
}

pub struct ConditionalIteratorExpression {
    pub token: Token, // IN
    pub variable: Identifier,
    pub r#in: Option<Box<dyn Expression>>,
}

impl Node for ConditionalIteratorExpression {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        todo!()
    }
}

impl Expression for ConditionalIteratorExpression {
    fn get_conditional_iter(&self) -> Option<&ConditionalIteratorExpression> {
        Some(self)
    }
}

pub struct IteratorLiteral {
    pub token: Token,
    pub start: Box<dyn Expression>,
    pub end: Option<Box<dyn Expression>>,
}

impl Node for IteratorLiteral {
    fn token_literal(&self) -> &str {
        todo!()
    }

    fn string(&self) -> String {
        todo!()
    }
}

impl Iterators for IteratorLiteral {}

impl Expression for IteratorLiteral {}

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
