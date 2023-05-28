use crate::lexer::token::TokenType;

use super::{
    ast::{Expression, Identifier, InfixExpression, IntegerLiteral, PrefixExpression},
    parser::{Parser, PREFIX},
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

pub(crate) fn parse_prefix_func(p: &mut Parser) -> Option<Box<dyn Expression>> {
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
