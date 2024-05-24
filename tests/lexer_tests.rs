use compiler::prelude::{Lexer, Token, TokenType::*};

#[test]
fn lexer_lexes_single_tokens() {
    let mut lexer = Lexer::new();
    lexer.lex("=+(){},;");

    let expeted_types = [
        Token {
            token_type: EQUAL_TOKEN,
            literal: String::from("="),
        },
        Token {
            token_type: PLUS_TOKEN,
            literal: String::from("+"),
        },
        Token {
            token_type: OPEN_PAREN_TOKEN,
            literal: String::from("("),
        },
        Token {
            token_type: CLOSE_PAREN_TOKEN,
            literal: String::from(")"),
        },
        Token {
            token_type: OPEN_BRACKET_TOKEN,
            literal: String::from("{"),
        },
        Token {
            token_type: CLOSE_BRACKET_TOKEN,
            literal: String::from("}"),
        },
        Token {
            token_type: COMMA_TOKEN,
            literal: String::from(","),
        },
        Token {
            token_type: SEMI_COLON_TOKEN,
            literal: String::from(";"),
        },
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}

#[test]
fn lexer_lexes_illegal_token() {
    let mut lexer = Lexer::new();
    lexer.lex("let a = 5?;");

    let expeted_types = [
        Token::new(LET_KEYWORD_TOKEN, "let".to_string()),
        Token::new(IDENTIFIER_TOKEN, "a".to_string()),
        Token::new(EQUAL_TOKEN, "=".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(ILLEGAL, "?".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}
#[test]
fn lexer_lexes_complex_tokens() {
    let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);";
    let mut lexer = Lexer::new();
    lexer.lex(input);

    let expeted_types = [
        // first assign
        Token::new(LET_KEYWORD_TOKEN, "let".to_string()),
        Token::new(IDENTIFIER_TOKEN, "five".to_string()),
        Token::new(EQUAL_TOKEN, "=".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
        // second assign
        Token::new(LET_KEYWORD_TOKEN, "let".to_string()),
        Token::new(IDENTIFIER_TOKEN, "ten".to_string()),
        Token::new(EQUAL_TOKEN, "=".to_string()),
        Token::new(INT_LITERAL_TOKEN, "10".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
        // function declaration
        Token::new(LET_KEYWORD_TOKEN, "let".to_string()),
        Token::new(IDENTIFIER_TOKEN, "add".to_string()),
        Token::new(EQUAL_TOKEN, "=".to_string()),
        Token::new(FUNC_KEYWORD_TOKEN, "fn".to_string()),
        Token::new(OPEN_PAREN_TOKEN, "(".to_string()),
        Token::new(IDENTIFIER_TOKEN, "x".to_string()),
        Token::new(COMMA_TOKEN, ",".to_string()),
        Token::new(IDENTIFIER_TOKEN, "y".to_string()),
        Token::new(CLOSE_PAREN_TOKEN, ")".to_string()),
        Token::new(OPEN_BRACKET_TOKEN, "{".to_string()),
        Token::new(IDENTIFIER_TOKEN, "x".to_string()),
        Token::new(PLUS_TOKEN, "+".to_string()),
        Token::new(IDENTIFIER_TOKEN, "y".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
        Token::new(CLOSE_BRACKET_TOKEN, "}".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
        // function call
        // let result = add(five, ten);
        Token::new(LET_KEYWORD_TOKEN, "let".to_string()),
        Token::new(IDENTIFIER_TOKEN, "result".to_string()),
        Token::new(EQUAL_TOKEN, "=".to_string()),
        Token::new(IDENTIFIER_TOKEN, "add".to_string()),
        Token::new(OPEN_PAREN_TOKEN, "(".to_string()),
        Token::new(IDENTIFIER_TOKEN, "five".to_string()),
        Token::new(COMMA_TOKEN, ",".to_string()),
        Token::new(IDENTIFIER_TOKEN, "ten".to_string()),
        Token::new(CLOSE_PAREN_TOKEN, ")".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}

#[test]
fn lexer_lexes_single_token_boolean_logic() {
    let input = "!-/*5;
                5 < 10 > 5;";
    let mut lexer = Lexer::new();
    lexer.lex(input);

    let expeted_types = [
        Token::new(BANG_TOKEN, "!".to_string()),
        Token::new(MINUS_TOKEN, "-".to_string()),
        Token::new(SLASH_TOKEN, "/".to_string()),
        Token::new(STAR_TOKEN, "*".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(LESS_THAN_TOKEN, "<".to_string()),
        Token::new(INT_LITERAL_TOKEN, "10".to_string()),
        Token::new(GREATER_THAN_TOKEN, ">".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}

#[test]
fn lexer_lexes_conditional_keywords() {
    let input = "
        if (5 < 6)
            return 5;
        else
            return 6;
                ";
    let mut lexer = Lexer::new();
    lexer.lex(input);

    let expeted_types = [
        Token::new(IF_KEYWORD_TOKEN, "if".to_string()),
        Token::new(OPEN_PAREN_TOKEN, "(".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(LESS_THAN_TOKEN, "<".to_string()),
        Token::new(INT_LITERAL_TOKEN, "6".to_string()),
        Token::new(CLOSE_PAREN_TOKEN, ")".to_string()),
        Token::new(RETURN_KEYWORD_TOKEN, "return".to_string()),
        Token::new(INT_LITERAL_TOKEN, "5".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),

        Token::new(ELSE_KEYWORD_TOKEN, "else".to_string()),
        Token::new(RETURN_KEYWORD_TOKEN, "return".to_string()),
        Token::new(INT_LITERAL_TOKEN, "6".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}

#[test]
fn lexr_lexes_two_token_booleans() {
    let input = "
        10 == 10;
        10 != 9;
                ";
    let mut lexer = Lexer::new();
    lexer.lex(input);

    let expeted_types = [
        Token::new(INT_LITERAL_TOKEN, "10".to_string()),
        Token::new(EQUAL_EQUAL_TOKEN, "==".to_string()),
        Token::new(INT_LITERAL_TOKEN, "10".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),

        Token::new(INT_LITERAL_TOKEN, "10".to_string()),
        Token::new(BANG_EQUAL_TOKEN, "!=".to_string()),
        Token::new(INT_LITERAL_TOKEN, "9".to_string()),
        Token::new(SEMI_COLON_TOKEN, ";".to_string()),
    ];

    expeted_types.iter().for_each(|expected| {
        let actual = lexer.next_token();
        assert_eq!(actual.token_type, expected.token_type);
        assert_eq!(actual.literal, expected.literal);
    })
}
