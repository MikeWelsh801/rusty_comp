use crate::prelude::Token;
use crate::prelude::TokenType::*;

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { tokens: vec![] }
    }

    pub fn print_tokens(&self) {
        println!("{:#?}", self.tokens);
    }

    pub fn lex(&mut self, input: &str) {
        input.split_whitespace().for_each(|block| {})
    }

    pub fn next_token(&mut self) -> Token {
        Token { token_type: OPEN_PAREN_TOKEN, literal: String::from("(") }
    }
}
