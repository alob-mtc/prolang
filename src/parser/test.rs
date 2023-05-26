mod test {

    use crate::{
        lexer::lexer::Lexer,
        parser::ast::{Expression, Node, Statement},
        parser::parser::Parser,
    };

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program().expect("parse_program() return none");
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
                expected_identifier: "x".to_string(),
            },
            TestCase {
                expected_identifier: "y".to_string(),
            },
            TestCase {
                expected_identifier: "foobar".to_string(),
            },
        ];

        let mut i = 0;
        for tt in tests {
            let stmt = program.statements.get(i).unwrap();
            let_statemnt(stmt, tt.expected_identifier);
            i += 1;
        }
    }

    #[test]
    fn test_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993322;
        "
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program().expect("parse_program() return none");
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
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return none");

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
            .expect("program.Statements[0] is not ast.ExpressionStatement");

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
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().expect("parse_program() return none");

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
            .expect("program.Statements[0] is not ast.ExpressionStatement");

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
        struct TestCase {
            input: String,
            operator: String,
            integer_value: i64,
        }

        let tests = vec![
            TestCase {
                input: "!5".to_string(),
                operator: "!".to_string(),
                integer_value: 5,
            },
            TestCase {
                input: "-15".to_string(),
                operator: "-".to_string(),
                integer_value: 15,
            },
        ];

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);
            let program = p.parse_program().expect("parse_program() return none");

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
                .expect("program.Statements[0] is not ast.ExpressionStatement");

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

            test_int_literal(exp.right.as_ref().unwrap(), tt.integer_value)
        }
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

    fn let_statemnt(s: &Box<dyn Statement>, name: String) {
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
}
