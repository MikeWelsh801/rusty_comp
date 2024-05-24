use compiler::prelude::{Lexer, Token, TokenType::*, Parser};

#[test]
fn parser_parses_let_statements() {
    let input = "
        let x = 5;
        ";
    let mut lexer = Lexer::new();
    lexer.lex(input);
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
}
