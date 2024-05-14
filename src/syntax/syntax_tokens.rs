use crate::prelude::TokenType::*;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    // end or throw error
    EOF,
    ILLEGAL,

    // binary/unary operators
    PLUS_TOKEN,
    MINUS_TOKEN,
    BANG_TOKEN,
    SLASH_TOKEN,
    STAR_TOKEN,

    // logical operators
    LESS_THAN_TOKEN,
    GREATER_THAN_TOKEN,
    EQUAL_EQUAL_TOKEN,
    BANG_EQUAL_TOKEN,

    // parens and brackets
    OPEN_PAREN_TOKEN,
    CLOSE_PAREN_TOKEN,
    OPEN_BRACKET_TOKEN,
    CLOSE_BRACKET_TOKEN,

    // punctuation
    COMMA_TOKEN,
    SEMI_COLON_TOKEN,

    // assign
    EQUAL_TOKEN,

    // literals and keywords
    LET_KEYWORD_TOKEN,
    FUNC_KEYWORD_TOKEN,
    RETURN_KEYWORD_TOKEN,
    IF_KEYWORD_TOKEN,
    ELSE_KEYWORD_TOKEN,
    IDENTIFIER_TOKEN,

    INT_LITERAL_TOKEN,
}

pub(crate) fn identifier_lookup(identifier: &str) -> TokenType {
    match identifier {
        "fn" => FUNC_KEYWORD_TOKEN,
        "let" => LET_KEYWORD_TOKEN,
        "return" => RETURN_KEYWORD_TOKEN,
        "if" => IF_KEYWORD_TOKEN,
        "else" => ELSE_KEYWORD_TOKEN,
        _ => IDENTIFIER_TOKEN,
    }
}
