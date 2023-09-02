use std::any::Any;

use crate::core::{
    lexer::lexer::Lexer,
    parser::{
        ast::{
            BooleanLiteral, CallExpression, ConditionalIteratorExpression, Expression,
            ExpressionStatement, ForLoopCondition, ForLoopExpression, FunctionLiteral, Identifier,
            IfExpression, InfixExpression, IntegerLiteral, IteratorLiteral, LetStatement, Node,
            PrefixExpression, Statement,
        },
        get_of_type,
        parser::Parser,
    },
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

    let tests = [
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
fn test_let_statements() {
    struct TestCase<'a> {
        input: String,
        expected_identifier: String,
        expected_value: &'a dyn Any,
    }

    let tests = [
        TestCase {
            input: String::from("let x = 5;"),
            expected_identifier: String::from("x"),
            expected_value: &"y",
        },
        TestCase {
            input: String::from("let y = true;"),
            expected_identifier: String::from("y"),
            expected_value: &"y",
        },
    ];

    for tt in tests {
        let l = Lexer::new(tt.input);
        let mut p = Parser::new(l);

        let program = p.parse_program().expect("parse_program() not return none");
        assert_eq!(chack_parser_errors(&p), false);
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements. got={}",
            program.statements.len()
        );

        let stmt = program.statements.get(0).unwrap();
        let_statemnt(stmt, &tt.expected_identifier);
        let let_exp = get_of_type::<LetStatement>(stmt.get_as_any()).unwrap();

        test_literal_expression(
            &Box::new(let_exp.value.as_ref().unwrap().as_ref()),
            tt.expected_value,
        );
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

    let stmt = get_of_type::<ExpressionStatement>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ExpressionStatement");

    let ident = match &stmt.expression {
        Some(ident) => match get_of_type::<Identifier>(ident.get_as_any()) {
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

    let stmt = get_of_type::<ExpressionStatement>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ExpressionStatement");

    let ident = match &stmt.expression {
        Some(ident) => match get_of_type::<IntegerLiteral>(ident.get_as_any()) {
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
fn test_if_expression() {
    let input = String::from("if (x < y) { x } else {}");

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

    let stmt = get_of_type::<ExpressionStatement>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ExpressionStatement");

    let ifexp = match &stmt.expression {
        Some(ifexp) => match get_of_type::<IfExpression>(ifexp.get_as_any()) {
            Some(ifexp) => ifexp,
            _ => panic!("exp is not If expression"),
        },
        _ => panic!("exp is none"),
    };

    test_infix_expression(ifexp.condition.as_ref().unwrap(), &"x", "<", &"y");
    assert_eq!(
        ifexp.consequence.as_ref().unwrap().statements.len(),
        1,
        "consequence is not 1 statements. got={}",
        ifexp.consequence.as_ref().unwrap().statements.len()
    );

    let consequence = get_of_type::<ExpressionStatement>(
        ifexp
            .consequence
            .as_ref()
            .unwrap()
            .statements
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("statements[0] is ast.ExpressionStatement");

    test_identifier(
        &Box::new(consequence.expression.as_ref().unwrap().as_ref()),
        "x",
    );
    assert_eq!(
        ifexp.alternative.is_some(),
        true,
        "alternative was not some.",
    )
}

#[test]
fn test_for_expression_type_loop_parsing() {
    let input = String::from("for { x + y; }");

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

    let stmt = get_of_type::<ForLoopExpression>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ForLoopExpression");

    match &stmt.condition {
        Some(condition) => match condition {
            ForLoopCondition::Loop => (),
            _ => panic!(),
        },
        None => panic!(),
    }

    match get_of_type::<ExpressionStatement>(
        stmt.body
            .as_ref()
            .unwrap()
            .statements
            .get(0)
            .unwrap()
            .get_as_any(),
    ) {
        Some(body_stmt) => match &body_stmt.expression {
            Some(exp) => test_infix_expression(exp, &"x", "+", &"y"),
            _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
        },
        _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
    }
}

#[test]
fn test_for_expression_type_for_parsing() {
    let input = String::from("for (x < y) { x + y; }");

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

    let stmt = get_of_type::<ForLoopExpression>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ForLoopExpression");

    match &stmt.condition {
        Some(condition) => match condition {
            ForLoopCondition::For(condition) => test_infix_expression(condition, &"x", "<", &"y"),
            _ => panic!(),
        },
        None => panic!(),
    }

    match get_of_type::<ExpressionStatement>(
        stmt.body
            .as_ref()
            .unwrap()
            .statements
            .get(0)
            .unwrap()
            .get_as_any(),
    ) {
        Some(body_stmt) => match &body_stmt.expression {
            Some(exp) => test_infix_expression(exp, &"x", "+", &"y"),
            _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
        },
        _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
    }
}

#[test]
fn test_for_expression_type_forin_parsing() {
    let input = String::from("for (i in 0..10) { x + y; }");

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

    let stmt = get_of_type::<ForLoopExpression>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ForLoopExpression");

    match &stmt.condition {
        Some(condition) => match condition {
            ForLoopCondition::ForIn(condition) => {
                test_conditional_iter_expression(&Box::new(condition.as_ref()));
            }
            _ => panic!(),
        },
        None => panic!(),
    }

    match get_of_type::<ExpressionStatement>(
        stmt.body
            .as_ref()
            .unwrap()
            .statements
            .get(0)
            .unwrap()
            .get_as_any(),
    ) {
        Some(body_stmt) => match &body_stmt.expression {
            Some(exp) => test_infix_expression(exp, &"x", "+", &"y"),
            _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
        },
        _ => panic!("for-loop body stmt is not ast.ExpressionStatement"),
    }
}

fn test_conditional_iter_expression(condition: &Box<&dyn Expression>) {
    match get_of_type::<ConditionalIteratorExpression>(condition.get_as_any()) {
        Some(condition) => {
            test_literal_expression(&Box::new(&condition.variable), &"i");
            match &condition.r#in {
                Some(r#in) => match get_of_type::<IteratorLiteral>(r#in.get_as_any()) {
                    Some(iter) => {
                        test_literal_expression(&Box::new(iter.start.as_ref()), &"0");
                        test_literal_expression(
                            &Box::new(iter.end.as_ref().unwrap().as_ref()),
                            &"10",
                        )
                    }
                    None => panic!("conditional not iteraltor literal"),
                },
                None => panic!("condition iter does not have in expression"),
            }
        }
        None => todo!(),
    }
}

#[test]
fn test_function_literal_parsing() {
    let input = String::from("fn(x, y) { x + y; }");

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

    let stmt = get_of_type::<ExpressionStatement>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ExpressionStatement");

    let fn_literal = match &stmt.expression {
        Some(fn_literal) => match get_of_type::<FunctionLiteral>(fn_literal.get_as_any()) {
            Some(fn_literal) => fn_literal,
            _ => panic!("exp is not If expression"),
        },
        _ => panic!("exp is none"),
    };

    assert_eq!(
        fn_literal.parameters.len(),
        2,
        "function literal parameters wrong. want 2, got={}",
        fn_literal.parameters.len()
    );

    let x = &Box::new(fn_literal.parameters.get(0).unwrap() as &dyn Expression);
    let y = &Box::new(fn_literal.parameters.get(1).unwrap() as &dyn Expression);

    test_literal_expression(x, &"x");
    test_literal_expression(y, &"y");

    match get_of_type::<ExpressionStatement>(
        fn_literal
            .body
            .as_ref()
            .unwrap()
            .statements
            .get(0)
            .unwrap()
            .get_as_any(),
    ) {
        Some(body_stmt) => match &body_stmt.expression {
            Some(exp) => test_infix_expression(exp, &"x", "+", &"y"),
            _ => panic!("function body stmt is not ast.ExpressionStatement"),
        },
        _ => panic!("function body stmt is not ast.ExpressionStatement"),
    }
}

#[test]
fn test_function_parameter_parsing() {
    struct TestCase {
        input: String,
        expected_params: Vec<String>,
    }

    let tests = [
        TestCase {
            input: String::from("fn() {};"),
            expected_params: vec![],
        },
        TestCase {
            input: String::from("fn(x) {};"),
            expected_params: vec![String::from("x")],
        },
        TestCase {
            input: String::from("fn(x, y, z) {};"),
            expected_params: vec![String::from("x"), String::from("y"), String::from("z")],
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

        let stmt = get_of_type::<ExpressionStatement>(
            program
                .statements
                .leak()
                .get(0)
                .expect("expected statemnt[0] to have a value")
                .get_as_any(),
        )
        .expect("program.Statements[0] is ast.ExpressionStatement");

        let fn_literal = match &stmt.expression {
            Some(fn_literal) => match get_of_type::<FunctionLiteral>(fn_literal.get_as_any()) {
                Some(fn_literal) => fn_literal,
                _ => panic!("exp is not If expression"),
            },
            _ => panic!("exp is none"),
        };

        assert_eq!(
            fn_literal.parameters.len(),
            tt.expected_params.len(),
            "function literal parameters wrong. want {}, got={}",
            tt.expected_params.len(),
            fn_literal.parameters.len()
        );

        for (i, ident) in tt.expected_params.iter().enumerate() {
            let p = &Box::new(fn_literal.parameters.get(i).unwrap() as &dyn Expression);
            test_literal_expression(p, ident);
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = String::from("add(1, 2 * 3, 4 + 5)");

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

    let stmt = get_of_type::<ExpressionStatement>(
        program
            .statements
            .leak()
            .get(0)
            .expect("expected statemnt[0] to have a value")
            .get_as_any(),
    )
    .expect("program.Statements[0] is ast.ExpressionStatement");

    let exp = match &stmt.expression {
        Some(exp) => match get_of_type::<CallExpression>(exp.get_as_any()) {
            Some(exp) => exp,
            _ => panic!("exp is not If expression"),
        },
        _ => panic!("exp is none"),
    };

    test_identifier(&Box::new(exp.function.as_ref()), "add");

    assert_eq!(
        exp.arguments.len(),
        3,
        "wrong legnth of arg. go={}",
        exp.arguments.len()
    );
    test_literal_expression(&Box::new(exp.arguments.get(0).unwrap().as_ref()), &1);
    test_infix_expression(exp.arguments.get(1).as_ref().unwrap(), &2, "*", &3);
    test_infix_expression(exp.arguments.get(2).as_ref().unwrap(), &4, "+", &5);
}

#[test]
fn test_parsing_prefix_expression() {
    struct TestCase<'a> {
        input: String,
        operator: String,
        integer_value: &'a dyn Any,
    }

    let tests = [
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

        let stmt = get_of_type::<ExpressionStatement>(
            program
                .statements
                .get(0)
                .expect("expected statemnt[0] to have a value")
                .get_as_any(),
        )
        .expect("program.Statements[0] is ast.ExpressionStatement");

        let exp = match &stmt.expression {
            Some(exp) => match get_of_type::<PrefixExpression>(exp.get_as_any()) {
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

        test_literal_expression(
            &Box::new(exp.right.as_ref().unwrap().as_ref()),
            tt.integer_value,
        );
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

    let tests = [
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

        let stmt = get_of_type::<ExpressionStatement>(
            program
                .statements
                .leak()
                .get(0)
                .expect("expected statemnt[0] to have a value")
                .get_as_any(),
        )
        .expect("program.Statements[0] is ast.ExpressionStatement");

        match &stmt.expression {
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

    let tests = [
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
        TestCase {
            input: String::from("a + add(b * c) + d"),
            expected: String::from("((a + add((b * c))) + d)"),
        },
        TestCase {
            input: String::from("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))"),
            expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        },
        TestCase {
            input: String::from("add(a + b + c * d / f + g)"),
            expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
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
    exp: &'static Box<dyn Expression>,
    left: &dyn Any,
    operator: &str,
    right: &dyn Any,
) {
    let op_exp =
        get_of_type::<InfixExpression>(exp.get_as_any()).expect("exp is OperatorExpression");
    test_literal_expression(&Box::new(op_exp.left.as_ref()), left);
    assert_eq!(
        op_exp.operator, operator,
        "exp.Operator is not {:?}. got={:?}",
        operator, op_exp.operator
    );

    test_literal_expression(&Box::new(op_exp.right.as_ref().unwrap().as_ref()), right);
}

fn test_int_literal(il: &Box<&dyn Expression>, value: i64) {
    let integ = get_of_type::<IntegerLiteral>(il.get_as_any()).expect("il is not IntergerLiteral");
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

fn test_identifier(il: &Box<&dyn Expression>, value: &str) {
    let ident = get_of_type::<Identifier>(il.get_as_any()).expect("il is Identifier");
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

fn test_boolean_literal(il: &Box<&dyn Expression>, value: bool) {
    let bo = get_of_type::<BooleanLiteral>(il.get_as_any()).expect("il is Boolean");
    assert_eq!(bo.value, value, "bo.value not {}. got={}", bo.value, value);
    assert_eq!(
        bo.token_literal(),
        value.to_string(),
        "bo.value not {}. got={}",
        bo.value,
        value
    );
}

fn test_literal_expression(exp: &Box<&dyn Expression>, expected: &dyn Any) {
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
    let let_stmt = get_of_type::<LetStatement>(s.get_as_any()).unwrap();
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
