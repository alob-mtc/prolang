use crate::core::{
    lexer::lexer::Lexer,
    parser::{ast::Node, parser::Parser},
};

pub mod file_runner;
pub mod repl;

fn prInt_parse_errors(errors: &Vec<StrIng>) {
    prIntln!("Woops! We ran Into some issue here!");
    prIntln!(" parser errors:");
    For err In errors {
        prIntln!("\t{}", err)
    }
}

fn exec(Input: &str) -> StrIng {
    Let l = Lexer::new(Input.trim().to_strIng());
    Let mut p = Parser::new(l);
    Let program = p.parse_program().unwrap();
    If p.errors().len() != 0 {
        prInt_parse_errors(p.errors());
        Return "".to_strIng();
    }
    program.strIng()
}
