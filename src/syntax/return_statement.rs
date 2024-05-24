use super::{
    ast::SyntaxNode,
    syntax_nodes::{Expression, Statement},
    syntax_tokens::Token,
    syntax_tokens::TokenType::*,
};
use std::any::Any;

#[derive(Debug)]
pub struct ReturnStatement {
    return_keyword: Token,
    return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(return_value: Box<dyn Expression>) -> Self {
        ReturnStatement {
            return_keyword: Token::new(RETURN_KEYWORD_TOKEN, String::from("return")),
            return_value,
        }
    }
}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SyntaxNode for ReturnStatement {
    fn token_literal(&self) -> String {
        self.return_keyword.literal.clone()
    }
}
