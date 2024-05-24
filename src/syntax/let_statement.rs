use super::{
    ast::SyntaxNode,
    syntax_nodes::{Expression, Statement},
    syntax_tokens::{Token, TokenType::*},
};

#[derive(Debug)]
pub(crate) struct LetStatement {
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
}

impl Statement for LetStatement {}
impl SyntaxNode for LetStatement {
    fn token_literal(&self) -> String {
        self.let_keyword.literal.clone()
    }
}

#[derive(Debug)]
pub(crate) struct Identifier {
    token: Token,
    value: String,
}

impl Identifier {
    fn new(token: Token) -> Self {
        let value = token.literal.clone();
        Identifier { token, value }
    }
}

impl Expression for Identifier {}
impl SyntaxNode for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
