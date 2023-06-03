use std::any::Any;

use crate::{
    lexer::lexer::Lexer,
    parser::ast::{Expression, Node, Statement},
    parser::parser::Parser,
};

#[test]
fn test_let_statement() {
    let input = String::from(
        "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ",
    );

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program().expect("parse_program() return some");
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
            expected_identifier: String::from("x"),
        },
        TestCase {
            expected_identifier: String::from("y"),
        },
        TestCase {
            expected_identifier: String::from("foobar"),
        },
    ];

    let mut i = 0;
    for tt in tests {
        let stmt = program.statements.get(i).unwrap();
        let_statemnt(stmt, &tt.expected_identifier);
        i += 1;
    }
}

#[test]
fn test_return_statement() {
    let input = String::from(
        "
        return 5;
        return 10;
        return 993322;
        ",
    );

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program().expect("parse_program() not return none");
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
    let input = String::from("foobar;");

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().expect("parse_program() return some");

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
        .expect("program.Statements[0] is ast.ExpressionStatement");

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
    let input = String::from("5;");

    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program().expect("parse_program() return some");

    assert_eq!(chack_parser_errors(&p), false);
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let stmt = program
        .statements
        .get(0)
        .expect("expected statemnt[0] to have a value")
        .get_expression_stmt()
        .expect("program.Statements[0] is ast.ExpressionStatement");

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

#[test]
fn test_parsing_prefix_expression() {
    struct TestCase<'a> {
        input: String,
        operator: String,
        integer_value: &'a dyn Any,
    }

    let tests = vec![
        TestCase {
            input: String::from("!5"),
            operator: String::from("!"),
            integer_value: &5,
        },
        TestCase {
            input: String::from("-15"),
            operator: String::from("-"),
            integer_value: &15,
        },
        TestCase {
            input: String::from("!true"),
            operator: String::from("!"),
            integer_value: &true,
        },
        TestCase {
            input: String::from("!false"),
            operator: String::from("!"),
            integer_value: &false,
        },
    ];

    for tt in tests {
        let l = Lexer::new(tt.input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return some");

        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements. got={}",
            program.statements.len()
        );

        let stmt = program
            .statements
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_expression_stmt()
            .expect("program.Statements[0] is ast.ExpressionStatement");

        let exp = match &stmt.expression {
            Some(exp) => match exp.get_prefix_exp() {
                Some(exp) => exp,
                _ => panic!("exp is not Identifier"),
            },
            _ => panic!("exp is none"),
        };

        assert_eq!(
            exp.operator, tt.operator,
            "ident.value not {}. got={}",
            tt.operator, exp.operator
        );

        test_literal_expression(exp.right.as_ref().unwrap(), tt.integer_value);
    }
}

