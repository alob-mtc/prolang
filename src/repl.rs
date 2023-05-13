use std::io;

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut input = String::new();
    loop {
        println!("{}", PROMPT);
        io::stdin().read_line(&mut input).unwrap();
        let mut l = Lexer::new(input.trim().to_string());

        while let tok = l.next_token() {
            if tok.token_type == TokenType::EOF {
                break;
            }
            println!("{:?}", tok)
        }
    }
}