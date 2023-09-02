use crate::core::lexer::token::TokenType;
use crate::core::parser::ast::ConditionalIteratorExpression;

use super::{
    ast::{
        BooleanLiteral, CallExpression, Expression, FunctionLiteral, Identifier, IfExpression,
        InfixExpression, IntegerLiteral, IteratorLiteral, PrefixExpression,
    },
    get_of_type,
    parser::{Parser, LOWEST, PREFIX},
};

pub(crate) fn parse_infix_func(
    p: &mut Parser,
    left: Box<dyn Expression>,
) -> Option<Box<dyn Expression>> {
    match p.cur_token.token_type {
        TokenType::PLUS
        | TokenType::MINUS
        | TokenType::SLASH
        | TokenType::ASTERISK
        | TokenType::EQ
        | TokenType::NotEq
        | TokenType::LT
        | TokenType::GT => Some(parse_infix_expression(p, left)),
        TokenType::LPAREN => Some(parse_call_epression(p, left)),
        TokenType::IN => parse_conditional_iter_expression(p, left),
        TokenType::Spreed => Some(parse_spreed_epression(p, left)),
        _ => None,
    }
}

fn parse_infix_expression(p: &mut Parser, left: Box<dyn Expression>) -> Box<dyn Expression> {
    let mut expression = Box::new(InfixExpression {
        token: p.cur_token.clone(),
        left,
        operator: p.cur_token.literal.clone(),
        right: None,
    });
    let precedence = p.cur_precedence();
    p.next_token();
    expression.right = p.parse_expression(precedence);
    expression
}

fn parse_call_epression(p: &mut Parser, function: Box<dyn Expression>) -> Box<dyn Expression> {
    let exp = CallExpression {
        token: p.cur_token.clone(),
        function,
        arguments: p.parse_call_argument(),
    };

    Box::new(exp)
}

pub fn parse_conditional_iter_expression(
    p: &mut Parser,
    variable: Box<dyn Expression>,
) -> Option<Box<dyn Expression>> {
    let ident = get_of_type::<Identifier>(variable.get_as_any())?;
    let mut expression = ConditionalIteratorExpression {
        token: p.cur_token.clone(),
        variable: Identifier {
            token: ident.token.clone(),
            value: ident.value.clone(),
        },
        r#in: None,
    };
    p.next_token();

    expression.r#in = p.parse_expression(LOWEST);

    if !p.expect_peek(TokenType::RPAREN) {
        return None;
    }

    Some(Box::new(expression))
}

fn parse_spreed_epression(p: &mut Parser, int: Box<dyn Expression>) -> Box<dyn Expression> {
    let mut expression = Box::new(IteratorLiteral {
        token: p.cur_token.clone(),
        start: int,
        end: None,
    });

    let precedence = p.cur_precedence();
    p.next_token();
    expression.end = p.parse_expression(precedence);

    expression
}

pub(crate) fn parse_prefix_func(p: &mut Parser) -> Option<Box<dyn Expression>> {
    match p.cur_token.token_type {
        TokenType::IDENT => parse_identifier(p),
        TokenType::INT => Some(parse_integer_literal(p)),
        TokenType::BANG => parse_prefix_expression(p),
        TokenType::MINUS => parse_prefix_expression(p),
        TokenType::TRUE | TokenType::FALSE => Some(parse_boolean(p)),
        TokenType::LPAREN => parse_grouped_expression(p),
        TokenType::IF => parse_if_expression(p),
        TokenType::FUNCTION => parse_fn_literal(p),
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

fn parse_identifier(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let expression = Identifier {
        token: p.cur_token.clone(),
        value: p.cur_token.literal.clone(),
    };

    Some(Box::new(expression))
}

fn parse_integer_literal(p: &Parser) -> Box<dyn Expression> {
    Box::new(IntegerLiteral {
        token: p.cur_token.clone(),
        value: p.cur_token.literal.parse().unwrap(),
    })
}

fn parse_boolean(p: &Parser) -> Box<dyn Expression> {
    Box::new(BooleanLiteral {
        token: p.cur_token.clone(),
        value: p.cur_token_is(TokenType::TRUE),
    })
}

fn parse_grouped_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    p.next_token();
    let exp = p.parse_expression(LOWEST);
    if !p.expect_peek(TokenType::RPAREN) {
        return None;
    }
    exp
}

fn parse_if_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut expression = IfExpression {
        token: p.cur_token.clone(),
        condition: None,
        consequence: None,
        alternative: None,
    };

    if !p.expect_peek(TokenType::LPAREN) {
        return None;
    }

    p.next_token();
    expression.condition = p.parse_expression(LOWEST);

    if !p.expect_peek(TokenType::RPAREN) {
        return None;
    }

    if !p.expect_peek(TokenType::LBRACE) {
        return None;
    }

    expression.consequence = p.parse_block_statement();

    if p.peek_token_is(&TokenType::ELSE) {
        p.next_token();
        if !p.expect_peek(TokenType::LBRACE) {
            return None;
        }
        expression.alternative = p.parse_block_statement();
    }

    Some(Box::new(expression))
}

fn parse_fn_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut lit = FunctionLiteral {
        token: p.cur_token.clone(),
        parameters: vec![],
        body: None,
    };
    if !p.expect_peek(TokenType::LPAREN) {
        return None;
    }

    if let Some(params) = p.parse_fn_parameters() {
        lit.parameters = params;
    }

    if !p.expect_peek(TokenType::LBRACE) {
        return None;
    }

    lit.body = p.parse_block_statement();

    Some(Box::new(lit))
}