#[test]
fn test_parsing_infix_expression() {
    struct TestCase<'a> {
        input: String,
        left_value: &'a dyn Any,
        operator: String,
        right_value: &'a dyn Any,
    }

    let tests = vec![
        TestCase {
            input: String::from("5 + 5;"),
            left_value: &5,
            operator: String::from("+"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 - 5;"),
            left_value: &5,
            operator: String::from("-"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 * 5;"),
            left_value: &5,
            operator: String::from("*"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 / 5;"),
            left_value: &5,
            operator: String::from("/"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 > 5;"),
            left_value: &5,
            operator: String::from(">"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 < 5;"),
            left_value: &5,
            operator: String::from("<"),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 == 5;"),
            left_value: &5,
            operator: String::from("=="),
            right_value: &5,
        },
        TestCase {
            input: String::from("5 != 5;"),
            left_value: &5,
            operator: String::from("!="),
            right_value: &5,
        },
        TestCase {
            input: String::from("true == true;"),
            left_value: &true,
            operator: String::from("=="),
            right_value: &true,
        },
        TestCase {
            input: String::from("true != false;"),
            left_value: &true,
            operator: String::from("!="),
            right_value: &false,
        },
        TestCase {
            input: String::from("false == false;"),
            left_value: &false,
            operator: String::from("=="),
            right_value: &false,
        },
    ];

    for tt in tests {
        let l = Lexer::new(tt.input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return some");

        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements. got={}",
            program.statements.len()
        );

        let stmt = program
            .statements
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_expression_stmt()
            .expect("program.Statements[0] is ast.ExpressionStatement");

        let exp = match &stmt.expression {
            Some(exp) => test_infix_expression(exp, &tt.left_value, &tt.operator, &tt.right_value),
            _ => panic!("exp is none"),
        };
    }
}

#[test]
fn test_operator_precedence_parsing() {
    struct TestCase {
        input: String,
        expected: String,
    }

    let tests = vec![
        TestCase {
            input: String::from("true"),
            expected: String::from("true"),
        },
        TestCase {
            input: String::from("false"),
            expected: String::from("false"),
        },
        TestCase {
            input: String::from("3 > 5 == false"),
            expected: String::from("((3 > 5) == false)"),
        },
        TestCase {
            input: String::from("3 < 5 == true"),
            expected: String::from("((3 < 5) == true)"),
        },
        TestCase {
            input: String::from("-a * b"),
            expected: String::from("((-a) * b)"),
        },
        TestCase {
            input: String::from("!-a"),
            expected: String::from("(!(-a))"),
        },
        TestCase {
            input: String::from("a + b + c"),
            expected: String::from("((a + b) + c)"),
        },
        TestCase {
            input: String::from("a + b - c"),
            expected: String::from("((a + b) - c)"),
        },
        TestCase {
            input: String::from("a * b * c"),
            expected: String::from("((a * b) * c)"),
        },
        TestCase {
            input: String::from("a * b / c"),
            expected: String::from("((a * b) / c)"),
        },
        TestCase {
            input: String::from("a + b / c"),
            expected: String::from("(a + (b / c))"),
        },
        TestCase {
            input: String::from("a + b * c + d / e - f"),
            expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
        },
        TestCase {
            input: String::from("3 + 4; -5 * 5"),
            expected: String::from("(3 + 4)((-5) * 5)"),
        },
        TestCase {
            input: String::from("5 > 4 == 3 < 4"),
            expected: String::from("((5 > 4) == (3 < 4))"),
        },
        TestCase {
            input: String::from("5 < 4 != 3 > 4"),
            expected: String::from("((5 < 4) != (3 > 4))"),
        },
        TestCase {
            input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
            expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        },
        TestCase {
            input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
            expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        },
        TestCase {
            input: String::from("1 + (2 + 3) + 4"),
            expected: String::from("((1 + (2 + 3)) + 4)"),
        },
        TestCase {
            input: String::from("(5 + 5) * 2"),
            expected: String::from("((5 + 5) * 2)"),
        },
        TestCase {
            input: String::from("2 / (5 + 5)"),
            expected: String::from("(2 / (5 + 5))"),
        },
        TestCase {
            input: String::from("-(5 + 5)"),
            expected: String::from("(-(5 + 5))"),
        },
        TestCase {
            input: String::from("!(true == true)"),
            expected: String::from("(!(true == true))"),
        },
    ];

    for tt in tests {
        let l = Lexer::new(tt.input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return none");

        assert_eq!(chack_parser_errors(&p), false);
        let actual = program.string();
        assert_eq!(
            actual, tt.expected,
            "expected={}, got={}",
            tt.expected, actual
        )
    }
}

//utils
fn test_infix_expression(
    exp: &Box<dyn Expression>,
    left: &dyn Any,
    operator: &str,
    right: &dyn Any,
) {
    let op_exp = exp.get_infix_exp().expect("exp is OperatorExpression");
    test_literal_expression(&op_exp.left, left);
    assert_eq!(
        op_exp.operator, operator,
        "exp.Operator is not {:?}. got={:?}",
        operator, op_exp.operator
    );

    test_literal_expression(op_exp.right.as_ref().unwrap(), right);
}

fn test_int_literal(il: &Box<dyn Expression>, value: i64) {
    let integ = il.get_int_literal().expect("il is not IntergerLiteral");
    assert_eq!(
        integ.value, value,
        "integ.value not {}. got={}",
        value, integ.value
    );

    assert_eq!(
        integ.token_literal(),
        value.to_string(),
        "integ.token_literal not {}. got={}",
        value,
        integ.token_literal()
    )
}

fn test_identifier(il: &Box<dyn Expression>, value: &str) {
    let ident = il.get_ident().expect("il is Identifier");
    assert_eq!(
        ident.value, value,
        "ident.value not {}. got={}",
        value, ident.value
    );

    assert_eq!(
        ident.token_literal(),
        value.to_string(),
        "ident.token_literal not {}. got={}",
        value,
        ident.token_literal()
    )
}

fn test_boolean_literal(il: &Box<dyn Expression>, value: bool) {
    let bo = il.get_bool_exp().expect("il is Boolean");
    assert_eq!(bo.value, value, "bo.value not {}. got={}", bo.value, value);
    assert_eq!(
        bo.token_literal(),
        value.to_string(),
        "bo.value not {}. got={}",
        bo.value,
        value
    );
}

fn test_literal_expression(exp: &Box<dyn Expression>, expected: &dyn Any) {
    if let Some(value) = expected.downcast_ref::<String>() {
        test_identifier(exp, value);
    } else if let Some(&value) = expected.downcast_ref::<i64>() {
        test_int_literal(exp, value);
    } else if let Some(&value) = expected.downcast_ref::<bool>() {
        test_boolean_literal(exp, value);
    } else {
        dbg!("type of exp not handled.");
    }
}

fn let_statemnt(s: &Box<dyn Statement>, name: &str) {
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

    true
}
