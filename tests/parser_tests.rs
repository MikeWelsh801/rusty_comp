use compiler::prelude::{LetStatement, Lexer, Parser, ReturnStatement, Statement, SyntaxNode};
use core::panic;

#[cfg(test)]
mod let_statement_tests {
    use super::*;

    #[test]
    fn parser_parses_let_statements() {
        let input = "
        let x = 5;
        let y = 10; 
        let foobar = 1223;
    ";
        let mut lexer = Lexer::new();
        lexer.lex(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        let expected_errors = vec![];

        check_parser_errors(&parser, &expected_errors);

        assert_eq!(3, program.statements.len());

        // test that each statement is a let statement and has the correct identifier
        let expected = ["x", "y", "foobar"];
        program
            .statements
            .iter()
            .enumerate()
            .for_each(|(i, s)| test_let_statement(s, expected[i]));
    }

    #[test]
    fn parser_parses_let_statements_with_errors() {
        let input = "
        let = 5;
        let y 10; 
        let 1223;
        let hello = 55; 
    ";
        let mut lexer = Lexer::new();
        lexer.lex(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        let expected_errors = vec![
            "Expected next token to be IDENTIFIER_TOKEN, but got EQUAL_TOKEN",
            "Expected next token to be EQUAL_TOKEN, but got INT_LITERAL_TOKEN",
            "Expected next token to be IDENTIFIER_TOKEN, but got INT_LITERAL_TOKEN",
        ];

        check_parser_errors(&parser, &expected_errors);

        // only hello statement in the program since the rest have errors
        assert_eq!(1, program.statements.len());

        // test that each statement is a let statement and has the correct identifier
        let expected = ["hello"];
        program
            .statements
            .iter()
            .enumerate()
            .for_each(|(i, s)| test_let_statement(s, expected[i]));
    }
}

#[cfg(test)]
mod return_statement_tests {
    use super::*;

    #[test]
    fn parser_parses_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let mut lexer = Lexer::new();
        lexer.lex(input);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        // no errors
        check_parser_errors(&parser, &vec![]);

        assert_eq!(3, program.statements.len());
        program
            .statements
            .iter()
            .for_each(|s| test_return_statement(s));
    }
}

fn check_parser_errors(parser: &Parser, expected_errors: &[&str]) {
    let errors = parser.errors();
    if errors.is_empty() {
        return;
    }

    errors.iter().enumerate().for_each(|(i, error)| {
        println!("Parser error: {}", error);
        assert_eq!(error, expected_errors[i]);
    });
}

fn test_return_statement(statement: &Box<dyn Statement>) {
    let s = match statement.as_any().downcast_ref::<ReturnStatement>() {
        Some(return_statement) => return_statement,
        None => panic!("Not a let statement! Statement: {:#?}", statement),
    };
    assert_eq!(s.token_literal(), String::from("return"));
}

fn test_let_statement(statement: &Box<dyn Statement>, expected_identifier: &str) {
    let s = match statement.as_any().downcast_ref::<LetStatement>() {
        Some(let_statement) => let_statement,
        None => panic!("Not a let statement! Statement: {:#?}", statement),
    };
    assert_eq!(s.identifier(), expected_identifier);
}
