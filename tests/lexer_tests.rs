use compiler::prelude::{Lexer, TokenType::*};

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new();
    let input = "=+(){},;";

    let expeted_types = [
        OPEN_PAREN_TOKEN,
        CLOSE_PAREN_TOKEN,
        OPEN_BRACKET_TOKEN,
        CLOSE_BRACKET_TOKEN,
        COMMA_TOKEN,
        SEMI_COLON_TOKEN,
    ];

    lexer.lex(input);
    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, *expected)
    })
}
