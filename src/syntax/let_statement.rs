use std::any::Any;

use super::{
    ast::SyntaxNode,
    syntax_nodes::{Expression, Statement, Identifier},
    syntax_tokens::{Token, TokenType::*},
};

#[derive(Debug)]
pub struct LetStatement {
    let_keyword: Token,
    identifier: Identifier,
    value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(identifier: Token, value: Box<dyn Expression>) -> Self {
        LetStatement {
            let_keyword: Token::new(LET_KEYWORD_TOKEN, String::from("let")),
            identifier: Identifier::new(identifier),
            value,
        }
    }

    pub fn identifier(&self) -> &str {
        self.identifier.name()
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl SyntaxNode for LetStatement {
    fn token_literal(&self) -> String {
        self.let_keyword.literal.clone()
    }
}
