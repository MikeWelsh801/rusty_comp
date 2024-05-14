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
    SLASH_TOKEN,
    STAR_TOKEN,

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
    IDENTIFIER_TOKEN,
    INT_LITERAL_TOKEN,
    FUNC_KEYWORD_TOKEN,
}

pub(crate) fn identifier_lookup(identifier: &str) -> TokenType {
    match identifier {
        "fn" => FUNC_KEYWORD_TOKEN,
        "let" => LET_KEYWORD_TOKEN,
        _ => IDENTIFIER_TOKEN,
    }
}
